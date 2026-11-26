use super::*;
use backend::Backend;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

#[derive(Default)]
struct Recording {
    calls: Vec<(String, String, Value)>,
}

impl Backend for Recording {
    fn call(&mut self, service: &str, command: &str, params: Value) -> Result<Value> {
        // Resolve through the actual Joog parser and API contract. This catches
        // wire names, required inputs, invalid enums, and incompatible bodies.
        let file = tempfile::NamedTempFile::new()?;
        serde_json::to_writer(file.as_file(), &params)?;
        let parsed = crate::cli::parse_from([
            "joog",
            service,
            "me@example.com",
            command,
            "--file",
            file.path().to_str().unwrap(),
        ])?;
        let context = parsed.subcommand().unwrap().1;
        let spec = crate::services::SERVICES
            .iter()
            .find(|spec| spec.cli == service)
            .unwrap();
        crate::service_call(spec, context)
            .with_context(|| format!("{service} {command} {params}"))?;
        self.calls
            .push((service.into(), command.into(), params.clone()));
        Ok(match (service, command) {
            ("calendar", "calendars.get") => json!({"id":"primary","timeZone":"Europe/Paris"}),
            ("calendar", "events.get") => {
                json!({"id":"event","start":{"dateTime":"2026-12-01T10:00:00+01:00"},"end":{"dateTime":"2026-12-01T11:00:00+01:00"}})
            }
            ("calendar", "freebusy.query") => {
                let calendars = params
                    .pointer("/body/items")
                    .and_then(Value::as_array)
                    .unwrap()
                    .iter()
                    .map(|item| (item["id"].as_str().unwrap().to_owned(), json!({"busy":[]})))
                    .collect::<Map<_, _>>();
                json!({"calendars":calendars})
            }
            ("drive", "files.get") => {
                json!({"id":"resource","mimeType":"application/octet-stream","parents":["old-folder"]})
            }
            ("drive", "files.copy") => json!({"id":"copied"}),
            ("docs", "create") => json!({"documentId":"document"}),
            ("docs", "get") => {
                json!({"documentId":"document","tabs":[{"tabProperties":{"tabId":"first"},"documentTab":{"body":{"content":[{"paragraph":{"elements":[{"textRun":{"content":"Hello\n"}}]}}]}}}]})
            }
            ("sheets", "get") => {
                json!({"spreadsheetId":"sheet","sheets":[{"properties":{"sheetId":0,"title":"First tab"}}]})
            }
            ("sheets", "values.get") => json!({"values":[["Name","Value"],["Lara","42"]]}),
            ("slides", "create") => json!({"presentationId":"presentation"}),
            ("slides", "get") => {
                json!({"presentationId":"presentation","revisionId":"revision","slides":[{"objectId":"slide1","pageElements":[{"objectId":"title1","shape":{"placeholder":{"type":"TITLE"}}},{"objectId":"body1","shape":{"placeholder":{"type":"BODY"}}}]}]})
            }
            ("forms", "create") => {
                json!({"formId":"form","responderUri":"https://docs.google.com/forms/d/e/example/viewform"})
            }
            ("forms", "get") => {
                json!({"formId":"form","revisionId":"revision","responderUri":"https://docs.google.com/forms/d/e/example/viewform","publishSettings":{"publishState":{"isPublished":true,"isAcceptingResponses":true}},"items":[]})
            }
            ("contacts", "people.get") => {
                json!({"resourceName":"people/contact","etag":"etag","metadata":{"sources":[{"id":"contact","type":"CONTACT","etag":"source-etag"}]},"names":[{"displayName":"Lara"}],"phoneNumbers":[{"value":"123"}]})
            }
            ("mail", "messages.list") => json!({"messages":[{"id":"message","threadId":"thread"}]}),
            ("mail", "drafts.list") => {
                json!({"drafts":[{"id":"draft","message":{"id":"message","threadId":"thread"}}]})
            }
            ("mail", "drafts.get") => {
                json!({"id":"draft","message":{"id":"message","threadId":"thread","snippet":"Draft text","payload":{"headers":[{"name":"From","value":"me@example.com"},{"name":"Subject","value":"Draft subject"}]}}})
            }
            ("mail", "messages.get") => {
                json!({"id":"message","threadId":"thread","raw":URL_SAFE_NO_PAD.encode(b"From: \"Lara, Martin\" <lara@example.com>\r\nTo: me@example.com, alex@example.com\r\nCc: other@example.com\r\nSubject: Hello\r\nMessage-ID: <original@example.com>\r\nReferences: <older@example.com>\r\nContent-Type: text/plain\r\n\r\nOriginal body\r\n"),"payload":{"mimeType":"text/plain","headers":[{"name":"From","value":"lara@example.com"},{"name":"To","value":"me@example.com"},{"name":"Subject","value":"Hello"},{"name":"Date","value":"Wed, 25 Nov 2026 10:00:00 +0100"}],"body":{"data":URL_SAFE_NO_PAD.encode("Hello")}}})
            }
            ("photos", "media-items.get") => {
                json!({"id":"photo","baseUrl":"https://lh3.googleusercontent.com/example","mimeType":"image/jpeg"})
            }
            ("photos", "media-items.batch-create") => {
                json!({"newMediaItemResults":[{"status":{},"mediaItem":{"id":"created-photo"}}]})
            }
            ("youtube", "video-categories.list") => {
                json!({"items":[{"id":"27","snippet":{"title":"Education","assignable":true}}]})
            }
            _ => json!({"id":"resource"}),
        })
    }
    fn upload_photo(&mut self, path: &str) -> Result<String> {
        values::format("local-file", path)?;
        Ok("upload-token".into())
    }
    fn download_photo(&mut self, url: &str, path: &str) -> Result<Value> {
        ensure!(url.ends_with("=d"), "wrong photo download URL");
        values::save(b"photo", path)
    }
}

