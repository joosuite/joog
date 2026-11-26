//! Explicitly opted-in Google checks. Normal `cargo test` never runs these.
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use sha2::Digest;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Output, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[path = "live/exhaustive.rs"]
mod exhaustive;
#[path = "live/recovery.rs"]
mod recovery;

fn bounded_process(command: &mut Command, deadline: Duration) -> Result<(Output, bool)> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("cannot run test executable")?;
    let stdout = child.stdout.take().context("child stdout missing")?;
    let stderr = child.stderr.take().context("child stderr missing")?;
    let collect = |stream: Box<dyn Read + Send>| {
        thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut bytes = Vec::new();
            stream.take(128 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
            Ok(bytes)
        })
    };
    let out = collect(Box::new(stdout));
    let err = collect(Box::new(stderr));
    let started = Instant::now();
    let mut expired = false;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= deadline {
            expired = true;
            child.kill()?;
            break child.wait()?;
        }
        thread::sleep(Duration::from_millis(20));
    };
    let stdout = out
        .join()
        .map_err(|_| anyhow::anyhow!("stdout reader failed"))??;
    let stderr = err
        .join()
        .map_err(|_| anyhow::anyhow!("stderr reader failed"))??;
    ensure!(
        stdout.len() <= 128 * 1024 * 1024 && stderr.len() <= 128 * 1024 * 1024,
        "test process output exceeds its limit"
    );
    Ok((
        Output {
            status,
            stdout,
            stderr,
        },
        expired,
    ))
}

fn bound_pages(schema: &Value, params: &mut Value) {
    let incompatible = schema["allOf"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|rule| {
            rule["x-cli-kind"] == "conflict"
                && rule["x-cli-parameters"].as_array().is_some_and(|names| {
                    names.contains(&json!("maxPages"))
                        && names
                            .iter()
                            .filter_map(Value::as_str)
                            .any(|name| name != "maxPages" && params.get(name).is_some())
                })
        });
    if schema["properties"].get("maxPages").is_some()
        && !incompatible
        && params.get("maxPages").is_none()
    {
        params["maxPages"] = json!(1);
    }
}

#[test]
fn page_bounds_respect_id_lookup_conflicts() {
    let schema: Value =
        serde_json::from_str(include_str!("../schemas/youtube.schema.json")).unwrap();
    let schema = &schema["$defs"]["comments.list"];
    let mut id = json!({"id":["fixture"],"max":1});
    bound_pages(schema, &mut id);
    assert!(id.get("maxPages").is_none());
    let mut replies = json!({"parentId":"fixture","max":1});
    bound_pages(schema, &mut replies);
    assert_eq!(replies["maxPages"], 1);
}

#[test]
fn a_stuck_command_is_killed_without_retries() {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([
            "--exact",
            "deadline_child_fixture",
            "--ignored",
            "--nocapture",
        ])
        .env("JOOG_DEADLINE_CHILD", "1");
    let started = Instant::now();
    let (output, expired) = bounded_process(&mut command, Duration::from_millis(100)).unwrap();
    assert!(expired);
    assert!(!output.status.success());
    assert!(started.elapsed() < Duration::from_secs(3));
}

#[test]
#[ignore = "child process fixture for the offline deadline regression"]
fn deadline_child_fixture() {
    assert_eq!(std::env::var("JOOG_DEADLINE_CHILD").as_deref(), Ok("1"));
    thread::sleep(Duration::from_secs(10));
}

fn decode_pages(bytes: &[u8]) -> Result<Value> {
    let mut pages = serde_json::Deserializer::from_slice(bytes).into_iter::<Value>();
    let mut result = pages.next().context("command returned no JSON")??;
    for page in pages {
        let page = page?;
        let target = result
            .as_object_mut()
            .context("incompatible paginated JSON")?;
        let page = page.as_object().context("incompatible paginated JSON")?;
        for (key, value) in page {
            if let Some(previous) = target.get_mut(key) {
                ensure!(
                    previous.is_array() == value.is_array(),
                    "incompatible paginated collection"
                );
                if let (Some(previous), Some(next)) = (previous.as_array_mut(), value.as_array()) {
                    previous.extend(next.iter().cloned());
                    continue;
                }
            }
            target.insert(key.clone(), value.clone());
        }
    }
    Ok(result)
}

