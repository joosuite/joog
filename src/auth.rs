use crate::{
    http::{Http, check_status, limited},
    store::Store,
};
use anyhow::{Context, Result, bail};
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use subtle::ConstantTimeEq;
use url::Url;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

const AUTHORIZE: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN: &str = "https://oauth2.googleapis.com/token";

pub mod transfer;

#[derive(Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Tokens {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
}

#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: String,
    expires_in: u64,
    token_type: String,
}

fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn masked_prompt(prompt: &str) -> std::io::Result<String> {
    rpassword::prompt_password_with_config(
        prompt,
        rpassword::ConfigBuilder::new()
            .password_feedback_mask('*')
            .build(),
    )
}

pub fn enroll(store: &Store, stdin: bool) -> Result<()> {
    store.initialize()?;
    let creds: Credentials = if stdin {
        let raw = Zeroizing::new(limited(std::io::stdin().lock(), 1024 * 1024)?);
        serde_json::from_slice(&raw)
            .context("expected client_id and client_secret JSON on stdin")?
    } else {
        let mut creds = Credentials {
            client_id: String::new(),
            client_secret: String::new(),
        };
        creds.client_id = masked_prompt("Google client ID: ")?;
        creds.client_secret = masked_prompt("Google client secret: ")?;
        creds
    };
    anyhow::ensure!(
        !creds.client_id.trim().is_empty() && !creds.client_secret.trim().is_empty(),
        "client_id and client_secret are required"
    );
    store.set_secret("oauth-client", &creds)
}

pub fn test(store: &Store, http: &Http) -> Result<()> {
    let creds: Credentials = store
        .secret("oauth-client")
        .context("enroll your Google client_id and client_secret with joog auth enroll")?;
    anyhow::ensure!(
        !creds.client_id.trim().is_empty() && !creds.client_secret.trim().is_empty(),
        "enrolled client_id and client_secret are required; run joog auth enroll"
    );
    let probe = Zeroizing::new(format!("joog-connectivity-test-{}", random_string()?));
    let request = client_test_request(http, &creds, &probe)?;
    let response = http
        .client
        .execute(request)
        .map_err(|e| e.without_url())
        .context("cannot reach Google's OAuth token endpoint")?;
    let status = response.status();
    let bytes = Zeroizing::new(limited(response, 1024 * 1024)?);
    client_test_response(status, &bytes)
}

fn client_test_request(
    http: &Http,
    creds: &Credentials,
    probe: &str,
) -> Result<reqwest::blocking::Request> {
    Ok(http
        .client
        .post(TOKEN)
        .form(&[
            ("client_id", creds.client_id.as_str()),
            ("client_secret", creds.client_secret.as_str()),
            ("grant_type", "refresh_token"),
            ("refresh_token", probe),
        ])
        .build()?)
}

fn client_test_response(status: reqwest::StatusCode, bytes: &[u8]) -> Result<()> {
    #[derive(Deserialize)]
    struct OAuthError {
        error: String,
    }
    if !matches!(status.as_u16(), 400 | 401) {
        check_status(status)?;
        bail!("Google returned an unexpected response to the connectivity test");
    }
    let response: OAuthError = serde_json::from_slice(bytes)
        .context("Google returned an invalid OAuth connectivity-test response")?;
    match (status.as_u16(), response.error.as_str()) {
        (400, "invalid_grant") => Ok(()),
        (_, "invalid_client") => {
            bail!(
                "Google rejected the client ID or secret. Run 'joog auth enroll' to replace them."
            )
        }
        (_, "deleted_client") => bail!(
            "The Google OAuth client has been deleted. Enroll an active Desktop app client with 'joog auth enroll'."
        ),
        (_, "unauthorized_client") => bail!(
            "This Google OAuth client cannot use the required sign-in flow. Run 'joog auth enroll' with a Desktop app client."
        ),
        _ => bail!(
            "Google is reachable but did not accept the OAuth connectivity test; check the enrolled client configuration"
        ),
    }
}