fn sample(shape: &Value, dir: &Path) -> Value {
    if let Some(default) = shape.get("default") {
        return default.clone();
    }
    if let Some(value) = shape.get("const") {
        return value.clone();
    }
    if let Some(choices) = shape["enum"].as_array() {
        return choices[0].clone();
    }
    match shape["type"].as_str() {
        Some("array") => json!([sample(&shape["items"], dir)]),
        Some("boolean") => json!(true),
        Some("integer") => json!(shape["minimum"].as_u64().unwrap_or(1).max(1)),
        _ => json!(match shape["format"].as_str() {
            Some("email") => "lara@example.com".into(),
            Some("local-file") => dir.join("data.csv").to_str().unwrap().into(),
            Some("new-file") => dir.join("saved.csv").to_str().unwrap().into(),
            Some("local-date") => "2026-12-01".into(),
            Some("local-time") => "2026-12-01 10:00".into(),
            Some("duration") => "30m".into(),
            Some("date-time") => "2026-12-01T00:00:00Z".into(),
            Some("iana-time-zone") => "Europe/Paris".into(),
            Some("single-cell") => "B2".into(),
            Some("cell-range") => "A1:B2".into(),
            _ => "example".to_owned(),
        }),
    }
}

fn request(wrapper: &str, words: &[&str]) -> Result<Request> {
    let args = std::iter::once("me@example.com")
        .chain(words.iter().copied())
        .map(OsString::from)
        .collect::<Vec<_>>();
    match parse(wrapper, &args)? {
        Parsed::Request(request) => Ok(request),
        _ => bail!("unexpected help"),
    }
}