#[test]
fn live_client_decodes_bounded_multi_page_results_without_dropping_records() {
    assert_eq!(
        decode_pages(b"{\"items\":[{\"id\":\"a\"}]}\n{\"items\":[{\"id\":\"b\"}]}").unwrap(),
        json!({"items":[{"id":"a"},{"id":"b"}]})
    );
    assert!(decode_pages(b"{\"items\":[]}\n{\"items\":true}").is_err());
    assert!(decode_pages(b"").is_err());
}

#[derive(Clone)]
struct Cleanup {
    service: &'static str,
    command: &'static str,
    params: Value,
    id: String,
    verification: Option<Value>,
}

struct Live {
    account: String,
    directory: tempfile::TempDir,
    bin_directory: PathBuf,
    _run_lock: fs::File,
    report: PathBuf,
    events: Vec<Value>,
    pending: Vec<Cleanup>,
    last_call: Option<Instant>,
    calls: usize,
    stop: bool,
    marker: String,
    max_commands: usize,
    cleaning: bool,
    trace_count: usize,
    scenario_failed: bool,
    last_remote_reason: Option<String>,
    binary_sha256: String,
    halt_after_cleanup: bool,
    command_schemas: std::collections::BTreeMap<String, Value>,
    manual_cleanup: Vec<Value>,
}

