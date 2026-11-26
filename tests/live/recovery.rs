use super::*;

fn entries(report: &Value, account: &str) -> Result<Vec<Cleanup>> {
    ensure!(
        report["account"].as_str() == Some(account),
        "cleanup report belongs to another account"
    );
    let pending = report["pendingCleanup"]
        .as_array()
        .context("cleanup report has no resource ledger")?;
    ensure!(
        pending.len() <= 32,
        "cleanup report exceeds the per-run resource limit"
    );
    let mut result = Vec::new();
    for entry in pending {
        let (service, command, key) = match (entry["service"].as_str(), entry["command"].as_str()) {
            (Some("drive"), Some("files.delete")) => ("drive", "files.delete", "fileId"),
            (Some("drive"), Some("files.update")) => ("drive", "files.update", "fileId"),
            (Some("drive"), Some("drives.delete")) => ("drive", "drives.delete", "driveId"),
            (Some("mail"), Some("messages.delete")) => ("mail", "messages.delete", "id"),
            (Some("mail"), Some("drafts.delete")) => ("mail", "drafts.delete", "id"),
            (Some("mail"), Some("labels.delete")) => ("mail", "labels.delete", "id"),
            (Some("calendar"), Some("calendars.delete")) => {
                ("calendar", "calendars.delete", "calendarId")
            }
            (Some("calendar"), Some("calendar-list.delete")) => {
                ("calendar", "calendar-list.delete", "calendarId")
            }
            (Some("calendar"), Some("channel-expiration")) => {
                ("calendar", "channel-expiration", "id")
            }
            (Some("drive"), Some("channels.stop")) => ("drive", "channels.stop", "id"),
            (Some("calendar"), Some("events.delete")) => ("calendar", "events.delete", "eventId"),
            (Some("contacts"), Some("people.delete-contact")) => {
                ("contacts", "people.delete-contact", "resourceName")
            }
            (Some("contacts"), Some("contact-groups.delete")) => {
                ("contacts", "contact-groups.delete", "resourceName")
            }
            (Some("tasks"), Some("tasklists.delete")) => ("tasks", "tasklists.delete", "tasklist"),
            (Some("youtube"), Some("playlists.delete")) => ("youtube", "playlists.delete", "id"),
            (Some("youtube"), Some("videos.delete")) => ("youtube", "videos.delete", "id"),
            (Some("youtube"), Some("channels.update")) => ("youtube", "channels.update", "id"),
            (Some("youtube"), Some("channel-banners.insert")) => {
                ("youtube", "channel-banners.insert", "upload")
            }
            (Some("youtube"), Some("watermarks.unset")) => {
                ("youtube", "watermarks.unset", "channelId")
            }
            (Some("youtube"), Some("captions.delete")) => ("youtube", "captions.delete", "id"),
            (Some("youtube"), Some("comments.delete")) => ("youtube", "comments.delete", "id"),
            (Some("youtube"), Some("subscriptions.delete")) => {
                ("youtube", "subscriptions.delete", "id")
            }
            (Some("youtube"), Some("playlist-images.delete")) => {
                ("youtube", "playlist-images.delete", "id")
            }
            (Some("youtube"), Some("channel-sections.delete")) => {
                ("youtube", "channel-sections.delete", "id")
            }
            _ => bail!("unsupported cleanup command in report"),
        };
        let id = entry["id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .context("cleanup resource ID missing")?;
        ensure!(
            entry["parameters"][key].as_str() == Some(id),
            "cleanup resource ID does not match its parameters"
        );
        if service == "drive" && command == "files.update" {
            ensure!(
                entry["parameters"]["trashed"] == true,
                "Drive recovery can only move fixtures to Trash"
            );
        }
        let verification = entry.get("verification").filter(|value| !value.is_null());
        if let Some(verification) = verification {
            ensure!(
                service == "youtube"
                    && matches!(command, "channels.update" | "channel-banners.insert")
                    && verification["command"] == "channels.list"
                    && verification["parameters"]
                        == json!({"mine":true,"part":"brandingSettings","fields":"items(id,brandingSettings)","max":1}),
                "unsupported cleanup verification"
            );
            if command == "channel-banners.insert" {
                ensure!(
                    verification["kind"] == "banner-content"
                        && verification["channelId"]
                            .as_str()
                            .is_some_and(|id| !id.is_empty())
                        && verification["channel"].is_object(),
                    "banner restoration metadata missing"
                );
                ensure!(
                    fs::metadata(id)?.len() <= 6 * 1024 * 1024,
                    "banner backup exceeds 6 MiB"
                );
                let digest = format!("{:x}", sha2::Sha256::digest(fs::read(id)?));
                ensure!(
                    verification["imageSha256"] == digest,
                    "banner backup differs from its recorded hash"
                );
            } else {
                ensure!(
                    verification["pointer"] == "/items/0/brandingSettings"
                        && verification["expected"] == entry["parameters"]["brandingSettings"],
                    "channel restoration verification does not match its snapshot"
                );
            }
        }
        ensure!(
            !matches!(command, "channels.update" | "channel-banners.insert")
                || verification.is_some(),
            "restoration requires a verification snapshot"
        );
        result.push(Cleanup {
            service,
            command,
            params: entry["parameters"].clone(),
            id: id.into(),
            verification: verification.cloned(),
        });
    }
    Ok(result)
}

pub(super) fn run() -> Result<()> {
    let source = PathBuf::from(
        std::env::var_os("JOOG_LIVE_CLEANUP_REPORT")
            .context("set JOOG_LIVE_CLEANUP_REPORT to the interrupted run's report")?,
    );
    let report: Value = serde_json::from_slice(&fs::read(&source)?)?;
    let account = std::env::var("JOOG_LIVE_ACCOUNT")
        .context("set JOOG_LIVE_ACCOUNT to the approved test account")?;
    let pending = entries(&report, &account)?;
    ensure!(!pending.is_empty(), "report has no pending cleanup");
    let mut run = Live::new()?;
    if std::env::var_os("JOOG_LIVE_REPORT").is_none() {
        run.report =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/live-cleanup-report.json");
    }
    ensure!(
        run.report != source,
        "write recovery results to a separate report to preserve the original failure"
    );
    verify_account(&mut run)?;
    run.pending = pending;
    run.events
        .push(json!({"kind":"recovery","source":source,"status":"started"}));
    run.save()?;
    run.cleanup()?;
    ensure!(
        run.pending.is_empty(),
        "cleanup failed; inspect {} before making any further requests",
        run.report.display()
    );
    Ok(())
}

#[test]
fn recovery_rejects_another_account_and_unrelated_commands_before_network_access() {
    let valid = json!({"account":"test@example.com","pendingCleanup":[{"service":"drive","command":"files.delete","id":"fixture","parameters":{"fileId":"fixture"}}]});
    assert_eq!(entries(&valid, "test@example.com").unwrap().len(), 1);
    assert!(entries(&valid, "other@example.com").is_err());
    for (field, value) in [
        ("command", json!("files.empty-trash")),
        ("id", json!("different")),
        ("parameters", json!({"fileId":"different"})),
    ] {
        let mut invalid = valid.clone();
        invalid["pendingCleanup"][0][field] = value;
        assert!(entries(&invalid, "test@example.com").is_err());
    }
}

#[test]
fn recovery_checks_restoration_snapshots_and_backup_bytes_before_network_access() {
    let query = json!({"mine":true,"part":"brandingSettings","fields":"items(id,brandingSettings)","max":1});
    let branding = json!({"channel":{"title":"Fixture"}});
    let entry = json!({"service":"youtube","command":"channels.update","id":"fixture","parameters":{"id":"fixture","part":"brandingSettings","brandingSettings":branding},"verification":{"command":"channels.list","parameters":query,"pointer":"/items/0/brandingSettings","expected":branding}});
    let valid = json!({"account":"test@example.com","pendingCleanup":[entry]});
    assert_eq!(entries(&valid, "test@example.com").unwrap().len(), 1);
    for (field, value) in [
        ("command", json!("videos.delete")),
        ("parameters", json!({"mine":true})),
        ("expected", json!({"channel":{"title":"Another value"}})),
    ] {
        let mut invalid = valid.clone();
        invalid["pendingCleanup"][0]["verification"][field] = value;
        assert!(entries(&invalid, "test@example.com").is_err());
    }
    let mut invalid = valid;
    invalid["pendingCleanup"][0]
        .as_object_mut()
        .unwrap()
        .remove("verification");
    assert!(entries(&invalid, "test@example.com").is_err());
    let mut backup = tempfile::NamedTempFile::new().unwrap();
    let image = include_bytes!("../data/live-fixture.png");
    backup.write_all(image).unwrap();
    let id = backup.path().to_str().unwrap();
    let report = json!({"account":"test@example.com","pendingCleanup":[{"service":"youtube","command":"channel-banners.insert","id":id,"parameters":{"upload":id,"contentType":"image/png"},"verification":{"kind":"banner-content","command":"channels.list","parameters":query,"channelId":"fixture","channel":{"title":"Fixture"},"imageSha256":format!("{:x}",sha2::Sha256::digest(image))}}]});
    assert_eq!(entries(&report, "test@example.com").unwrap().len(), 1);
    backup.write_all(b"changed").unwrap();
    assert!(entries(&report, "test@example.com").is_err());
}