fn sample_parameters(schema: &Value, dir: &Path) -> Map<String, Value> {
    let mut params = Map::new();
    for name in schema["required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        params.insert(name.into(), sample(&schema["properties"][name], dir));
    }
    for (name, shape) in schema["properties"].as_object().unwrap() {
        if let Some(default) = shape.get("default") {
            params
                .entry(name.clone())
                .or_insert_with(|| default.clone());
        }
    }
    for rule in schema["allOf"].as_array().into_iter().flatten() {
        if let Some(choices) = rule["anyOf"].as_array()
            && !assertion(rule, &json!(params))
        {
            for name in choices[0]["required"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
            {
                params.insert(name.into(), sample(&schema["properties"][name], dir));
            }
        }
    }
    params
}

#[test]
fn adversarial_files_reject_type_confusion_unknown_keys_and_missing_inputs_for_every_wrapper() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("data.csv"), "Lara,42\n").unwrap();
    let path = dir.path().join("parameters.json");
    let file = format!("file:{}", path.display());
    let mut checked = 0;
    let mut mutations = 0;
    for name in catalog()["$defs"]
        .as_object()
        .unwrap()
        .keys()
        .filter(|name| name.contains('.'))
    {
        let (wrapper, command) = name.split_once('.').unwrap();
        let schema = definition(wrapper, command).unwrap();
        let valid = sample_parameters(&schema, dir.path());
        let parse_file = |params: &Map<String, Value>| {
            std::fs::write(&path, serde_json::to_vec(params).unwrap()).unwrap();
            request(wrapper, &[command, &file])
        };
        assert!(parse_file(&valid).is_ok(), "invalid baseline for {name}");
        for key in schema["properties"].as_object().unwrap().keys() {
            let mut invalid = valid.clone();
            invalid.insert(key.clone(), json!({"injected":"wrong type"}));
            assert!(
                parse_file(&invalid).is_err(),
                "{name} accepted an object for {key}"
            );
            mutations += 1;
        }
        for key in ["__proto__", "constructor", "unknownParameter"] {
            let mut invalid = valid.clone();
            invalid.insert(key.into(), json!({"injected":true}));
            assert!(parse_file(&invalid).is_err(), "{name} accepted {key}");
            mutations += 1;
        }
        for key in schema["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            let mut invalid = valid.clone();
            invalid.remove(key);
            assert!(
                parse_file(&invalid).is_err(),
                "{name} accepted missing {key}"
            );
            mutations += 1;
        }
        checked += 1;
    }
    assert_eq!(checked, 95);
    assert!(mutations > 500);
}

#[test]
fn adversarial_pagination_rejects_broken_json_and_incompatible_response_shapes() {
    assert_eq!(
        backend::merge_pages(
            br#"{"items":[{"id":"a"}],"nextPageToken":"one"}
{"items":[{"id":"b"}]}"#
        )
        .unwrap(),
        json!({"items":[{"id":"a"},{"id":"b"}]})
    );
    for bytes in [
        &b"{\"items\":["[..],
        &b"{}\n[]"[..],
        &b"[]\n{}"[..],
        &b"{\"items\":[]}\n{\"items\":\"invalid\"}"[..],
        &b"{\"items\":\"invalid\"}\n{\"items\":[]}"[..],
    ] {
        assert!(backend::merge_pages(bytes).is_err());
    }
}

#[test]
fn adversarial_limits_dates_headers_and_resource_urls_fail_without_a_backend() {
    for value in [
        "0m",
        "-1m",
        "18446744073709551615h",
        "9223372036854775807d",
        "NaNh",
        "1e9m",
    ] {
        assert!(
            values::duration(value).is_err(),
            "accepted duration {value}"
        );
    }
    for (kind, value) in [
        ("local-date", "2026-02-30"),
        ("local-date", "2026-13-01"),
        ("local-time", "tomorrow 25:61"),
        ("date-time", "2026-12-01T10:00:00"),
        ("single-cell", "A0"),
        ("single-cell", "A18446744073709551616"),
        ("cell-range", "A1:B2:C3"),
        ("email", "user@example.com\r\nBcc:other@example.com"),
        ("email", "Name <user@example.com>"),
        ("single-line", "Subject\nInjected: value"),
    ] {
        assert!(values::format(kind, value).is_err(), "accepted {kind}");
    }
    for kind in ["drive", "docs", "sheets", "slides", "forms"] {
        for value in [
            "https://docs.google.com.attacker.example/document/d/id/edit",
            "https://attacker.example/?id=secret",
            "https://user:password@docs.google.com/document/d/id/edit",
            "http://docs.google.com/document/d/id/edit",
            "https://docs.google.com:444/document/d/id/edit",
            "https://docs.google.com/document/d/%2e%2e%2fsecret/edit",
        ] {
            assert!(
                values::resource(kind, value).is_err(),
                "accepted untrusted {kind} URL"
            );
        }
    }
    for (wrapper, args) in [
        (
            "jmail",
            vec![
                "draft",
                "to:user@example.com",
                "subject:Hello\r\nBcc:other@example.com",
            ],
        ),
        (
            "jcal",
            vec![
                "add",
                "Test",
                "at:tomorrow 10:00",
                "for:18446744073709551615d",
            ],
        ),
        ("jdrive", vec!["ls", "limit:18446744073709551615"]),
        ("jphotos", vec!["ls", "limit:-1"]),
    ] {
        assert!(
            request(wrapper, &args).is_err(),
            "accepted adversarial {wrapper} request"
        );
    }
}