impl Live {
    fn new() -> Result<Self> {
        let account = std::env::var("JOOG_LIVE_ACCOUNT")
            .context("set JOOG_LIVE_ACCOUNT to the explicitly approved test account")?;
        ensure!(
            account.contains('@') && !account.contains(['\r', '\n']),
            "invalid test account"
        );
        let directory = tempfile::tempdir()?;
        let bin_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/live-test-bin");
        fs::create_dir_all(&bin_directory)?;
        let run_lock = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(bin_directory.join(".run.lock"))?;
        fs2::FileExt::try_lock_exclusive(&run_lock)
            .context("another live test run is using the account; wait for it to finish")?;
        let binary = bin_directory.join(format!("joog{}", std::env::consts::EXE_SUFFIX));
        let source = fs::read(env!("CARGO_BIN_EXE_joog"))?;
        if fs::read(&binary).ok().as_deref() != Some(source.as_slice()) {
            fs::copy(env!("CARGO_BIN_EXE_joog"), &binary)?;
        }
        let schema: Value = serde_json::from_str(include_str!("../schemas/wrappers.schema.json"))?;
        for name in schema["x-wrappers"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
        {
            let destination = bin_directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
            #[cfg(unix)]
            {
                match fs::symlink_metadata(&destination) {
                    Ok(metadata)
                        if metadata.file_type().is_symlink()
                            && fs::read_link(&destination)? == std::path::Path::new("joog") => {}
                    Ok(_) => bail!("unrelated file at {}", destination.display()),
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        std::os::unix::fs::symlink("joog", &destination)?
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            #[cfg(windows)]
            {
                fs::copy(&binary, &destination)?;
            }
        }
        Ok(Self {
            account,
            directory,
            bin_directory,
            _run_lock: run_lock,
            report: std::env::var_os("JOOG_LIVE_REPORT")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/live-full-report.json")
                }),
            events: Vec::new(),
            pending: Vec::new(),
            last_call: None,
            calls: 0,
            stop: false,
            marker: format!(
                "joog-test-{}-{}",
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                std::process::id()
            ),
            max_commands: std::env::var("JOOG_LIVE_MAX_COMMANDS")
                .ok()
                .map(|value| value.parse())
                .transpose()?
                .unwrap_or(1000)
                .clamp(1, 2000),
            cleaning: false,
            trace_count: 0,
            scenario_failed: false,
            last_remote_reason: None,
            binary_sha256: format!("{:x}", sha2::Sha256::digest(&source)),
            halt_after_cleanup: false,
            command_schemas: std::collections::BTreeMap::new(),
            manual_cleanup: Vec::new(),
        })
    }

    fn process(&mut self, command: &mut Command) -> Result<Output> {
        ensure!(
            !self.scenario_failed || self.cleaning,
            "remaining dependent checks stopped after the first unexpected service failure"
        );
        ensure!(
            !self.stop,
            "live checks stopped after an authentication, quota, or connectivity failure"
        );
        ensure!(
            self.calls < self.max_commands + if self.cleaning { 32 } else { 0 },
            "live command budget exhausted"
        );
        if let Some(previous) = self.last_call {
            thread::sleep(Duration::from_millis(750).saturating_sub(previous.elapsed()));
        }
        self.calls += 1;
        self.last_remote_reason = None;
        let trace = self.directory.path().join("commands.jsonl");
        command.env("JOOG_LIVE_COMMAND_REPORT", &trace);
        let errors = std::env::var_os("JOOG_LIVE_ERROR_REPORT")
            .map(PathBuf::from)
            .unwrap_or_else(|| self.directory.path().join("errors.jsonl"));
        let previous_errors = fs::metadata(&errors)
            .map(|metadata| metadata.len() as usize)
            .unwrap_or(0);
        command.env("JOOG_LIVE_ERROR_REPORT", &errors);
        let (output, expired) = bounded_process(command, Duration::from_secs(45))?;
        if expired {
            self.halt_after_cleanup = true;
            bail!(
                "live command exceeded its 45-second process limit; the child was stopped without retrying"
            );
        }
        if let Ok(bytes) = fs::read(&errors)
            && let Some(new) = bytes.get(previous_errors..)
        {
            for record in serde_json::Deserializer::from_slice(new)
                .into_iter::<Value>()
                .flatten()
            {
                self.last_remote_reason = record
                    .pointer("/googleError/error/errors/0/reason")
                    .or_else(|| record.pointer("/googleError/error/status"))
                    .and_then(Value::as_str)
                    .filter(|reason| {
                        reason.len() <= 100
                            && reason
                                .chars()
                                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.'))
                    })
                    .map(str::to_owned);
            }
        }
        self.last_call = Some(Instant::now());
        if let Ok(bytes) = fs::read(&trace) {
            let records = serde_json::Deserializer::from_slice(&bytes)
                .into_iter::<Value>()
                .collect::<std::result::Result<Vec<_>, _>>()?;
            self.events
                .extend(records.iter().skip(self.trace_count).cloned());
            self.trace_count = records.len();
        }
        if !output.status.success() {
            let error = serde_json::from_slice::<Value>(&output.stderr)
                .ok()
                .and_then(|value| value["error"].as_str().map(str::to_owned))
                .unwrap_or_else(|| String::from_utf8_lossy(&output.stderr).trim().to_owned());
            if [
                "HTTP 401",
                "HTTP 429",
                "HTTP 500",
                "HTTP 502",
                "HTTP 503",
                "HTTP 504",
                "timed out",
                "error sending request",
                "keychain",
                "master key",
                "credential vault",
            ]
            .iter()
            .any(|needle| error.contains(needle))
            {
                self.stop = true;
            }
            bail!("{error}");
        }
        ensure!(
            output.stderr.is_empty(),
            "successful command wrote an unexpected diagnostic"
        );
        Ok(output)
    }

    fn api(&mut self, service: &str, action: &str, mut params: Value) -> Result<Value> {
        if !self.command_schemas.contains_key(service) {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("schemas/{service}.schema.json"));
            self.command_schemas
                .insert(service.into(), serde_json::from_slice(&fs::read(path)?)?);
        }
        bound_pages(&self.command_schemas[service]["$defs"][action], &mut params);
        let file = tempfile::NamedTempFile::new()?;
        serde_json::to_writer(file.as_file(), &params)?;
        let mut command = Command::new(
            self.bin_directory
                .join(format!("joog{}", std::env::consts::EXE_SUFFIX)),
        );
        command
            .args([
                "--json",
                "--timeout",
                "15",
                service,
                &self.account,
                action,
                "--file",
            ])
            .arg(file.path());
        let result = self
            .process(&mut command)
            .and_then(|output| decode_pages(&output.stdout));
        self.scenario_failed |= result.is_err() && !self.cleaning;
        self.events.push(json!({"kind":"command","interface":"joog","service":service,"command":action,"status":if result.is_ok(){"passed"}else{"failed"},"error":result.as_ref().err().map(ToString::to_string),"remoteReason":self.last_remote_reason}));
        self.save()?;
        result
    }

    fn wrapper(&mut self, name: &str, words: &[&str]) -> Result<Vec<u8>> {
        let mut command = Command::new(
            self.bin_directory
                .join(format!("{name}{}", std::env::consts::EXE_SUFFIX)),
        );
        command.arg(&self.account).args(words).arg("timeout:15");
        command.arg("json:true");
        let result = self.process(&mut command).map(|output| output.stdout);
        self.scenario_failed |= result.is_err() && !self.cleaning;
        self.events.push(json!({"kind":"command","interface":name,"command":words.first().copied().unwrap_or("default"),"status":if result.is_ok(){"passed"}else{"failed"},"error":result.as_ref().err().map(ToString::to_string)}));
        self.save()?;
        result
    }

