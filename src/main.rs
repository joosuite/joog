mod auth;
mod cache;
mod cli;
mod discovery;
mod file_inputs;
mod help;
mod http;
mod mail;
mod native;
mod output;
mod pagination;
mod presentation;
mod schema_commands;
mod services;
mod store;
mod wrappers;

use anyhow::{Context, Result};
use clap::ArgMatches;
use reqwest::Method;
use serde_json::{Map, Value, json};
use std::{
    io::{Read, Write},
    process::ExitCode,
};
use zeroize::Zeroizing;

fn main() -> ExitCode {
    let mut raw = std::env::args_os();
    if let Some(name) = raw.next().as_deref().and_then(wrappers::executable) {
        return wrappers::main(&name, raw.collect());
    }
    let matches = cli::parse_from(std::env::args_os()).unwrap_or_else(|e| e.exit());
    match run(&matches) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error)
            if error.chain().any(|e| {
                e.downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::BrokenPipe)
            }) =>
        {
            ExitCode::SUCCESS
        }
        Err(error) => {
            let mut stderr = std::io::stderr().lock();
            if matches.get_flag("json") {
                let _ = serde_json::to_writer(&mut stderr, &json!({"error":format!("{error:#}")}));
                let _ = writeln!(stderr);
            } else {
                let _ = writeln!(stderr, "error: {error:#}");
            }
            ExitCode::FAILURE
        }
    }
}

fn string<'a>(args: &'a ArgMatches, key: &str) -> Option<&'a str> {
    native::text(args, key)
}

fn run(root: &ArgMatches) -> Result<()> {
    let store = store::Store::new(string(root, "home"))?;
    let http = http::Http::new(*root.get_one::<u64>("timeout").context("missing timeout")?)?;
    let json = root.get_flag("json");
    let (command, args) = root.subcommand().context("choose a Google service")?;
    match command {
        "auth" => {
            let account = string(args, "account");
            let (command, args) = args.subcommand().context("choose an auth command")?;
            match command {
                "init" => {
                    if args.get_flag("headless") {
                        store.initialize_headless()?;
                    } else {
                        store.initialize()?;
                    }
                    output::status(
                        "Credential vault initialized.",
                        &json!({"initialized":true,"master_key":if store.uses_key_file()? {"file"} else {"os-keychain"},"secrets":"encrypted-files"}),
                        json,
                    )
                }
                "enroll" => {
                    auth::enroll(&store, args.get_flag("stdin"))?;
                    output::status(
                        "Google client credentials saved.",
                        &json!({"stored":"oauth-client","backend":"encrypted-vault"}),
                        json,
                    )
                }
                "test" => {
                    auth::test(&store, &http)?;
                    output::status(
                        "Google connection successful.",
                        &json!({"google":"reachable","oauth_client":"not_rejected"}),
                        json,
                    )
                }
                "list" => output::emit(&json!(store.config()?.accounts), json, false),
                "login" | "all" => {
                    let scopes = services::scopes(if command == "all" {
                        "all"
                    } else {
                        string(args, "services").context("missing service list")?
                    })?;
                    let email = auth::login(&store, &http, scopes, account)?;
                    let _ = cache::invalidate(&store.root, &email);
                    output::status(
                        &format!("Signed in as {email}."),
                        &json!({"account":email}),
                        json,
                    )
                }
                "export" | "import" => {
                    let path = std::path::Path::new(
                        string(args, "transfer-file").context("missing transfer file")?,
                    );
                    let passphrase_file = string(args, "passphrase-file").map(std::path::Path::new);
                    let accounts = if command == "export" {
                        auth::transfer::export(&store, account, path, passphrase_file)?
                    } else {
                        auth::transfer::import(
                            &store,
                            account,
                            path,
                            passphrase_file,
                            args.get_flag("headless"),
                            args.get_flag("replace"),
                        )?
                    };
                    output::status(
                        &format!(
                            "{} credentials for {}{}.",
                            if command == "export" {
                                "Exported"
                            } else {
                                "Imported"
                            },
                            accounts.join(", "),
                            if command == "export" {
                                format!(" to {}", path.display())
                            } else {
                                String::new()
                            }
                        ),
                        &json!({"action":command,"accounts":accounts,"file":path}),
                        json,
                    )
                }
                "logout" => {
                    let email = auth::logout(&store, account)?;
                    let _ = cache::invalidate(&store.root, &email);
                    output::status(
                        &format!("Signed out {email}."),
                        &json!({"removed":email}),
                        json,
                    )
                }
                _ => unreachable!(),
            }
        }
        service => {
            let service = services::SERVICES
                .iter()
                .find(|s| s.cli == service)
                .context("unknown Google service")?;
            let account = string(args, "account").context("missing account")?;
            let call = service_call(service, args)?;
            let args = native::arguments(args);
            let doc = &schema_commands::contract(service.cli).document;
            execute(root, args, &store, &http, doc, call, account)
        }
    }
}