#[test]
fn every_declared_wrapper_command_builds_requests_accepted_by_joog() {
    let mut count = 0;
    let mut failures = Vec::new();
    for (name, _) in catalog()["$defs"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(name, _)| name.contains('.'))
    {
        let (wrapper, command) = name.split_once('.').unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("data.csv"), "Lara,42\nAlex,43\n").unwrap();
        let schema = definition(wrapper, command).unwrap();
        let mut params = Map::new();
        for name in schema["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            params.insert(name.into(), sample(&schema["properties"][name], dir.path()));
        }
        for (name, shape) in schema["properties"].as_object().unwrap() {
            if let Some(default) = shape.get("default") {
                params
                    .entry(name.clone())
                    .or_insert_with(|| default.clone());
            }
        }
        for rule in schema["allOf"].as_array().into_iter().flatten() {
            if let Some(choices) = rule["anyOf"].as_array()
                && !assertion(rule, &json!(params))
            {
                for name in choices[0]["required"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                {
                    params.insert(name.into(), sample(&schema["properties"][name], dir.path()));
                }
            }
        }
        // Exercise actual row parsing and numeric category IDs rather than an
        // unrelated category-list fixture.
        if wrapper == "jsheets" && command == "append" {
            params.insert("row".into(), json!(["Lara,42"]));
        }
        if wrapper == "jyt" && command == "upload" {
            params.insert("category".into(), json!("27"));
        }
        let file = dir.path().join("request.json");
        std::fs::write(&file, serde_json::to_vec(&params).unwrap()).unwrap();
        let mut request = request(wrapper, &[command, &format!("file:{}", file.display())])
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let mut api = Recording::default();
        match tasks::execute(&request, &mut api) {
            Ok(value) => {
                assert!(!api.calls.is_empty(), "{name} made no requests");
                // The same provider responses must yield identical data in
                // both output modes, including for compound commands.
                if let Some(path) = request.text("out")
                    && Path::new(path).exists()
                {
                    std::fs::remove_file(path).unwrap();
                }
                request.params.insert("json".into(), json!(true));
                let mut json_api = Recording::default();
                let json_value = tasks::execute(&request, &mut json_api)
                    .unwrap_or_else(|error| panic!("{name} JSON: {error:#}"));
                assert_eq!(
                    value, json_value,
                    "{name}: output format changed the result data"
                );
                assert_eq!(
                    api.calls
                        .iter()
                        .map(|(service, command, _)| (service, command))
                        .collect::<Vec<_>>(),
                    json_api
                        .calls
                        .iter()
                        .map(|(service, command, _)| (service, command))
                        .collect::<Vec<_>>(),
                    "{name}: output format changed the operations performed"
                );
            }
            Err(error) => failures.push(format!("{name}: {error:#}")),
        }
        count += 1;
    }
    assert_eq!(count, 95);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn mail_formats_share_message_metadata_identifiers_and_requests() {
    for words in [
        vec!["inbox", "limit:10", "unread:true"],
        vec!["find", "Lara"],
        vec!["read", "message"],
        vec!["drafts"],
    ] {
        let mut query = request("jmail", &words).unwrap();
        let mut plain_api = Recording::default();
        let plain = tasks::execute(&query, &mut plain_api).unwrap();
        query.params.insert("json".into(), json!(true));
        let mut json_api = Recording::default();
        let json = tasks::execute(&query, &mut json_api).unwrap();
        assert_eq!(plain, json);
        assert_eq!(plain_api.calls, json_api.calls);
        let row = if plain.is_array() { &plain[0] } else { &plain };
        assert_eq!(row["id"], "message");
        assert_eq!(row["threadId"], "thread");
        assert!(row["from"].is_string());
        assert!(row["subject"].is_string());
        assert!(row["text"].is_string());
        if words[0] == "drafts" {
            assert_eq!(row["draftId"], "draft");
        }
        if words[0] == "inbox" {
            assert_eq!(plain_api.calls[0].2["q"], "in:inbox is:unread");
            assert_eq!(plain_api.calls[0].2["max"], 10);
        }
    }
}

#[test]
fn json_format_keeps_document_tab_validation() {
    let mut query = request("jdocs", &["read", "document", "tab:missing"]).unwrap();
    let plain = tasks::execute(&query, &mut Recording::default()).unwrap_err();
    query.params.insert("json".into(), json!(true));
    let json = tasks::execute(&query, &mut Recording::default()).unwrap_err();
    assert_eq!(plain.to_string(), json.to_string());
    assert!(plain.to_string().contains("tab was not found"));
}

#[test]
fn wrappers_have_their_own_syntax_and_defaults() {
    assert_eq!(request("jmail", &[]).unwrap().command, "inbox");
    let sent = request(
        "jmail",
        &[
            "to:lara@example.com",
            "to:alex@example.com",
            r"body:Hello\nHow are you?",
        ],
    )
    .unwrap();
    assert_eq!(sent.command, "send");
    assert_eq!(
        sent.params["to"],
        json!(["lara@example.com", "alex@example.com"])
    );
    assert_eq!(sent.text("body"), Some("Hello\nHow are you?"));
    let default = request("jyt", &[]).unwrap();
    assert_eq!(default.command, "info");
    assert!(default.text("target").is_none());
    assert!(
        crate::cli::parse_from([
            "joog",
            "mail",
            "me@example.com",
            "send",
            "to:lara@example.com"
        ])
        .is_err()
    );
    assert!(
        crate::cli::parse_from([
            "joog",
            "mail",
            "me@example.com",
            "messages.send",
            "--to",
            "lara@example.com"
        ])
        .is_ok()
    );
    for wrapper in catalog()["x-wrappers"].as_object().unwrap().keys() {
        assert!(matches!(
            parse(wrapper, &["--help".into()]).unwrap(),
            Parsed::Help(_)
        ));
    }
    let common = help_text("jmail", Some("send"), false).unwrap();
    assert!(!common.contains("home:PATH"));
    assert!(
        help_text("jmail", Some("send"), true)
            .unwrap()
            .contains("home:PATH")
    );
    assert!(common.contains("Provide at least one recipient"));
}

#[test]
fn schema_constraints_reject_invalid_wrapper_inputs_before_execution() {
    for (wrapper, words) in [
        ("jmail", vec!["send", "body:Hello"]),
        ("jmail", vec!["send", "to:not-an-email"]),
        ("jmail", vec!["send", "to:lara@example.com", "unknown:yes"]),
        ("jdrive", vec!["move", "file"]),
        (
            "jdrive",
            vec!["share", "file", "to:lara@example.com", "role:owner"],
        ),
        (
            "jcal",
            vec!["add", "Lunch", "at:2026-12-01 10:00", "on:2026-12-01"],
        ),
        (
            "jcal",
            vec!["add", "Lunch", "at:2026-12-01 10:00", "for:0m"],
        ),
        ("jsheets", vec!["clear", "sheet"]),
        ("jsheets", vec!["set", "sheet", "cell:A1:B2", "value:x"]),
        ("jdocs", vec!["append", "doc"]),
        ("jslides", vec!["set", "deck", "slide:0", "title:Hello"]),
        ("jforms", vec!["question", "form", "Title", "type:choice"]),
        (
            "jforms",
            vec!["question", "form", "Title", "type:text", "option:wrong"],
        ),
        ("jtasks", vec!["add", "Task", "due:2026-02-30"]),
        ("jcontacts", vec!["set", "contact"]),
        ("jyt", vec!["new-playlist", "Title", "privacy:friends"]),
        ("jphotos", vec!["find", "album:unsupported"]),
    ] {
        assert!(
            request(wrapper, &words).is_err(),
            "accepted {wrapper} {words:?}"
        );
    }
}

#[test]
fn file_overrides_arrays_text_representations_and_unknown_fields() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("input.json");
    std::fs::write(
        &file,
        r#"{"to":["old@example.com"],"bodyFile":"must-not-be-read.txt"}"#,
    )
    .unwrap();
    let parsed = request(
        "jmail",
        &[
            "send",
            &format!("file:{}", file.display()),
            "to:lara@example.com",
            "body:New",
        ],
    )
    .unwrap();
    assert_eq!(parsed.strings("to"), ["lara@example.com"]);
    assert!(parsed.text("bodyFile").is_none());
    std::fs::write(&file, r#"{"to":["lara@example.com"],"unknown":true}"#).unwrap();
    assert!(request("jmail", &["send", &format!("file:{}", file.display())]).is_err());
}

#[test]
fn replies_keep_thread_headers_and_forwarding_preserves_original_mime() {
    let mut api = Recording::default();
    let reply = request("jmail", &["reply", "message", "body:Thanks", "all:true"]).unwrap();
    tasks::execute(&reply, &mut api).unwrap();
    let sent = &api.calls.last().unwrap().2;
    assert_eq!(sent["threadId"], "thread");
    let bytes = URL_SAFE_NO_PAD
        .decode(sent["raw"].as_str().unwrap())
        .unwrap();
    let parsed = mail_parser::MessageParser::default().parse(&bytes).unwrap();
    assert_eq!(parsed.in_reply_to().as_text(), Some("original@example.com"));
    assert!(parsed.to().unwrap().contains("lara@example.com"));
    assert!(parsed.to().unwrap().contains("alex@example.com"));
    assert!(!parsed.to().unwrap().contains("me@example.com"));
    let forward = request("jmail", &["forward", "message", "to:someone@example.com"]).unwrap();
    tasks::execute(&forward, &mut api).unwrap();
    let bytes = URL_SAFE_NO_PAD
        .decode(api.calls.last().unwrap().2["raw"].as_str().unwrap())
        .unwrap();
    let mime = String::from_utf8(bytes).unwrap();
    assert!(mime.contains("message/rfc822"));
    assert!(mime.contains("forwarded.eml"));
}

#[test]
fn contact_updates_only_replace_named_fields_and_keep_concurrency_metadata() {
    let mut api = Recording::default();
    tasks::execute(
        &request("jcontacts", &["set", "contact", "email:new@example.com"]).unwrap(),
        &mut api,
    )
    .unwrap();
    let params = &api.calls.last().unwrap().2;
    assert_eq!(params["updatePersonFields"], "emailAddresses");
    assert_eq!(
        params.pointer("/body/metadata/sources/0/etag"),
        Some(&json!("source-etag"))
    );
    assert!(params["body"].get("phoneNumbers").is_none());
    assert_eq!(params.pointer("/body/etag"), Some(&json!("etag")));
}

#[test]
fn csv_exports_preserve_ragged_google_rows_and_literal_formulas() {
    let data = vec![
        vec!["one".into(), "quoted, cell".into(), "=1+1".into()],
        vec!["Unicode café".into()],
        vec![],
    ];
    let bytes = values::csv_bytes(&data).unwrap();
    assert_eq!(
        String::from_utf8(bytes).unwrap(),
        "one,\"quoted, cell\",=1+1\nUnicode café,,\n,,\n"
    );
}

#[test]
fn csv_quotes_timezones_and_atomic_output_are_preserved() {
    assert!(values::duration("1é").is_err());
    let rows = values::csv_rows(
        &json!({"row":["\"Lara, Martin\",42","Alex,43"]})
            .as_object()
            .unwrap()
            .clone(),
    )
    .unwrap();
    assert_eq!(rows[0], ["Lara, Martin", "42"]);
    let now = "2026-03-28T12:00:00Z".parse::<jiff::Timestamp>().unwrap();
    let zone = jiff::tz::TimeZone::get("Europe/Paris").unwrap();
    assert!(values::timestamp("2026-03-29 02:30", &zone, now).is_err());
    assert_eq!(
        values::rfc3339(
            values::timestamp("2026-03-29T03:30:00+02:00", &zone, now).unwrap(),
            &zone
        ),
        "2026-03-29T03:30:00+02:00"
    );
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("saved.csv");
    values::save(b"original", path.to_str().unwrap()).unwrap();
    assert!(values::save(b"replacement", path.to_str().unwrap()).is_err());
    assert_eq!(std::fs::read(path).unwrap(), b"original");
    assert_eq!(
        backend::merge_pages(b"{\"items\":[1],\"nextPageToken\":\"next\"}\n{\"items\":[2]}")
            .unwrap(),
        json!({"items":[1,2]})
    );
}

#[test]
fn slides_set_does_not_delete_text_from_an_empty_placeholder() {
    let mut api = Recording::default();
    tasks::execute(
        &request(
            "jslides",
            &[
                "set",
                "presentation",
                "slide:1",
                "title:New title",
                "body:New body",
            ],
        )
        .unwrap(),
        &mut api,
    )
    .unwrap();
    let requests = api.calls.last().unwrap().2["requests"].as_array().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .all(|request| request.get("insertText").is_some())
    );
}