pub fn login(
    store: &Store,
    http: &Http,
    scopes: Vec<String>,
    account: Option<&str>,
) -> Result<String> {
    let creds: Credentials = store
        .secret("oauth-client")
        .context("enroll your Google client_id and client_secret with joog auth enroll")?;
    anyhow::ensure!(
        !scopes.is_empty(),
        "specify at least one service after login"
    );
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))?;
    listener.set_nonblocking(true)?;
    let redirect = format!(
        "http://127.0.0.1:{}/oauth/callback",
        listener.local_addr()?.port()
    );
    let verifier = Zeroizing::new(random_string()?);
    let state = Zeroizing::new(random_string()?);
    let url = authorization_url(
        &creds.client_id,
        &redirect,
        scopes,
        account,
        &verifier,
        &state,
    )?;
    open_browser(&url)?;
    writeln!(
        std::io::stderr().lock(),
        "Complete sign-in in your browser. Waiting for authorization for up to 3 minutes."
    )?;
    let code = Zeroizing::new(wait_callback(&listener, &state, Duration::from_secs(180))?);
    let response = token_request(
        http,
        &[
            ("client_id", &creds.client_id),
            ("client_secret", &creds.client_secret),
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", &redirect),
            ("code_verifier", &verifier),
        ],
    )?;
    anyhow::ensure!(
        !response.refresh_token.is_empty(),
        "Google did not return a refresh token; retry consent"
    );
    let profile = http.json(
        &reqwest::Method::GET,
        &Url::parse("https://openidconnect.googleapis.com/v1/userinfo")?,
        None,
        Some(&response.access_token),
        1024 * 1024,
    )?;
    anyhow::ensure!(
        profile["email_verified"] == true,
        "Google account email is not verified"
    );
    let email = profile["email"]
        .as_str()
        .context("Google did not return an email address")?;
    if let Some(expected) = account {
        anyhow::ensure!(
            email.eq_ignore_ascii_case(expected),
            "the signed-in account differs from the account before login"
        );
    }
    let email = email.to_ascii_lowercase();
    let tokens = Tokens {
        access_token: response.access_token.clone(),
        refresh_token: response.refresh_token.clone(),
        expires_at: now()?.saturating_add(response.expires_in),
    };
    store.set_secret(&format!("account/{email}"), &tokens)?;
    let mut config = store.config()?;
    if !config.accounts.contains(&email) {
        config.accounts.push(email.clone());
    }
    config.account = Some(email.clone());
    store.save_config(&config)?;
    Ok(email)
}

fn authorization_url(
    client_id: &str,
    redirect: &str,
    scopes: Vec<String>,
    account: Option<&str>,
    verifier: &str,
    state: &str,
) -> Result<Url> {
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let mut url = Url::parse(AUTHORIZE)?;
    let mut scopes = scopes;
    scopes.push("openid".into());
    scopes.push("email".into());
    scopes.sort();
    scopes.dedup();
    url.query_pairs_mut().extend_pairs([
        ("client_id", client_id),
        ("redirect_uri", redirect),
        ("response_type", "code"),
        ("scope", &scopes.join(" ")),
        ("access_type", "offline"),
        ("prompt", "consent"),
        ("code_challenge", &challenge),
        ("code_challenge_method", "S256"),
        ("state", state),
    ]);
    if let Some(account) = account {
        url.query_pairs_mut().append_pair("login_hint", account);
    }
    Ok(url)
}

fn browser_command(url: &Url) -> Command {
    #[cfg(target_os = "macos")]
    let mut command = Command::new("/usr/bin/open");
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("rundll32.exe");
        command.arg("url.dll,FileProtocolHandler");
        command
    };
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let mut command = Command::new("xdg-open");
    command
        .arg(url.as_str())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn open_browser(url: &Url) -> Result<()> {
    let status = browser_command(url).status().context(
        "could not open the default browser; check your browser configuration and retry login",
    )?;
    anyhow::ensure!(
        status.success(),
        "could not open the default browser; check your browser configuration and retry login"
    );
    Ok(())
}

pub fn access_token(store: &Store, http: &Http, account: &str) -> Result<Zeroizing<String>> {
    let account = account.to_ascii_lowercase();
    let mut tokens: Tokens = store.secret(&format!("account/{account}"))?;
    if tokens.expires_at > now()?.saturating_add(60) {
        return Ok(Zeroizing::new(tokens.access_token.clone()));
    }
    let creds: Credentials = store.secret("oauth-client")?;
    let response = token_request(
        http,
        &[
            ("client_id", &creds.client_id),
            ("client_secret", &creds.client_secret),
            ("grant_type", "refresh_token"),
            ("refresh_token", &tokens.refresh_token),
        ],
    )?;
    tokens.access_token = response.access_token.clone();
    if !response.refresh_token.is_empty() {
        tokens.refresh_token = response.refresh_token.clone();
    }
    tokens.expires_at = now()?.saturating_add(response.expires_in);
    store.set_secret(&format!("account/{account}"), &tokens)?;
    Ok(Zeroizing::new(tokens.access_token.clone()))
}