    fn json(&mut self, name: &str, words: &[&str]) -> Result<Value> {
        let bytes = self.wrapper(name, words)?;
        let parsed = serde_json::from_slice(&bytes).context("wrapper returned invalid JSON");
        if parsed.is_err() {
            self.scenario_failed = true;
            if let Some(event) = self.events.last_mut() {
                event["status"] = json!("failed");
                event["error"] = json!("wrapper returned invalid JSON");
            }
            self.save()?;
        }
        parsed
    }

    fn register(
        &mut self,
        service: &'static str,
        command: &'static str,
        params: Value,
        id: &str,
    ) -> Result<()> {
        self.pending.push(Cleanup {
            service,
            command,
            params,
            id: id.into(),
            verification: None,
        });
        self.save()
    }

    fn save(&self) -> Result<()> {
        let mut file = tempfile::NamedTempFile::new_in(
            self.report
                .parent()
                .context("report needs a parent directory")?,
        )?;
        serde_json::to_writer_pretty(
            &mut file,
            &json!({
                "account":self.account,"run":self.marker,"binarySha256":self.binary_sha256,"commands":self.calls,"stopped":self.stop,"checks":self.events,"manualCleanup":self.manual_cleanup,
                "pendingCleanup":self.pending.iter().map(|item| json!({"service":item.service,"command":item.command,"parameters":item.params,"id":item.id,"verification":item.verification})).collect::<Vec<_>>()
            }),
        )?;
        file.write_all(b"\n")?;
        file.as_file().sync_all()?;
        file.persist(&self.report)?;
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        self.cleaning = true;
        for index in (0..self.pending.len()).rev() {
            let item = self.pending[index].clone();
            if self.stop {
                break;
            }
            if item.command == "channel-expiration" {
                let expiration = item.params["expiration"]
                    .as_u64()
                    .context("watch expiration missing")?;
                let now = u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())?;
                let wait = expiration.saturating_sub(now);
                ensure!(
                    wait <= 30_000,
                    "watch expiry exceeds the live cleanup wait limit"
                );
                thread::sleep(Duration::from_millis(wait + 250));
                self.events.push(json!({"cleanup":item.service,"command":"channel-expiration","id":item.id,"status":"passed","expiration":expiration}));
                self.pending.remove(index);
                self.save()?;
                continue;
            }
            let mut result = self.api(item.service, item.command, item.params.clone());
            if result.is_ok()
                && let Some(verification) = &item.verification
            {
                result = self.verify_cleanup(item.service, verification);
            }
            let already_absent = item.verification.is_none()
                && (item.command.ends_with(".delete")
                    || item.command.ends_with(".delete-contact")
                    || item.command == "channels.stop")
                && result
                    .as_ref()
                    .err()
                    .is_some_and(|error| error.to_string().contains("HTTP 404"));
            if already_absent && let Some(event) = self.events.last_mut() {
                event["status"] = json!("expected-absence");
            }
            let failed = result.is_err() && !already_absent;
            self.events.push(json!({"cleanup":item.service,"id":item.id,"command":item.command,"parameters":item.params,"status":if failed{"failed"}else{"passed"},"alreadyAbsent":already_absent,"error":if failed{result.err().map(|error|error.to_string())}else{None}}));
            if failed {
                self.stop = true;
            } else {
                self.pending.remove(index);
            }
            // Failed cleanups remain in the report, but are never retried.
        }
        self.cleaning = false;
        self.stop |= self.halt_after_cleanup;
        self.save()
    }

    fn verify_cleanup(&mut self, service: &str, verification: &Value) -> Result<Value> {
        let value = self.api(
            service,
            verification["command"]
                .as_str()
                .context("cleanup verification command missing")?,
            verification["parameters"].clone(),
        )?;
        if verification["kind"] == "banner-content" {
            ensure!(
                value.pointer("/items/0/id") == Some(&verification["channelId"])
                    && value.pointer("/items/0/brandingSettings/channel")
                        == Some(&verification["channel"]),
                "restoring the banner changed other channel settings"
            );
            let image = value
                .pointer("/items/0/brandingSettings/image/bannerExternalUrl")
                .and_then(Value::as_str)
                .context("restored banner URL missing")?;
            let digest = format!("{:x}", sha2::Sha256::digest(banner_image(image)?));
            ensure!(
                verification["imageSha256"] == digest,
                "restored banner content differs from the saved original"
            );
        } else {
            ensure!(
                value.pointer(
                    verification["pointer"]
                        .as_str()
                        .context("cleanup verification pointer missing")?
                ) == Some(&verification["expected"]),
                "cleanup postcondition failed; original snapshot remains in the pending ledger"
            );
        }
        Ok(value)
    }
}