#[test]
fn spreadsheet_append_inserts_rows_across_the_whole_tab() {
    let mut api = Recording::default();
    let request = request("jsheets", &["append", "sheet", "row:Lara,42"]).unwrap();
    tasks::execute(&request, &mut api).unwrap();
    let params = &api.calls.last().unwrap().2;
    assert_eq!(params["range"], "'First tab'");
    assert_eq!(params["insertDataOption"], "INSERT_ROWS");
    assert_eq!(params["valueInputOption"], "RAW");
}

#[test]
fn google_urls_and_literal_text_are_normalized_without_accepting_other_hosts() {
    assert_eq!(
        values::resource("docs", "https://docs.google.com/document/d/abc123/edit").unwrap(),
        "abc123"
    );
    assert_eq!(
        values::resource("youtube", "https://www.youtube.com/watch?v=abc123").unwrap(),
        "abc123"
    );
    assert_eq!(
        values::resource("playlist", "https://www.youtube.com/playlist?list=abc123").unwrap(),
        "abc123"
    );
    assert!(
        values::resource(
            "docs",
            "https://docs.google.com.evil.example/document/d/abc123/edit"
        )
        .is_err()
    );
    assert!(
        values::resource(
            "forms",
            "https://docs.google.com/forms/d/e/published/viewform"
        )
        .is_err()
    );
    assert_eq!(values::unescape(r"a\\nb\nc"), "a\\nb\nc");
}