fn service_call(service: &services::Service, args: &ArgMatches) -> Result<services::Invocation> {
    if let Some((action, command)) = args.subcommand() {
        let (parameters, effective) =
            file_inputs::prepare(service.cli, action, command, parameters(command)?)?;
        let command = &effective;
        if schema_commands::contract(service.cli)
            .document
            .root
            .methods
            .contains_key(action)
        {
            let mut call = schema_commands::resolve(
                service.cli,
                string(args, "account").context("missing account")?,
                action,
                command,
                parameters,
            )?;
            call.options = Some(effective);
            return Ok(call);
        }
        anyhow::ensure!(
            !parameters.contains_key("body"),
            "this command builds its own body; use its named options or a schema method command"
        );
        let original = parameters.clone();
        let mut call = native::resolve(
            service.cli,
            string(args, "account").context("missing account")?,
            action,
            command,
            parameters,
        )?;
        let spec = schema_commands::contract(service.cli)
            .document
            .method(&call.method)?;
        schema_commands::validate_file_parameters(spec, &original)?;
        file_inputs::validate_constraints(
            service.cli,
            action,
            command,
            file_inputs::RequestValues {
                params: &call.params,
                body: call.body.as_ref(),
                envelope: &Map::new(),
                compose: None,
            },
        )?;
        call.options = Some(effective);
        return Ok(call);
    }
    anyhow::bail!("choose a command; use joog {} --help", service.cli)
}

fn read_json(text: &str) -> Result<Value> {
    anyhow::ensure!(
        text.len() as u64 <= http::DISCOVERY_LIMIT,
        "JSON input exceeds size limit"
    );
    serde_json::from_str(text).context("invalid JSON input")
}

fn parameters(args: &ArgMatches) -> Result<Map<String, Value>> {
    let Some(path) = string(args, "file") else {
        return Ok(Map::new());
    };
    let bytes = Zeroizing::new(http::limited(
        std::fs::File::open(path).context("cannot read --file")?,
        http::DISCOVERY_LIMIT,
    )?);
    let value = serde_json::from_slice(&bytes).context("--file must contain valid JSON")?;
    match value {
        Value::Object(parameters) => Ok(parameters),
        _ => anyhow::bail!("--file must contain a JSON object"),
    }
}

fn execute(
    root: &ArgMatches,
    args: &ArgMatches,
    store: &store::Store,
    http: &http::Http,
    doc: &discovery::Document,
    call: services::Invocation,
    account: &str,
) -> Result<()> {
    let (service, context) = root.subcommand().context("missing service")?;
    let command = context.subcommand_name().filter(|_| !call.custom_fields);
    let file_output = call
        .options
        .as_ref()
        .unwrap_or(args)
        .try_get_one::<String>("output")
        .ok()
        .flatten()
        .is_some();
    execute_with_output(
        root,
        args,
        (store, http),
        doc,
        call,
        account,
        &mut |value, paginated| {
            if file_output {
                output::status(
                    &format!(
                        "Saved {} ({} bytes).",
                        value["path"].as_str().unwrap_or_default(),
                        value["bytes"]
                    ),
                    value,
                    root.get_flag("json"),
                )
            } else {
                presentation::emit(service, command, value, root.get_flag("json"), paginated)
            }
        },
    )
}

