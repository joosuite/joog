use anyhow::{Context, Result, bail};
use reqwest::{Method, StatusCode, blocking::Client, header};
use serde_json::Value;
use std::{io::Read, time::Duration};
use url::Url;

pub const JSON_LIMIT: u64 = 64 * 1024 * 1024;
pub const DISCOVERY_LIMIT: u64 = 16 * 1024 * 1024;

pub fn google_url(url: &Url) -> Result<()> {
    let host = url.host_str().unwrap_or_default();
    anyhow::ensure!(
        url.scheme() == "https"
            && url.username().is_empty()
            && url.password().is_none()
            && url.fragment().is_none()
            && url.port_or_known_default() == Some(443)
            && (host == "googleapis.com" || host.ends_with(".googleapis.com")),
        "refusing an API URL outside https://*.googleapis.com"
    );
    Ok(())
}

pub fn limited(mut reader: impl Read, limit: u64) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.by_ref().take(limit + 1).read_to_end(&mut bytes)?;
    anyhow::ensure!(
        bytes.len() as u64 <= limit,
        "response exceeds {limit} byte limit"
    );
    Ok(bytes)
}

pub struct Http {
    pub client: Client,
}

impl Http {
    pub fn new(timeout: u64) -> Result<Self> {
        anyhow::ensure!(
            (1..=600).contains(&timeout),
            "--timeout must be between 1 and 600 seconds"
        );
        Ok(Self {
            client: Client::builder()
                .user_agent(concat!("joog/", env!("CARGO_PKG_VERSION")))
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(timeout))
                .redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never())
                // Avoid environment-controlled proxies receiving bearer credentials.
                .no_proxy()
                .gzip(true)
                .build()?,
        })
    }

    pub fn json(
        &self,
        method: &Method,
        url: &Url,
        body: Option<&Value>,
        token: Option<&str>,
        limit: u64,
    ) -> Result<Value> {
        if token.is_some() {
            google_url(url)?;
        }
        if method == Method::GET
            && matches!(
                url.path(),
                "/v1/people:searchContacts" | "/v1/otherContacts:search"
            )
            && url
                .query_pairs()
                .any(|(name, value)| name == "query" && !value.is_empty())
        {
            let pairs = url
                .query_pairs()
                .map(|(name, value)| {
                    let value = if name == "query" {
                        String::new()
                    } else {
                        value.into_owned()
                    };
                    (name.into_owned(), value)
                })
                .collect::<Vec<_>>();
            let mut warmup = url.clone();
            warmup.query_pairs_mut().clear().extend_pairs(pairs);
            self.json(method, &warmup, None, token, limit)?;
        }
        for attempt in 0..3 {
            let mut req = self.client.request(method.clone(), url.clone());
            if let Some(token) = token {
                let mut value = header::HeaderValue::from_str(&format!("Bearer {token}"))
                    .context("invalid access token")?;
                value.set_sensitive(true);
                req = req.header(header::AUTHORIZATION, value);
            }
            if let Some(body) = body {
                req = req.json(body);
            } else if matches!(*method, Method::POST | Method::PUT | Method::PATCH) {
                req = req.header(header::CONTENT_LENGTH, 0);
            }
            let response = req.send().map_err(|e| e.without_url())?;
            let status = response.status();
            let retry = method == Method::GET
                && matches!(status.as_u16(), 429 | 502 | 503 | 504)
                && attempt < 2;
            if retry {
                let wait = response
                    .headers()
                    .get(header::RETRY_AFTER)
                    .and_then(|h| h.to_str().ok())
                    .and_then(|h| h.parse::<u64>().ok());
                let mut random = [0_u8; 1];
                getrandom::fill(&mut random).context("operating system randomness unavailable")?;
                let delay = Duration::from_secs(wait.unwrap_or(1 << attempt).min(10))
                    + Duration::from_millis(u64::from(random[0]));
                std::thread::sleep(delay);
                continue;
            }
            // Do not echo arbitrary remote error bodies, which can include secrets.
            let response = check_response(response, method, url)?;
            let bytes = limited(response, limit)?;
            return if bytes.is_empty() {
                Ok(Value::Null)
            } else {
                serde_json::from_slice(&bytes)
                    .context("Google returned invalid JSON; use --output for media")
            };
        }
        unreachable!("bounded retry loop always returns")
    }
}