#[test]
fn schema_constraints_are_supported_and_explained_by_help() {
    fn assertion_keywords(rule: &Value) {
        for (key, value) in rule.as_object().unwrap() {
            match key.as_str() {
                "required" | "const" | "enum" | "description" => {}
                "properties" => value
                    .as_object()
                    .unwrap()
                    .values()
                    .for_each(assertion_keywords),
                "allOf" | "anyOf" => value
                    .as_array()
                    .unwrap()
                    .iter()
                    .for_each(assertion_keywords),
                "if" | "then" | "else" | "not" => assertion_keywords(value),
                key => panic!("unsupported wrapper assertion {key}"),
            }
        }
    }
    for name in catalog()["$defs"]
        .as_object()
        .unwrap()
        .keys()
        .filter(|name| name.contains('.'))
    {
        let (wrapper, command) = name.split_once('.').unwrap();
        let schema = definition(wrapper, command).unwrap();
        let help = help_text(wrapper, Some(command), true).unwrap();
        for rule in schema["allOf"].as_array().into_iter().flatten() {
            if let Some(description) = rule["description"].as_str() {
                assertion_keywords(rule);
                assert!(help.contains(description), "{name}: {description}");
            }
        }
        for (name, shape) in schema["properties"].as_object().unwrap() {
            assert!(help.contains(&format!("{}:", key(name, shape))));
            assert!(
                shape["description"]
                    .as_str()
                    .is_some_and(|text| !text.is_empty()),
                "{name} has no help description"
            );
        }
    }
    for invalid in ["A0", "A-1", "1A", "A1:B2:C3", "$$A1"] {
        assert!(
            values::format("cell-range", invalid).is_err(),
            "accepted {invalid}"
        );
    }
    for valid in ["A1", "$A$1", "A1:B20", "A:Z", "1:10"] {
        values::format("cell-range", valid).unwrap();
    }
}

