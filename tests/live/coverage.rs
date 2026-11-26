use super::*;

pub(super) const SERVICES: &[&str] = &[
    "mail", "drive", "calendar", "sheets", "tasks", "contacts", "forms", "docs", "slides",
    "youtube", "photos",
];

pub(super) fn selection(
    value: Option<&str>,
    case: Option<&str>,
) -> Result<Option<BTreeSet<String>>> {
    let selected = value
        .map(|value| {
            let names = value.split(',').map(str::trim).collect::<BTreeSet<_>>();
            ensure!(
                names.iter().all(|name| SERVICES.contains(name)),
                "JOOG_LIVE_SERVICES must contain known, nonempty service names"
            );
            Ok(names
                .into_iter()
                .map(str::to_owned)
                .collect::<BTreeSet<_>>())
        })
        .transpose()?;
    if let Some(case) = case {
        let service = match case {
            "csv" => "sheets",
            "dropdown" => "docs",
            "docs-import" => "docs",
            "slides-media" => "slides",
            "mail-draft" => "mail",
            "youtube-remaining" => "youtube",
            "youtube-capabilities" => "youtube",
            "youtube-image-update" => "youtube",
            "youtube-channel" => "youtube",
            "youtube-banner" => "youtube",
            "youtube-watermark" => "youtube",
            "youtube-sections" => "youtube",
            "drive-shared" => "drive",
            "contacts-other" => "contacts",
            "calendar-owner" => "calendar",
            "calendar-subscription" => "calendar",
            "drive-watch" => "drive",
            "calendar-watch" => "calendar",
            _ => bail!(
                "unknown JOOG_LIVE_CASE; see docs/testing.md for supported service/case pairs"
            ),
        };
        ensure!(
            selected
                .as_ref()
                .is_some_and(|names| names.len() == 1 && names.contains(service)),
            "JOOG_LIVE_CASE={case} requires JOOG_LIVE_SERVICES={service}"
        );
    }
    Ok(selected)
}

pub(super) fn summarize(events: &[Value], selected: Option<&BTreeSet<String>>) -> Result<Value> {
    let mut expected: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let native: Value = serde_json::from_str(include_str!("../data/native-requests.json"))?;
    for service in SERVICES {
        if selected.is_some_and(|names| !names.contains(*service)) {
            continue;
        }
        let catalog: Value =
            serde_json::from_slice(&fs::read(root.join(format!("api/{service}.json")))?)?;
        let names = expected.entry(format!("joog.{service}")).or_default();
        names.extend(
            catalog["$defs"]
                .as_object()
                .context("missing API commands")?
                .keys()
                .cloned(),
        );
        names.extend(
            native[service]["methods"]
                .as_object()
                .context("missing native commands")?
                .keys()
                .cloned(),
        );
    }
    let wrappers: Value = serde_json::from_str(include_str!("../../schemas/wrappers.schema.json"))?;
    for name in wrappers["$defs"]
        .as_object()
        .context("missing wrapper commands")?
        .keys()
    {
        if let Some((wrapper, command)) = name.split_once('.') {
            let service = wrappers["x-wrappers"][wrapper]["service"]
                .as_str()
                .context("missing wrapper service")?;
            if selected.is_none_or(|names| names.contains(service)) {
                expected
                    .entry(wrapper.into())
                    .or_default()
                    .insert(command.into());
            }
        }
    }
    let mut passed: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut rejected: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for event in events {
        if !matches!(event["kind"].as_str(), Some("command" | "wire-command")) {
            continue;
        }
        let Some(interface) = event["interface"].as_str() else {
            continue;
        };
        let scope = if interface == "joog" {
            format!("joog.{}", event["service"].as_str().unwrap_or(""))
        } else {
            interface.into()
        };
        let Some(command) = event["command"].as_str() else {
            continue;
        };
        let target = match event["status"].as_str() {
            Some("passed") => &mut passed,
            Some("expected-rejection" | "expected-absence") => &mut rejected,
            _ => continue,
        };
        target.entry(scope).or_default().insert(command.into());
    }
    let mut missing = BTreeMap::new();
    let mut total = 0;
    let mut covered = 0;
    for (scope, commands) in &expected {
        let empty = BTreeSet::new();
        let successes = passed.get(scope).unwrap_or(&empty);
        total += commands.len();
        covered += commands.intersection(successes).count();
        let gaps = commands.difference(successes).cloned().collect::<Vec<_>>();
        if !gaps.is_empty() {
            missing.insert(scope, gaps);
        }
    }
    Ok(
        json!({"kind":"coverage","status":if missing.is_empty(){"passed"}else{"incomplete"},"expected":total,"passed":covered,"commands":passed,"expectedRejections":rejected,"missing":missing}),
    )
}