fn execute_with_output(
    root: &ArgMatches,
    args: &ArgMatches,
    session: (&store::Store, &http::Http),
    doc: &discovery::Document,
    call: services::Invocation,
    account: &str,
    emit: &mut dyn FnMut(&Value, bool) -> Result<()>,
) -> Result<()> {
    let result = execute_request_with_output(root, args, session, doc, call, account, emit);
    #[cfg(feature = "live-test-diagnostics")]
    if let Ok(path) = std::env::var("JOOG_LIVE_COMMAND_REPORT") {
        let mut options = std::fs::OpenOptions::new();
        options.create(true).append(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        if let (Some((service, context)), Ok(mut file)) = (root.subcommand(), options.open(path)) {
            let record = json!({"kind":"wire-command","interface":"joog","service":service,"command":context.subcommand_name(),"status":if result.is_ok(){"passed"}else{"request-failed"}});
            let _ = serde_json::to_writer(&mut file, &record);
            let _ = writeln!(file);
        }
    }
    result
}

fn execute_request_with_output(
    root: &ArgMatches,
    args: &ArgMatches,
    session: (&store::Store, &http::Http),
    doc: &discovery::Document,
    mut call: services::Invocation,
    account: &str,
    emit: &mut dyn FnMut(&Value, bool) -> Result<()>,
) -> Result<()> {
    let (store, http) = session;
    let effective = call.options.take();
    let args = effective.as_ref().unwrap_or(args);
    let spec = doc.method(&call.method)?;
    let method = Method::from_bytes(spec.http_method.as_bytes())?;
    anyhow::ensure!(
        matches!(
            method,
            Method::GET
                | Method::HEAD
                | Method::POST
                | Method::PUT
                | Method::PATCH
                | Method::DELETE
        ),
        "unsupported HTTP method"
    );
    let read = method == Method::GET || method == Method::HEAD || spec.read_only;
    anyhow::ensure!(
        !matches!(method, Method::GET | Method::HEAD) || call.body.is_none(),
        "GET and HEAD requests cannot have a body"
    );
    if let Some(fields) = string(args, "fields") {
        call.params.insert("fields".into(), json!(fields));
    }
    let output = string(args, "output");
    let upload = string(args, "upload").or_else(|| {
        (root.subcommand().is_some_and(|(service, context)| {
            service == "drive" && context.subcommand_name() == Some("upload")
        }))
        .then(|| string(args, "path"))
        .flatten()
    });
    let maximum = args.try_get_one::<u32>("max").ok().flatten().copied();
    let pages = args.try_get_one::<u32>("max-pages").ok().flatten().copied();
    anyhow::ensure!(
        upload.is_none() || (!read && output.is_none() && maximum.is_none() && pages.is_none()),
        "--upload requires a write method and cannot be combined with --output, --max, or --max-pages"
    );
    anyhow::ensure!(
        output.is_none() || (maximum.is_none() && pages.is_none()),
        "--max and --max-pages cannot be combined with --output"
    );
    if output.is_some() {
        anyhow::ensure!(method == Method::GET, "--output requires a GET method");
        if spec.supports_media_download {
            call.params.entry("alt").or_insert(json!("media"));
        }
    }
    let url = if upload.is_some() {
        doc.upload_url(spec, &call.params)?
    } else {
        doc.url(spec, &call.params)?
    };
    anyhow::ensure!(
        call.body.is_none() || spec.request.is_some(),
        "this command does not accept a request body"
    );
    if let Some(path) = output {
        anyhow::ensure!(
            !std::path::Path::new(path).exists(),
            "output path already exists"
        );
    }
    if let Some(path) = upload {
        let meta = std::fs::metadata(path).context("cannot read upload file")?;
        anyhow::ensure!(
            meta.is_file() && meta.len() <= 1024 * 1024 * 1024,
            "upload must be a regular file up to 1 GiB"
        );
    }
    let plan = pagination::Plan::new(spec, read, maximum, pages)?;
    if let Some(path) = upload {
        schema_commands::validate_upload(
            spec,
            path,
            string(args, "content-type").unwrap_or(schema_commands::upload_mime(spec)),
        )?;
    }
    let token = if spec.anonymous {
        None
    } else {
        Some(auth::access_token(store, http, account)?)
    };
    let authorization = token.as_ref().map(|value| value.as_str());
    if let Some(path) = upload {
        cache::invalidate(&store.root, account)
            .context("cannot invalidate cached data before upload")?;
        let value = upload_file(
            http,
            &method,
            &url,
            authorization.context("upload requires an authenticated method")?,
            path,
            string(args, "content-type").unwrap_or("application/octet-stream"),
            call.body.as_ref(),
        )?;
        let _ = cache::invalidate(&store.root, account);
        return emit(&value, false);
    }
    if let Some(path) = output {
        if doc.name == "gmail" && spec.id == "gmail.users.messages.attachments.get" {
            use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
            let value = http.json(&Method::GET, &url, None, authorization, 96 * 1024 * 1024)?;
            let data = value["data"]
                .as_str()
                .context("Google did not return attachment data")?;
            let bytes = URL_SAFE_NO_PAD
                .decode(data.trim_end_matches('='))
                .context("Google returned invalid attachment encoding")?;
            return emit(&save_content(std::io::Cursor::new(bytes), path)?, false);
        }
        return emit(
            &download(
                http,
                &url,
                authorization.context("download requires an authenticated method")?,
                path,
            )?,
            false,
        );
    }
    let ttl = args
        .try_get_one::<u32>("ttl")
        .ok()
        .flatten()
        .copied()
        .unwrap_or(0);
    let responses = if read && !spec.anonymous && !spec.fresh_response && ttl > 0 {
        cache::Responses::new(&store.root, account, store.master_key()?, ttl).ok()
    } else {
        None
    };
    if !read {
        cache::invalidate(&store.root, account)
            .context("cannot invalidate cached data before sending the change")?;
    }
    let result = pagination::request(
        spec,
        &mut call.params,
        &mut call.body,
        plan,
        |params, body| {
            let url = doc.url(spec, params)?;
            let fetch = || http.json(&method, &url, body, authorization, http::JSON_LIMIT);
            match &responses {
                Some(cache) => {
                    let identity = cache::body_identity(
                        cache::request_identity(doc, spec, params, &call.relative_times)?,
                        body,
                    )?;
                    cache.fetch(&identity, fetch)
                }
                None => fetch(),
            }
        },
        emit,
    );
    if !read {
        let _ = cache::invalidate(&store.root, account);
    }
    result
}

fn upload_file(
    http: &http::Http,
    method: &Method,
    url: &url::Url,
    token: &str,
    path: &str,
    content_type: &str,
    metadata: Option<&Value>,
) -> Result<Value> {
    http::google_url(url)?;
    let file = std::fs::File::open(path).context("cannot open upload file")?;
    let meta = file.metadata()?;
    anyhow::ensure!(
        meta.is_file() && meta.len() <= 1024 * 1024 * 1024,
        "upload must be a regular file of at most 1 GiB"
    );
    let content_type =
        reqwest::header::HeaderValue::from_str(content_type).context("invalid --content-type")?;
    let request = http
        .client
        .request(method.clone(), url.clone())
        .bearer_auth(token)
        .header("x-upload-content-type", content_type.clone())
        .header("x-upload-content-length", meta.len())
        .json(metadata.unwrap_or(&json!({})));
    let response = request.send().map_err(|e| e.without_url())?;
    let response = http::check_response(response, method, url)?;
    let location = response
        .headers()
        .get(reqwest::header::LOCATION)
        .context("Google did not return an upload session URL")?
        .to_str()?;
    let location = url::Url::parse(location).context("invalid upload session URL")?;
    http::google_url(&location)?;
    drop(response);
    let request = http
        .client
        .put(location.clone())
        .bearer_auth(token)
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .header(reqwest::header::CONTENT_LENGTH, meta.len())
        .body(reqwest::blocking::Body::new(file.take(meta.len())));
    let response = request.send().map_err(|e| e.without_url())?;
    let response = http::check_response(response, &Method::PUT, &location)?;
    let bytes = http::limited(response, http::JSON_LIMIT)?;
    if bytes.is_empty() {
        Ok(Value::Null)
    } else {
        serde_json::from_slice(&bytes).context("Google returned invalid upload JSON")
    }
}

fn download(http: &http::Http, url: &url::Url, token: &str, path: &str) -> Result<Value> {
    http::google_url(url)?;
    let path = std::path::Path::new(path);
    anyhow::ensure!(!path.exists(), "output path already exists");
    let request = http.client.get(url.clone()).bearer_auth(token);
    let response = request.send().map_err(|e| e.without_url())?;
    http::check_status(response.status())?;
    const LIMIT: u64 = 1024 * 1024 * 1024;
    anyhow::ensure!(
        response.content_length().is_none_or(|n| n <= LIMIT),
        "download exceeds 1 GiB limit"
    );
    save_content(response, path.to_str().context("output path is not UTF-8")?)
}

fn save_content(reader: impl Read, path: &str) -> Result<Value> {
    const LIMIT: u64 = 1024 * 1024 * 1024;
    let path = std::path::Path::new(path);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    let size = std::io::copy(&mut reader.take(LIMIT + 1), &mut tmp)?;
    anyhow::ensure!(size <= LIMIT, "download exceeds 1 GiB limit");
    tmp.as_file().sync_all()?;
    tmp.persist_noclobber(path).map_err(|e| e.error)?;
    Ok(json!({"path":path,"bytes":size}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare(args: &[&str]) -> (String, services::Invocation) {
        let parsed = cli::parse_from(args.iter().copied()).unwrap();
        let (name, context) = parsed.subcommand().unwrap();
        let service = services::SERVICES.iter().find(|s| s.cli == name).unwrap();
        (
            string(context, "account").unwrap().to_owned(),
            service_call(service, context).unwrap(),
        )
    }

    #[test]
    fn service_actions_preserve_parameters_and_build_write_requests_directly() {
        let (account, list) = prepare(&["joog", "drive", "reader@example.com", "files.list"]);
        assert_eq!(account, "reader@example.com");
        assert_eq!(list.method, "files.list");
        let (_, restore) = prepare(&["joog", "drive", "writer@example.com", "restore", "AbC123"]);
        assert_eq!(restore.method, "files.update");
        assert_eq!(restore.body.unwrap()["trashed"], false);
        let (_, raw) = prepare(&[
            "joog",
            "drive",
            "writer@example.com",
            "files.create",
            "--body",
            "{\"name\":\"Example\"}",
        ]);
        assert_eq!(raw.method, "files.create");
        assert_eq!(raw.body.unwrap()["name"], "Example");
    }

    #[test]
    fn every_service_has_a_schema_and_an_example_accepted_by_request_validation() {
        let fixtures: Value =
            serde_json::from_str(include_str!("../tests/data/file-parameters.json")).unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        assert_eq!(
            fixtures.as_object().unwrap().len(),
            services::SERVICES.len()
        );
        for service in services::SERVICES {
            let schema: Value = serde_json::from_slice(
                &std::fs::read(root.join(format!("schemas/{}.schema.json", service.cli))).unwrap(),
            )
            .unwrap();
            assert_eq!(
                schema["$schema"],
                "https://json-schema.org/draft/2020-12/schema"
            );
            assert_eq!(
                schema["title"],
                format!("Joog {} command parameters", service.cli)
            );
            assert!(schema.get("x-google-methods").is_none());
            assert!(schema.get("x-body-types").is_none());
            let definitions = schema["$defs"].as_object().unwrap();
            assert!(!definitions.is_empty());
            assert_eq!(schema["anyOf"].as_array().unwrap().len(), definitions.len());
            for definition in definitions.values() {
                assert_eq!(definition["additionalProperties"], false);
                let properties = definition["properties"].as_object().unwrap();
                assert!(!properties.contains_key("access_token"));
                assert!(!properties.contains_key("oauth_token"));
            }
            let fixture = &fixtures[service.cli];
            let method = fixture["method"].as_str().unwrap();
            assert!(definitions.contains_key(method));
            let path = root.join(format!("examples/requests/{}.json", service.cli));
            let (_, call) = prepare(&[
                "joog",
                service.cli,
                "reader@example.com",
                method,
                "--file",
                path.to_str().unwrap(),
            ]);
            let document: discovery::Document =
                serde_json::from_value(fixture["document"].clone()).unwrap();
            let method_id = &schema_commands::contract(service.cli)
                .document
                .method(&call.method)
                .unwrap()
                .id;
            let specification = document.method(method_id).unwrap();
            let url = document
                .url(specification, &call.params)
                .unwrap_or_else(|error| panic!("{} example: {error}", service.cli));
            http::google_url(&url).unwrap();
        }
    }

    #[test]
    fn request_parameters_load_from_a_direct_file_path_and_body_stays_inline() {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), br#"{"fileId":"AbC123","fields":"id,name"}"#).unwrap();
        let (_, call) = prepare(&[
            "joog",
            "drive",
            "reader@example.com",
            "files.update",
            "--file",
            file.path().to_str().unwrap(),
            "--body",
            r#"{"name":"Updated"}"#,
        ]);
        assert_eq!(call.method, "files.update");
        assert_eq!(
            call.params,
            json!({"fileId":"AbC123","fields":"id,name"})
                .as_object()
                .unwrap()
                .clone()
        );
        assert_eq!(call.body.unwrap(), json!({"name":"Updated"}));
        assert!(read_json("@-").is_err());
        assert!(read_json(&format!("@{}", file.path().display())).is_err());
    }

    #[test]
    fn json_preserves_large_sheet_numbers() {
        let text = "{\"values\":[[18446744073709551616001]]}";
        assert_eq!(super::read_json(text).unwrap().to_string(), text);
    }
}