#[test]
fn relative_calendar_bounds_are_checked_in_the_selected_timezone() {
    let bounds = request(
        "jcal",
        &[
            "find",
            "Meeting",
            "from:tomorrow",
            "until:today",
            "timezone:Pacific/Honolulu",
        ],
    )
    .unwrap();
    let mut api = Recording::default();
    let error = tasks::execute(&bounds, &mut api).unwrap_err();
    assert!(error.to_string().contains("calendar timezone"));
    assert!(api.calls.is_empty());
    assert_eq!(
        request("jcal", &["add", "Lunch", "at:tomorrow 12:00"])
            .unwrap()
            .text("for"),
        Some("30m")
    );
    assert!(
        request("jcal", &["add", "Holiday", "on:tomorrow"])
            .unwrap()
            .text("for")
            .is_none()
    );
}

#[test]
fn optional_parameters_build_complete_compound_requests() {
    let cases = [
        ("jdrive", vec!["copy", "file", "to:folder", "name:Copy"]),
        (
            "jdrive",
            vec![
                "share",
                "file",
                "to:lara@example.com",
                "role:writer",
                "notify:false",
            ],
        ),
        ("jsheets", vec!["new", "Budget", "tab:January"]),
        (
            "jsheets",
            vec!["read", "sheet", "tab:January", "range:A1:D20"],
        ),
        (
            "jdocs",
            vec!["new", "Report", "text:Hello", "folder:folder"],
        ),
        ("jdocs", vec!["append", "doc", "text:Hello", "tab:first"]),
        (
            "jdocs",
            vec![
                "share",
                "doc",
                "to:lara@example.com",
                "role:reader",
                "notify:false",
            ],
        ),
        (
            "jslides",
            vec!["new", "Presentation", "template:template", "folder:folder"],
        ),
        (
            "jslides",
            vec![
                "add",
                "deck",
                "title:Hello",
                "body:World",
                "layout:title",
                "after:0",
            ],
        ),
        (
            "jslides",
            vec!["replace", "deck", "find:Old", "with:New", "slide:1"],
        ),
        (
            "jforms",
            vec!["new", "Feedback", "description:Your feedback"],
        ),
        (
            "jforms",
            vec![
                "question",
                "form",
                "Which day?",
                "type:choice",
                "option:Monday",
                "option:Tuesday",
                "required:true",
            ],
        ),
        (
            "jtasks",
            vec![
                "set",
                "task",
                "title:Updated",
                "notes:Notes",
                "due:tomorrow",
                "list:list",
            ],
        ),
        (
            "jcontacts",
            vec![
                "new",
                "Lara Martin",
                "email:lara@example.com",
                "phone:+33123456789",
                "company:Company",
            ],
        ),
        (
            "jcontacts",
            vec!["set", "contact", "phone:+33123456789", "company:Company"],
        ),
        (
            "jyt",
            vec![
                "new-playlist",
                "Playlist",
                "privacy:unlisted",
                "description:Videos",
            ],
        ),
    ];
    let mut failures = Vec::new();
    for (wrapper, words) in cases {
        let request = request(wrapper, &words).unwrap();
        if let Err(error) = tasks::execute(&request, &mut Recording::default()) {
            failures.push(format!("{wrapper} {words:?}: {error:#}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