pub fn logout(store: &Store, account: Option<&str>) -> Result<String> {
    let mut config = store.config()?;
    let email = account
        .or(config.account.as_deref())
        .context("choose an account with joog auth <account> logout")?
        .to_ascii_lowercase();
    store.delete_secret(&format!("account/{email}"))?;
    config.accounts.retain(|a| a != &email);
    if config.account.as_deref() == Some(&email) {
        config.account = config.accounts.first().cloned();
    }
    store.save_config(&config)?;
    Ok(email)
}

fn token_request(http: &Http, fields: &[(&str, &str)]) -> Result<TokenResponse> {
    let response = http
        .client
        .post(TOKEN)
        .form(fields)
        .send()
        .map_err(|e| e.without_url())?;
    check_status(response.status())?;
    let bytes = Zeroizing::new(limited(response, 1024 * 1024)?);
    let tokens: TokenResponse =
        serde_json::from_slice(&bytes).context("invalid OAuth token response")?;
    anyhow::ensure!(
        tokens.token_type.eq_ignore_ascii_case("bearer") && !tokens.access_token.is_empty(),
        "invalid OAuth token type"
    );
    Ok(tokens)
}

fn random_string() -> Result<String> {
    let mut bytes = Zeroizing::new([0_u8; 32]);
    getrandom::fill(bytes.as_mut()).context("operating system randomness unavailable")?;
    Ok(URL_SAFE_NO_PAD.encode(bytes.as_ref()))
}

#[derive(Debug)]
enum CallbackError {
    UnexpectedPath,
    InvalidState,
    AuthorizationDenied,
    MissingCode,
}

impl std::fmt::Display for CallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::UnexpectedPath => "unexpected callback path",
            Self::InvalidState => "this callback does not match the current sign-in attempt",
            Self::AuthorizationDenied => "Google did not authorize sign-in; run 'joog auth ACCOUNT login SERVICE_LIST' to try again",
            Self::MissingCode => "Google did not return a single authorization code",
        })
    }
}

impl std::error::Error for CallbackError {}

fn callback_page(title: &str, message: &str, success: bool) -> String {
    include_str!("auth/callback.html")
        .replace(
            "{{brandLogo}}",
            &STANDARD.encode(include_bytes!("../assets/brand/logo.png")),
        )
        .replace(
            "{{brandIcon}}",
            &STANDARD.encode(include_bytes!("../assets/brand/icon.png")),
        )
        .replace("{{title}}", title)
        .replace("{{message}}", message)
        .replace("{{state}}", if success { "success" } else { "error" })
        .replace(
            "{{icon}}",
            if success {
                "M17 28l7 7 15-15"
            } else {
                "M28 16v15M28 39v1"
            },
        )
        .replace(
            "{{next}}",
            if success {
                "You can close this tab and return to your terminal."
            } else {
                "Return to your terminal to continue."
            },
        )
        .replace(
            "{{detail}}",
            if success {
                "Joog will confirm in your terminal when sign-in is complete."
            } else {
                "You can close this tab."
            },
        )
}