pub(super) fn audit() -> Result<()> {
    let paths = std::env::var_os("JOOG_LIVE_REPORTS").context("set JOOG_LIVE_REPORTS to the reports to combine, separated by the platform's PATH separator")?;
    let paths = std::env::split_paths(&paths).collect::<Vec<_>>();
    ensure!(
        !paths.is_empty() && paths.len() <= 100,
        "provide between one and 100 live reports"
    );
    let mut events = Vec::new();
    let mut sources = Vec::new();
    let mut account = None;
    let mut pending = Vec::new();
    let mut manual = Vec::new();
    for path in paths {
        let report: Value = serde_json::from_slice(
            &fs::read(&path).with_context(|| format!("cannot read {}", path.display()))?,
        )?;
        let owner = report["account"]
            .as_str()
            .context("report has no account")?;
        if let Some(account) = &account {
            ensure!(account == owner, "reports belong to different accounts");
        } else {
            account = Some(owner.to_owned());
        }
        let checks = report["checks"]
            .as_array()
            .context("report has no checks")?;
        sources.push(json!({"path":path,"run":report["run"],"binarySha256":report["binarySha256"],"failedChecks":checks.iter().filter(|event|event["status"]=="failed").count()}));
        pending.extend(
            report["pendingCleanup"]
                .as_array()
                .context("report has no cleanup ledger")?
                .iter()
                .cloned(),
        );
        manual.extend(
            report["manualCleanup"]
                .as_array()
                .into_iter()
                .flatten()
                .cloned(),
        );
        events.extend(checks.iter().cloned());
    }
    let coverage = summarize(&events, None)?;
    let destination = std::env::var_os("JOOG_LIVE_REPORT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/live-coverage.json")
        });
    ensure!(
        sources
            .iter()
            .all(|source| source["path"].as_str() != destination.to_str()),
        "coverage output must not overwrite a source report"
    );
    let mut file = tempfile::NamedTempFile::new_in(
        destination
            .parent()
            .context("report needs a parent directory")?,
    )?;
    serde_json::to_writer_pretty(
        &mut file,
        &json!({"account":account,"sources":sources,"coverage":coverage,"pendingCleanup":pending,"manualCleanup":manual,"interpretation":"Successful command executions across the supplied reports. Historical failures remain listed per source. This audit does not certify every scenario or a single current binary."}),
    )?;
    file.write_all(b"\n")?;
    file.as_file().sync_all()?;
    file.persist(&destination)?;
    writeln!(
        std::io::stdout().lock(),
        "Successful live commands: {}/{}. Report: {}",
        coverage["passed"],
        coverage["expected"],
        destination.display()
    )?;
    Ok(())
}

#[test]
fn missing_and_rejected_commands_cannot_make_live_coverage_pass() {
    let report = summarize(&[
        json!({"kind":"command","interface":"joog","service":"mail","command":"profile","status":"passed"}),
        json!({"kind":"wire-command","interface":"joog","service":"mail","command":"profile","status":"passed"}),
        json!({"kind":"command","interface":"joog","service":"mail","command":"messages.send","status":"expected-rejection"}),
    ], None).unwrap();
    assert_eq!(report["expected"], 435);
    assert_eq!(report["passed"], 1);
    assert_eq!(report["status"], "incomplete");
    assert!(
        report["missing"]["joog.mail"]
            .as_array()
            .unwrap()
            .contains(&json!("messages.send"))
    );
    let selected = selection(Some("sheets"), Some("csv")).unwrap();
    assert_eq!(summarize(&[], selected.as_ref()).unwrap()["expected"], 21);
}

#[test]
fn invalid_selection_fails_before_account_access() {
    for (value, case) in [
        (Some(""), None),
        (Some("mail,"), None),
        (Some("typo"), None),
        (None, Some("csv")),
        (Some("docs"), Some("csv")),
        (Some("mail"), Some("typo")),
    ] {
        assert!(selection(value, case).is_err());
    }
}
