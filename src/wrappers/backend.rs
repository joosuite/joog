use super::{Request, values};
use anyhow::{Context, Result, ensure};
use reqwest::{Method, blocking::Body};
use serde_json::{Value, json};
use std::{ffi::OsString, io::Read};

pub trait Backend {
    fn call(&mut self, service: &str, command: &str, params: Value) -> Result<Value>;
    fn upload_photo(&mut self, path: &str) -> Result<String>;
    fn download_photo(&mut self, url: &str, path: &str) -> Result<Value>;
}

pub struct Joog {
    account: String,
    timeout: u64,
    store: crate::store::Store,
    http: crate::http::Http,
}

impl Joog {
    pub fn new(request: &Request) -> Result<Self> {
        let timeout = request.params["timeout"].as_u64().unwrap_or(30);
        Ok(Self {
            account: request.account.clone(),
            timeout,
            store: crate::store::Store::new(request.text("home"))?,
            http: crate::http::Http::new(timeout)?,
        })
    }
    fn access(&self) -> Result<zeroize::Zeroizing<String>> {
        crate::auth::access_token(&self.store, &self.http, &self.account)
    }
}

pub fn merge_pages(bytes: &[u8]) -> Result<Value> {
    let mut result = Value::Null;
    for page in serde_json::Deserializer::from_slice(bytes).into_iter::<Value>() {
        let page = page.context("joog returned invalid JSON")?;
        if result.is_null() {
            result = page;
        } else if let (Some(target), Some(page)) = (result.as_object_mut(), page.as_object()) {
            for (key, value) in page {
                ensure!(
                    target
                        .get(key)
                        .is_none_or(|previous| previous.is_array() == value.is_array()),
                    "joog returned an inconsistent paginated collection"
                );
                if let Some(rows) = target.get_mut(key).and_then(Value::as_array_mut) {
                    if let Some(more) = value.as_array() {
                        rows.extend(more.iter().cloned());
                    }
                } else {
                    target.insert(key.clone(), value.clone());
                }
            }
        } else {
            anyhow::bail!("unexpected paginated output from joog");
        }
    }
    if let Some(object) = result.as_object_mut() {
        object.remove("nextPageToken");
    }
    Ok(result)
}

impl Backend for Joog {
    fn call(&mut self, service: &str, command: &str, params: Value) -> Result<Value> {
        let file = tempfile::NamedTempFile::new()?;
        serde_json::to_writer(file.as_file(), &params)?;
        let mut args: Vec<OsString> = [
            "joog",
            "--json",
            "--timeout",
            &self.timeout.to_string(),
            service,
            &self.account,
            command,
            "--file",
        ]
        .into_iter()
        .map(OsString::from)
        .collect();
        args.push(file.path().as_os_str().to_owned());
        let root = crate::cli::parse_from(args).context("invalid wrapper request")?;
        let definition = crate::services::SERVICES
            .iter()
            .find(|item| item.cli == service)
            .context("unknown Google service")?;
        let (_, context) = root.subcommand().context("missing service")?;
        let call = crate::service_call(definition, context)?;
        let doc = &crate::schema_commands::contract(service).document;
        let mut bytes = Vec::new();
        crate::execute_with_output(
            &root,
            crate::native::arguments(context),
            (&self.store, &self.http),
            doc,
            call,
            &self.account,
            &mut |value, _| {
                serde_json::to_writer(&mut bytes, value)?;
                bytes.push(b'\n');
                ensure!(
                    bytes.len() as u64 <= crate::http::JSON_LIMIT,
                    "result exceeds 64 MiB; use a smaller limit:"
                );
                Ok(())
            },
        )
        .with_context(|| format!("{service} {command}"))?;
        merge_pages(&bytes)
    }

    fn upload_photo(&mut self, path: &str) -> Result<String> {
        let token = self.access()?;
        crate::cache::invalidate(&self.store.root, &self.account)?;
        let file = std::fs::File::open(path)?;
        let size = file.metadata()?.len();
        let response = self
            .http
            .client
            .request(
                Method::POST,
                "https://photoslibrary.googleapis.com/v1/uploads",
            )
            .bearer_auth(token.as_str())
            .header("Content-Type", "application/octet-stream")
            .header("X-Goog-Upload-Protocol", "raw")
            .header("X-Goog-Upload-Content-Type", values::upload_mime(path))
            .header("Content-Length", size)
            .body(Body::new(file.take(size)))
            .send()
            .map_err(|error| error.without_url())?;
        crate::http::check_status(response.status())?;
        let bytes = crate::http::limited(response, 1024 * 1024)?;
        let token = String::from_utf8(bytes).context("Photos returned an invalid upload token")?;
        ensure!(
            !token.trim().is_empty(),
            "Photos returned an empty upload token"
        );
        Ok(token)
    }

    fn download_photo(&mut self, address: &str, path: &str) -> Result<Value> {
        let url = url::Url::parse(address)?;
        let host = url.host_str().unwrap_or_default();
        ensure!(
            url.scheme() == "https"
                && (host == "googleusercontent.com" || host.ends_with(".googleusercontent.com"))
                && url.username().is_empty()
                && url.password().is_none()
                && url.port_or_known_default() == Some(443),
            "Photos returned an unsupported media URL"
        );
        let token = self.access()?;
        let response = self
            .http
            .client
            .get(url)
            .bearer_auth(token.as_str())
            .send()
            .map_err(|error| error.without_url())?;
        crate::http::check_status(response.status())?;
        let parent = std::path::Path::new(path)
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or(std::path::Path::new("."));
        let mut file = tempfile::NamedTempFile::new_in(parent)?;
        let size = std::io::copy(&mut response.take(1024 * 1024 * 1024 + 1), &mut file)?;
        ensure!(size <= 1024 * 1024 * 1024, "download exceeds 1 GiB");
        file.as_file().sync_all()?;
        file.persist_noclobber(path).map_err(|error| error.error)?;
        Ok(json!({"path":path,"bytes":size}))
    }
}