fn wait_callback(listener: &TcpListener, state: &str, timeout: Duration) -> Result<String> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match listener.accept() {
            Ok((mut stream, peer)) => {
                if !peer.ip().is_loopback() {
                    continue;
                }
                // macOS inherits O_NONBLOCK from the listener. Browser headers can
                // arrive in several reads; wait for them within the socket timeout.
                stream.set_nonblocking(false)?;
                stream.set_read_timeout(Some(Duration::from_secs(2)))?;
                stream.set_write_timeout(Some(Duration::from_secs(2)))?;
                let result = read_callback(&mut stream, state);
                let rejection = result
                    .as_ref()
                    .err()
                    .and_then(|e| e.downcast_ref::<CallbackError>());
                let (status, title, message) = match (&result, rejection) {
                    (Ok(_), _) => (
                        "200 OK",
                        "You're done here.",
                        "Google has sent your approval to Joog.",
                    ),
                    (_, Some(CallbackError::UnexpectedPath)) => (
                        "404 Not Found",
                        "Nothing to sign in to here.",
                        "This address is not a sign-in callback.",
                    ),
                    (_, Some(CallbackError::InvalidState)) => (
                        "400 Bad Request",
                        "Let's use the right tab.",
                        "This callback does not match the current sign-in attempt. Complete sign-in in the browser opened by the current command.",
                    ),
                    (_, Some(CallbackError::AuthorizationDenied)) => (
                        "403 Forbidden",
                        "Sign-in wasn't approved.",
                        "Google did not authorize sign-in. Return to the terminal to try again.",
                    ),
                    (_, Some(CallbackError::MissingCode)) => (
                        "400 Bad Request",
                        "Sign-in needs another try.",
                        "Google did not return an authorization code. Return to the terminal to try again.",
                    ),
                    _ => (
                        "400 Bad Request",
                        "We couldn't finish this step.",
                        "The browser request could not be read. Return to the terminal for details.",
                    ),
                };
                let page = callback_page(title, message, result.is_ok());
                let _ = write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nCache-Control: no-store\r\nReferrer-Policy: no-referrer\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: default-src 'none'; img-src data:; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{page}",
                    page.len()
                );
                match result {
                    Ok(code) => return Ok(code),
                    Err(error) => match error.downcast_ref::<CallbackError>() {
                        Some(CallbackError::UnexpectedPath) => {}
                        Some(CallbackError::AuthorizationDenied) => return Err(error),
                        _ => writeln!(
                            std::io::stderr().lock(),
                            "Sign-in callback rejected: {error}. Still waiting for the current sign-in."
                        )?,
                    },
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(50))
            }
            Err(e) => return Err(e.into()),
        }
    }
    bail!("browser authorization timed out; run joog auth <account> login <service-list> again")
}