pub fn check_response(
    response: reqwest::blocking::Response,
    _method: &Method,
    _url: &Url,
) -> Result<reqwest::blocking::Response> {
    let status = response.status();
    if !status.is_success() {
        #[cfg(feature = "live-test-diagnostics")]
        record_live_error(_method, _url, status, response);
        check_status(status)?;
        unreachable!("an unsuccessful status always returns an error");
    }
    Ok(response)
}

#[cfg(feature = "live-test-diagnostics")]
fn record_live_error(
    method: &Method,
    url: &Url,
    status: StatusCode,
    response: reqwest::blocking::Response,
) {
    use std::io::Write;
    let Ok(path) = std::env::var("JOOG_LIVE_ERROR_REPORT") else {
        return;
    };
    let Ok(bytes) = limited(response, 64 * 1024) else {
        return;
    };
    let mut options = std::fs::OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    if let Ok(mut file) = options.open(path) {
        let diagnostic = serde_json::json!({"method":method.as_str(),"host":url.host_str(),"path":url.path(),"status":status.as_u16(),"googleError":serde_json::from_slice::<Value>(&bytes).ok()});
        let _ = serde_json::to_writer(&mut file, &diagnostic);
        let _ = writeln!(file);
    }
}

pub fn check_status(status: StatusCode) -> Result<()> {
    if status.is_success() {
        return Ok(());
    }
    let hint = match status.as_u16() {
        400 => "check the supplied argument values and request parameters",
        401 => "sign in again with joog auth <account> login <service-list>",
        403 => {
            "sign in for this service and check that its API is enabled and the resource is shared with this account"
        }
        404 => "check the resource ID and selected Google service",
        429 => "Google quota exceeded; retry later",
        300..=399 => "redirect refused",
        _ => "Google request failed",
    };
    bail!("HTTP {}: {hint}", status.as_u16())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_search_warms_the_google_cache_before_the_real_query() {
        use std::{
            io::{BufRead, BufReader, Write},
            net::TcpListener,
            time::Instant,
        };
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let mut queries = Vec::new();
            let deadline = Instant::now() + Duration::from_secs(5);
            while queries.len() < 4 {
                let (mut stream, _) = match listener.accept() {
                    Ok(pair) => pair,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(
                            Instant::now() < deadline,
                            "expected both a warm-up and a real search"
                        );
                        std::thread::sleep(Duration::from_millis(5));
                        continue;
                    }
                    Err(error) => panic!("{error}"),
                };
                stream.set_nonblocking(false).unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut reader = BufReader::new(&stream);
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                let path = line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&format!("http://{address}{path}")).unwrap();
                let pairs = url
                    .query_pairs()
                    .into_owned()
                    .collect::<std::collections::BTreeMap<_, _>>();
                assert_eq!(pairs["readMask"], "names,emailAddresses");
                assert_eq!(pairs["pageSize"], "30");
                queries.push(pairs["query"].clone());
                loop {
                    line.clear();
                    reader.read_line(&mut line).unwrap();
                    if line == "\r\n" {
                        break;
                    }
                }
                write!(stream,"HTTP/1.1 200 OK\r\nContent-Length: 14\r\nConnection: close\r\n\r\n{{\"results\":[]}}").unwrap();
            }
            queries
        });
        for path in ["/v1/people:searchContacts", "/v1/otherContacts:search"] {
            let url = Url::parse(&format!(
                "http://{address}{path}?query=Alice&readMask=names,emailAddresses&pageSize=30"
            ))
            .unwrap();
            let result = Http::new(2)
                .unwrap()
                .json(&Method::GET, &url, None, None, 1024)
                .unwrap();
            assert_eq!(result, serde_json::json!({"results":[]}));
        }
        assert_eq!(server.join().unwrap(), ["", "Alice", "", "Alice"]);
    }

    #[test]
    fn credentials_only_go_to_google_https() {
        for url in [
            "https://gmail.googleapis.com/gmail/v1",
            "https://www.googleapis.com/drive/v3",
        ] {
            assert!(google_url(&Url::parse(url).unwrap()).is_ok());
        }
        for url in [
            "http://gmail.googleapis.com",
            "https://googleapis.com.attacker.test",
            "https://evilgoogleapis.com",
            "https://user@gmail.googleapis.com",
            "https://gmail.googleapis.com:444",
            "https://gmail.googleapis.com/#x",
        ] {
            assert!(google_url(&Url::parse(url).unwrap()).is_err(), "{url}");
        }
    }

    #[test]
    fn response_limit_detects_truncation() {
        assert_eq!(limited(&b"1234"[..], 4).unwrap(), b"1234");
        assert!(limited(&b"12345"[..], 4).is_err());
    }

    #[test]
    fn redirects_are_refused_and_error_bodies_are_not_echoed() {
        use std::{io::Write, net::TcpListener};
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0_u8; 2048];
            let n = stream.read(&mut buf).unwrap();
            assert!(buf[..n].starts_with(b"GET "));
            let body = "sensitive-reflected-data";
            write!(stream, "HTTP/1.1 302 Found\r\nLocation: http://{address}/other\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
            listener
        });
        let error = Http::new(2)
            .unwrap()
            .json(
                &Method::GET,
                &Url::parse(&format!("http://{address}/")).unwrap(),
                None,
                None,
                1024,
            )
            .unwrap_err();
        assert!(error.to_string().contains("redirect refused"));
        assert!(!error.to_string().contains("sensitive-reflected-data"));
        let listener = server.join().unwrap();
        listener.set_nonblocking(true).unwrap();
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }

    #[test]
    fn bodyless_mutations_send_a_zero_content_length() {
        use std::{
            io::{BufRead, BufReader, Write},
            net::TcpListener,
        };
        for method in [Method::POST, Method::PUT, Method::PATCH] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut reader = BufReader::new(&stream);
                let mut headers = String::new();
                loop {
                    let mut line = String::new();
                    assert!(reader.read_line(&mut line).unwrap() > 0);
                    if line == "\r\n" {
                        break;
                    }
                    headers.push_str(&line);
                }
                assert!(
                    headers
                        .to_ascii_lowercase()
                        .contains("content-length: 0\r\n")
                );
                write!(
                    stream,
                    "HTTP/1.1 204 No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                )
                .unwrap();
            });
            assert!(
                Http::new(2)
                    .unwrap()
                    .json(
                        &method,
                        &Url::parse(&format!("http://{address}/")).unwrap(),
                        None,
                        None,
                        1024
                    )
                    .unwrap()
                    .is_null()
            );
            server.join().unwrap();
        }
    }

    #[test]
    fn writes_are_not_retried_on_service_failure() {
        use std::{io::Write, net::TcpListener};
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0_u8; 2048];
            let n = stream.read(&mut buf).unwrap();
            assert!(buf[..n].starts_with(b"POST "));
            write!(
                stream,
                "HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .unwrap();
            listener
        });
        assert!(
            Http::new(2)
                .unwrap()
                .json(
                    &Method::POST,
                    &Url::parse(&format!("http://{address}/")).unwrap(),
                    None,
                    None,
                    1024
                )
                .is_err()
        );
        let listener = server.join().unwrap();
        listener.set_nonblocking(true).unwrap();
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }

    #[test]
    fn adversarial_auth_failures_stop_after_one_request_without_echoing_the_body() {
        use std::{io::Write, net::TcpListener};
        for status in ["401 Unauthorized", "403 Forbidden"] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut request = [0; 2048];
                assert!(stream.read(&mut request).unwrap() > 0);
                let body = "private-code<script>injected</script>";
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .unwrap();
                listener
            });
            let error = Http::new(2)
                .unwrap()
                .json(
                    &Method::GET,
                    &Url::parse(&format!("http://{address}/")).unwrap(),
                    None,
                    None,
                    1024,
                )
                .unwrap_err();
            assert!(error.to_string().contains(&status[..3]));
            assert!(!error.to_string().contains("private-code"));
            assert!(!error.to_string().contains("<script>"));
            let listener = server.join().unwrap();
            listener.set_nonblocking(true).unwrap();
            assert_eq!(
                listener.accept().unwrap_err().kind(),
                std::io::ErrorKind::WouldBlock
            );
        }
    }
}