#[test]
#[ignore = "creates isolated Google test resources and records exact command coverage"]
fn full_account_coverage() -> Result<()> {
    exhaustive::run()
}

#[test]
#[ignore = "deletes only the registered temporary resources in JOOG_LIVE_CLEANUP_REPORT"]
fn cleanup_recorded_resources() -> Result<()> {
    recovery::run()
}

#[test]
#[ignore = "reads JOOG_LIVE_REPORTS without making Google requests"]
fn recorded_command_coverage() -> Result<()> {
    exhaustive::audit()
}

fn id(value: &Value, field: &str) -> Result<String> {
    value[field]
        .as_str()
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .with_context(|| {
            format!("Google did not return {field}; inspect the test resource before rerunning")
        })
}

fn banner_image(address: &str) -> Result<Vec<u8>> {
    let mut address = url::Url::parse(address)?;
    ensure!(
        address.scheme() == "https"
            && address
                .host_str()
                .is_some_and(|host| host.ends_with(".googleusercontent.com"))
            && address.port().is_none()
            && address.username().is_empty()
            && address.password().is_none()
            && address.query().is_none()
            && address.fragment().is_none(),
        "unexpected banner image URL"
    );
    let path = format!(
        "{}=s0",
        address
            .path()
            .split('=')
            .next()
            .context("banner path missing")?
    );
    address.set_path(&path);
    let response = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .no_proxy()
        .build()?
        .get(address)
        .send()
        .map_err(|error| error.without_url())?
        .error_for_status()
        .map_err(|error| error.without_url())?;
    let mut bytes = Vec::new();
    response.take(6 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= 6 * 1024 * 1024
            && (bytes.starts_with(b"\x89PNG\r\n\x1a\n") || bytes.starts_with(b"\xff\xd8\xff")),
        "banner backup is not a supported image of at most 6 MiB"
    );
    Ok(bytes)
}

#[test]
fn banner_backups_reject_untrusted_urls_before_connecting() {
    for address in [
        "http://yt3.googleusercontent.com/image",
        "https://googleusercontent.com.attacker.test/image",
        "https://yt3.googleusercontent.com:444/image",
        "https://user:password@yt3.googleusercontent.com/image",
        "https://yt3.googleusercontent.com/image?token=private",
        "https://yt3.googleusercontent.com/image#fragment",
    ] {
        assert!(banner_image(address).is_err(), "{address}");
    }
}

#[test]
fn coverage_audit_keeps_manual_cleanup_and_historical_failures() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.json");
    let output = directory.path().join("coverage.json");
    let cleanup = json!({"service":"photos","kind":"album","id":"fixture-album","url":"https://photos.google.com/fixture","status":"pending-user-cleanup"});
    fs::write(&source,serde_json::to_vec(&json!({"account":"test@example.com","run":"fixture","checks":[{"kind":"scenario","status":"failed"}],"pendingCleanup":[],"manualCleanup":[cleanup]})).unwrap()).unwrap();
    let status = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "recorded_command_coverage", "--ignored"])
        .env("JOOG_LIVE_REPORTS", &source)
        .env("JOOG_LIVE_REPORT", &output)
        .output()
        .unwrap();
    assert!(
        status.status.success(),
        "{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let report: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(report["manualCleanup"], json!([cleanup]));
    assert_eq!(report["sources"][0]["failedChecks"], 1);
    assert_eq!(report["coverage"]["passed"], 0);
    assert_eq!(report["coverage"]["status"], "incomplete");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(output).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}

fn verify_account(run: &mut Live) -> Result<()> {
    let profile = run.api("mail", "profile", json!({}))?;
    if !profile["emailAddress"]
        .as_str()
        .is_some_and(|email| email.eq_ignore_ascii_case(&run.account))
    {
        run.stop = true;
        bail!("mail profile does not match the approved test account");
    }
    Ok(())
}