fn read_callback(stream: &mut TcpStream, state: &str) -> Result<String> {
    let mut bytes = Zeroizing::new(Vec::new());
    let start = Instant::now();
    while bytes.len() < 8192 && start.elapsed() < Duration::from_secs(2) {
        let mut buf = [0_u8; 512];
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        bytes.extend_from_slice(&buf[..n]);
        if bytes.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    anyhow::ensure!(
        bytes.len() < 8192 && bytes.windows(4).any(|w| w == b"\r\n\r\n"),
        "invalid callback headers"
    );
    let request = std::str::from_utf8(&bytes)?;
    let mut words = request
        .lines()
        .next()
        .unwrap_or_default()
        .split_whitespace();
    anyhow::ensure!(words.next() == Some("GET"), "callback requires GET");
    callback_code(words.next().context("missing callback target")?, state)
}

fn callback_code(target: &str, state: &str) -> Result<String> {
    anyhow::ensure!(
        target.starts_with("/oauth/callback?"),
        CallbackError::UnexpectedPath
    );
    let url = Url::parse(&format!("http://127.0.0.1{target}"))?;
    anyhow::ensure!(
        url.path() == "/oauth/callback" && url.fragment().is_none(),
        CallbackError::UnexpectedPath
    );
    let pairs = url.query_pairs().collect::<Vec<_>>();
    let states = pairs
        .iter()
        .filter(|(k, _)| k == "state")
        .collect::<Vec<_>>();
    anyhow::ensure!(
        states.len() == 1 && bool::from(states[0].1.as_bytes().ct_eq(state.as_bytes())),
        CallbackError::InvalidState
    );
    anyhow::ensure!(
        !pairs.iter().any(|(k, _)| k == "error"),
        CallbackError::AuthorizationDenied
    );
    let codes = pairs
        .iter()
        .filter(|(k, _)| k == "code")
        .collect::<Vec<_>>();
    anyhow::ensure!(
        codes.len() == 1 && !codes[0].1.is_empty(),
        CallbackError::MissingCode
    );
    Ok(codes[0].1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connectivity_probe_posts_client_credentials_only_to_google() {
        let credentials = Credentials {
            client_id: "dummy-client-id".into(),
            client_secret: "dummy-client-secret".into(),
        };
        let request =
            client_test_request(&Http::new(2).unwrap(), &credentials, "dummy-probe").unwrap();
        assert_eq!(request.method(), reqwest::Method::POST);
        assert_eq!(request.url().as_str(), TOKEN);
        assert!(request.url().query().is_none());
        assert!(
            !request
                .headers()
                .contains_key(reqwest::header::AUTHORIZATION)
        );
        let fields = url::form_urlencoded::parse(request.body().unwrap().as_bytes().unwrap())
            .collect::<std::collections::HashMap<_, _>>();
        assert_eq!(fields.len(), 4);
        assert_eq!(fields["client_id"], "dummy-client-id");
        assert_eq!(fields["client_secret"], "dummy-client-secret");
        assert_eq!(fields["grant_type"], "refresh_token");
        assert_eq!(fields["refresh_token"], "dummy-probe");
    }

    #[test]
    fn connectivity_probe_accepts_only_the_expected_google_response() {
        use reqwest::StatusCode;
        assert!(
            client_test_response(StatusCode::BAD_REQUEST, br#"{"error":"invalid_grant"}"#).is_ok()
        );
        for (status, body) in [
            (StatusCode::UNAUTHORIZED, r#"{"error":"invalid_client"}"#),
            (StatusCode::BAD_REQUEST, r#"{"error":"deleted_client"}"#),
            (
                StatusCode::BAD_REQUEST,
                r#"{"error":"unauthorized_client"}"#,
            ),
            (StatusCode::BAD_REQUEST, r#"{"error":"invalid_request"}"#),
            (StatusCode::BAD_REQUEST, "not JSON"),
            (StatusCode::BAD_REQUEST, "{}"),
            (StatusCode::UNAUTHORIZED, r#"{"error":"invalid_grant"}"#),
            (StatusCode::OK, r#"{"access_token":"must-not-be-used"}"#),
            (StatusCode::FOUND, r#"{"error":"invalid_grant"}"#),
            (
                StatusCode::SERVICE_UNAVAILABLE,
                r#"{"error":"invalid_grant"}"#,
            ),
        ] {
            assert!(
                client_test_response(status, body.as_bytes()).is_err(),
                "{status}: {body}"
            );
        }
    }

    #[test]
    fn connectivity_probe_errors_never_echo_remote_fields_or_secrets() {
        for error in [
            "invalid_client",
            "deleted_client",
            "unauthorized_client",
            "reflected-secret",
        ] {
            let body = serde_json::json!({"error":error,"error_description":"reflected-secret","client_secret":"reflected-secret"});
            let result = client_test_response(
                reqwest::StatusCode::BAD_REQUEST,
                &serde_json::to_vec(&body).unwrap(),
            )
            .unwrap_err();
            assert!(!format!("{result:#}").contains("reflected-secret"));
            if error != "reflected-secret" {
                assert!(result.to_string().contains("joog auth enroll"));
            }
        }
    }

    #[test]
    fn browser_authorization_uses_pkce_s256_and_account_hint() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let redirect = "http://127.0.0.1:49152/oauth/callback";
        let url = authorization_url(
            "client-id",
            redirect,
            vec!["gmail-scope".into(), "gmail-scope".into()],
            Some("you@example.com"),
            verifier,
            "random-state",
        )
        .unwrap();
        assert_eq!(
            url.origin().ascii_serialization(),
            "https://accounts.google.com"
        );
        assert_eq!(url.path(), "/o/oauth2/v2/auth");
        let pairs = url
            .query_pairs()
            .collect::<std::collections::HashMap<_, _>>();
        assert_eq!(pairs["code_challenge_method"], "S256");
        assert_eq!(
            pairs["code_challenge"],
            "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        );
        assert_eq!(pairs["redirect_uri"], redirect);
        assert_eq!(pairs["response_type"], "code");
        assert_eq!(pairs["state"], "random-state");
        assert_eq!(pairs["login_hint"], "you@example.com");
        assert_eq!(pairs["access_type"], "offline");
        assert_eq!(pairs["scope"], "email gmail-scope openid");
        assert!(!pairs.contains_key("code_verifier"));
        assert!(!pairs.contains_key("client_secret"));
    }

    #[test]
    fn browser_launcher_passes_url_as_one_argument_without_shell() {
        let url =
            Url::parse("https://accounts.google.com/o/oauth2/v2/auth?state=x&scope=email%20openid")
                .unwrap();
        let command = browser_command(&url);
        let args = command.get_args().collect::<Vec<_>>();
        #[cfg(target_os = "macos")]
        assert_eq!(command.get_program(), "/usr/bin/open");
        #[cfg(target_os = "windows")]
        {
            assert_eq!(command.get_program(), "rundll32.exe");
            assert_eq!(args[0], "url.dll,FileProtocolHandler");
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        assert_eq!(command.get_program(), "xdg-open");
        assert_eq!(args.len(), if cfg!(target_os = "windows") { 2 } else { 1 });
        assert_eq!(args.last().unwrap(), &url.as_str());
    }

    #[test]
    fn callback_requires_exact_state_and_single_code() {
        assert_eq!(
            callback_code("/oauth/callback?state=abc&code=xyz", "abc").unwrap(),
            "xyz"
        );
        for target in [
            "/oauth/callback?state=bad&code=xyz",
            "/oauth/callback?state=abc&code=x&code=y",
            "/oauth/callback?state=abc&state=abc&code=x",
            "/oauth/callback?state=abc&error=denied",
            "//evil.test/?state=abc&code=x",
            "/oauth/callback?state=abc&code=",
        ] {
            assert!(callback_code(target, "abc").is_err(), "{target}");
        }
    }

    #[test]
    fn callback_accepts_a_browser_request_delivered_in_fragments() {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let browser = std::thread::spawn(move || {
            let mut stream = TcpStream::connect(address).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            stream.write_all(b"GET /oauth/callback?state=expected-state&iss=https%3A%2F%2Faccounts.google.com&code=dummy-code&scope=email%20openid&authuser=4&prompt=consent HTTP/1.1\r\nHost: 127.0.0.1\r\n").unwrap();
            std::thread::sleep(Duration::from_millis(100));
            stream.write_all(b"Connection: close\r\n\r\n").unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            response
        });
        let result = wait_callback(&listener, "expected-state", Duration::from_secs(2));
        let response = browser.join().unwrap();
        assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
        assert!(response.contains("Content-Type: text/html; charset=utf-8"));
        assert!(response.contains("You're done here."));
        assert!(response.contains("You can close this tab and return to your terminal."));
        assert!(response.contains("Joog will confirm in your terminal when sign-in is complete."));
        assert!(!response.contains("dummy-code"));
        assert!(!response.contains("expected-state"));
        assert_eq!(result.unwrap(), "dummy-code");
    }

    fn browser_callback(address: std::net::SocketAddr, target: &str) -> String {
        let mut stream = TcpStream::connect(address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        write!(
            stream,
            "GET {target} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }

    #[test]
    fn callback_keeps_waiting_after_unrelated_or_wrong_state_requests() {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let browser = std::thread::spawn(move || {
            let unrelated = browser_callback(address, "/favicon.ico");
            assert!(unrelated.starts_with("HTTP/1.1 404 Not Found"));
            let wrong_state = browser_callback(
                address,
                "/oauth/callback?state=private-state&code=private-code",
            );
            assert!(wrong_state.starts_with("HTTP/1.1 400 Bad Request"));
            assert!(wrong_state.contains("does not match the current sign-in attempt"));
            assert!(!wrong_state.contains("private-state"));
            assert!(!wrong_state.contains("private-code"));
            let valid = browser_callback(
                address,
                "/oauth/callback?state=current-state&code=4%2Fdummy-code&iss=https%3A%2F%2Faccounts.google.com&scope=email%20openid",
            );
            assert!(valid.starts_with("HTTP/1.1 200 OK"));
        });
        let code = wait_callback(&listener, "current-state", Duration::from_secs(3)).unwrap();
        browser.join().unwrap();
        assert_eq!(code, "4/dummy-code");
    }

    #[test]
    fn callback_reports_google_denial_instead_of_waiting_until_timeout() {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let browser = std::thread::spawn(move || {
            browser_callback(
                address,
                "/oauth/callback?state=current-state&error=access_denied&error_description=private-error",
            )
        });
        let error = wait_callback(&listener, "current-state", Duration::from_secs(3)).unwrap_err();
        let response = browser.join().unwrap();
        assert!(matches!(
            error.downcast_ref::<CallbackError>(),
            Some(CallbackError::AuthorizationDenied)
        ));
        assert!(response.starts_with("HTTP/1.1 403 Forbidden"));
        assert!(response.contains("Google did not authorize sign-in"));
        assert!(!response.contains("private-error"));
        assert!(!error.to_string().contains("private-error"));
    }

    #[test]
    fn verifier_has_rfc7636_length_and_entropy() {
        let first = random_string().unwrap();
        assert_eq!(first.len(), 43);
        assert_ne!(first, random_string().unwrap());
    }
}
