use crate::services::Invocation;
use anyhow::{Context, Result, ensure};
use clap::{Arg, ArgMatches, Command, builder::PossibleValuesParser};
use serde_json::{Map, Value, json};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

fn value(name: &'static str, label: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .value_name(label)
        .help(help)
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
}
fn positional(name: &'static str, label: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .value_name(label)
        .help(help)
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
}
fn command(name: &'static str, about: &'static str, read: bool, list: bool) -> Command {
    let mut command = Command::new(name)
        .about(about)
        .arg(value(
            "file",
            "PATH",
            "Read command parameters from a JSON file",
        ))
        .arg(value("fields", "MASK", "Select Google response fields"));
    if read {
        command = command.arg(
            value(
                "ttl",
                "MINUTES",
                "Reuse cached data for MINUTES; 0 disables caching",
            )
            .default_value("0")
            .value_parser(clap::value_parser!(u32)),
        );
    }
    if list {
        command = command
            .arg(
                value(
                    "max",
                    "N",
                    "Return at most N results; by default, return all results",
                )
                .value_parser(clap::value_parser!(u32).range(1..)),
            )
            .arg(
                value(
                    "max-pages",
                    "N",
                    "Stop after N pages; by default, fetch all pages",
                )
                .value_parser(clap::value_parser!(u32).range(1..=10000)),
            );
    }
    command
}
pub fn commands(service: &str) -> Vec<Command> {
    let commands = match service {
        "mail" => [
            ("read", "Mark a message as read"),
            ("unread", "Mark a message as unread"),
            ("star", "Star a message"),
            ("unstar", "Remove a message's star"),
        ]
        .into_iter()
        .map(|(name, about)| {
            command(name, about, false, false).arg(positional("id", "MESSAGE_ID", "Message ID"))
        })
        .collect(),
        "drive" => vec![
            command("search", "Search file contents", true, true)
                .arg(positional("query", "TEXT", "Search text"))
                .arg(value("parent", "ID", "Search inside this folder"))
                .arg(value("type", "MIME", "Filter by MIME type")),
            command(
                "restore",
                "Restore a file or folder from Trash",
                false,
                false,
            )
            .arg(positional("id", "FILE_ID", "File or folder ID")),
        ],
        "calendar" => vec![
            command("events", "List events in chronological order", true, true)
                .arg(value("calendar", "ID", "Calendar ID").default_value("primary"))
                .arg(value(
                    "from",
                    "TIMESTAMP",
                    "RFC 3339 lower bound for event end times; defaults to now",
                ))
                .arg(value(
                    "until",
                    "TIMESTAMP",
                    "RFC 3339 upper bound for event start times; defaults to 30 days after --from",
                ))
                .arg(value("query", "TEXT", "Search event text")),
        ],
        "forms" => vec![
            command("responses", "List submitted responses", true, true)
                .arg(positional("id", "FORM_ID", "Form ID"))
                .arg(value(
                    "since",
                    "TIMESTAMP",
                    "Responses submitted at or after this UTC RFC 3339 timestamp",
                )),
            command("response", "Show a submitted response", true, false)
                .arg(positional("id", "FORM_ID", "Form ID"))
                .arg(positional("response-id", "RESPONSE_ID", "Response ID")),
        ],
        "youtube" => vec![
            command("search", "Search YouTube", true, true)
                .arg(positional("query", "QUERY", "Search text"))
                .arg(
                    value("type", "TYPE", "Search result type")
                        .value_parser(PossibleValuesParser::new(["video", "channel", "playlist"]))
                        .default_value("video"),
                )
                .arg(
                    value("order", "ORDER", "Result order")
                        .value_parser(PossibleValuesParser::new([
                            "date",
                            "rating",
                            "relevance",
                            "title",
                            "videoCount",
                            "viewCount",
                        ]))
                        .default_value("relevance"),
                ),
        ],
        _ => Vec::new(),
    };
    commands
        .into_iter()
        .map(|command| {
            let schema = &crate::schema_commands::contract(service).inputs[command.get_name()];
            let required = schema["allOf"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|rule| rule["x-cli-kind"] == "required")
                .flat_map(|rule| rule["x-cli-parameters"].as_array().into_iter().flatten())
                .filter_map(|key| {
                    schema["properties"][key.as_str().unwrap()]["x-cli-binding"]["property"]
                        .as_str()
                })
                .collect::<std::collections::BTreeSet<_>>();
            let command = command.mut_args(|arg| {
                if required.contains(arg.get_id().as_str()) {
                    arg.required_unless_present("file")
                } else {
                    arg
                }
            });
            let mut preview = command.clone().bin_name(format!(
                "joog [OPTIONS] {service} <ACCOUNT> {}",
                command.get_name()
            ));
            let usage = preview
                .render_usage()
                .to_string()
                .trim_start_matches("Usage: ")
                .replace(
                    &format!("{} [OPTIONS]", command.get_name()),
                    &format!("{} [COMMAND_OPTIONS]", command.get_name()),
                );
            command.override_usage(usage)
        })
        .collect()
}
pub fn arguments(args: &ArgMatches) -> &ArgMatches {
    args.subcommand().map_or(args, |(_, child)| child)
}

pub fn text<'a>(args: &'a ArgMatches, name: &str) -> Option<&'a str> {
    args.try_get_one::<String>(name)
        .ok()
        .flatten()
        .map(String::as_str)
}

fn set(params: &mut Map<String, Value>, key: &str, args: &ArgMatches, flag: &str) {
    if let Some(value) = text(args, flag)
        && (args.value_source(flag) == Some(clap::parser::ValueSource::CommandLine)
            || !params.contains_key(key))
    {
        params.insert(key.into(), json!(value));
    }
}

fn quoted(text: &str) -> String {
    text.replace('\\', "\\\\").replace('\'', "\\'")
}

fn timestamp(value: &str) -> Result<()> {
    OffsetDateTime::parse(value, &Rfc3339)
        .context("use an RFC 3339 timestamp with a time zone offset")?;
    Ok(())
}

#[cfg(test)]
pub fn documentation_form(service: &str, command: &Command) -> String {
    let mut form = format!("joog {service} ACCOUNT {}", command.get_name());
    let mut positionals = command
        .get_arguments()
        .filter(|arg| arg.get_long().is_none())
        .collect::<Vec<_>>();
    positionals.sort_by_key(|arg| arg.get_index());
    let label = |arg: &Arg| {
        arg.get_value_names()
            .map(|names| {
                names
                    .iter()
                    .map(|name| name.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_else(|| arg.get_id().as_str().to_ascii_uppercase())
    };
    for arg in positionals {
        form.push(' ');
        form.push_str(&label(arg));
    }
    for arg in command.get_arguments() {
        let Some(long) = arg.get_long() else {
            continue;
        };
        if ["file", "fields", "ttl", "max", "max-pages"].contains(&long) {
            continue;
        }
        let mut option = format!("--{long}");
        if arg.get_action().takes_values() {
            option.push(' ');
            option.push_str(&label(arg));
            if arg
                .get_num_args()
                .is_some_and(|range| range.max_values() > 1)
            {
                option.push_str("...");
            }
        }
        form.push(' ');
        if arg.is_required_set() {
            form.push_str(&option);
        } else {
            form.push_str(&format!("[{option}]"));
        }
    }
    form
}

pub fn resolve(
    service: &str,
    _account: &str,
    action: &str,
    args: &ArgMatches,
    mut params: Map<String, Value>,
) -> Result<Invocation> {
    let custom_fields = params.contains_key("fields") || text(args, "fields").is_some();
    let mut body = None;
    let mut relative_times = Vec::new();
    let method = match (service, action) {
        ("mail", "read" | "unread" | "star" | "unstar") => {
            params.insert(
                "id".into(),
                json!(text(args, "id").context("missing message ID")?),
            );
            params.entry("userId").or_insert(json!("me"));
            body = Some(match action {
                "read" => json!({"removeLabelIds":["UNREAD"]}),
                "unread" => json!({"addLabelIds":["UNREAD"]}),
                "star" => json!({"addLabelIds":["STARRED"]}),
                _ => json!({"removeLabelIds":["STARRED"]}),
            });
            "users.messages.modify"
        }
        ("drive", "search") => {
            let mut clauses = vec!["trashed = false".to_owned()];
            if let Some(parent) = text(args, "parent") {
                clauses.push(format!("'{}' in parents", quoted(parent)));
            }
            if let Some(kind) = text(args, "type") {
                clauses.push(format!("mimeType = '{}'", quoted(kind)));
            }
            if let Some(query) = text(args, "query") {
                clauses.push(format!("fullText contains '{}'", quoted(query)));
            }
            if let Some(query) = params.remove("q") {
                ensure!(query.is_string(), "q must be a string");
                clauses.push(format!("({})", query.as_str().unwrap()));
            }
            params.insert("q".into(), json!(clauses.join(" and ")));
            params.entry("fields").or_insert(json!(
                "nextPageToken,files(id,name,mimeType,size,modifiedTime,webViewLink)"
            ));
            "files.list"
        }
        ("drive", "restore") => {
            params.insert(
                "fileId".into(),
                json!(text(args, "id").context("missing file ID")?),
            );
            body = Some(json!({"trashed":false}));
            "files.update"
        }
        ("calendar", "events") => {
            set(&mut params, "calendarId", args, "calendar");
            params.entry("singleEvents").or_insert(json!(true));
            params.entry("orderBy").or_insert(json!("startTime"));
            for (flag, key) in [("from", "timeMin"), ("until", "timeMax")] {
                if let Some(value) = text(args, flag) {
                    timestamp(value)?;
                    params.insert(key.into(), json!(value));
                }
            }
            if !params.contains_key("timeMin") {
                relative_times.push("timeMin");
                let now = OffsetDateTime::now_utc();
                params.insert("timeMin".into(), json!(now.format(&Rfc3339)?));
            }
            if !params.contains_key("timeMax") {
                relative_times.push("timeMax");
                let start = params["timeMin"]
                    .as_str()
                    .context("timeMin must be a timestamp string")?;
                let end = OffsetDateTime::parse(start, &Rfc3339)?
                    .checked_add(time::Duration::days(30))
                    .context("calendar window is outside the supported date range")?;
                params.insert("timeMax".into(), json!(end.format(&Rfc3339)?));
            }
            if let (Some(start), Some(end)) = (
                params.get("timeMin").and_then(Value::as_str),
                params.get("timeMax").and_then(Value::as_str),
            ) {
                ensure!(
                    OffsetDateTime::parse(start, &Rfc3339)? < OffsetDateTime::parse(end, &Rfc3339)?,
                    "--until must be later than --from"
                );
            }
            set(&mut params, "q", args, "query");
            "events.list"
        }
        ("forms", "responses" | "response") => {
            params.insert(
                "formId".into(),
                json!(text(args, "id").context("missing form ID")?),
            );
            if action == "responses" {
                if let Some(since) = text(args, "since") {
                    timestamp(since)?;
                    ensure!(
                        since.ends_with('Z'),
                        "--since requires a UTC timestamp ending in Z"
                    );
                    params.insert("filter".into(), json!(format!("timestamp >= {since}")));
                }
                "forms.responses.list"
            } else {
                set(&mut params, "responseId", args, "response-id");
                "forms.responses.get"
            }
        }
        ("youtube", "search") => {
            params.insert("part".into(), json!("snippet"));
            for (flag, key) in [("query", "q"), ("type", "type"), ("order", "order")] {
                set(&mut params, key, args, flag);
            }
            "search.list"
        }
        _ => anyhow::bail!("unsupported command {service} {action}"),
    };
    if let Some(fields) = text(args, "fields") {
        params.insert("fields".into(), json!(fields));
    }
    Ok(Invocation {
        options: None,
        method: method.into(),
        params,
        body,
        custom_fields,
        relative_times,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cli, discovery::Document, pagination::Plan, services::SERVICES};

    fn sample(name: &str) -> &'static str {
        match name {
            "subject" | "title" | "given-name" | "name" => "Example",
            "text" | "notes" => "Message text",
            "query" => "test query",
            "range" => "Sheet1!A1:B2",
            "values" => "[[\"Name\",1],[\"Other\",2]]",
            "start" => "2026-12-01T10:00:00+01:00",
            "end" => "2026-12-01T11:00:00+01:00",
            "to" | "cc" | "bcc" => "recipient@example.com",
            "type" => "application/pdf",
            _ => "example-id",
        }
    }

    fn check_body(
        schema: &Value,
        value: &Value,
        schemas: &std::collections::BTreeMap<String, Value>,
        path: &str,
    ) {
        if let Some(name) = schema["$ref"].as_str() {
            return check_body(&schemas[name], value, schemas, path);
        }
        if value.is_null() {
            return;
        }
        match schema["type"].as_str().unwrap_or_default() {
            "object" => {
                let object = value
                    .as_object()
                    .unwrap_or_else(|| panic!("{path}: expected object"));
                for (key, child) in object {
                    let child_schema = schema["properties"]
                        .get(key)
                        .or_else(|| schema.get("additionalProperties"))
                        .unwrap_or_else(|| panic!("{path}.{key}: unknown property"));
                    assert_ne!(
                        child_schema["readOnly"], true,
                        "{path}.{key}: output-only property"
                    );
                    check_body(child_schema, child, schemas, &format!("{path}.{key}"));
                }
            }
            "array" => {
                for (index, child) in value.as_array().unwrap().iter().enumerate() {
                    check_body(
                        &schema["items"],
                        child,
                        schemas,
                        &format!("{path}[{index}]"),
                    );
                }
            }
            "string" => assert!(value.is_string(), "{path}: expected string"),
            "integer" => assert!(
                value.as_i64().is_some() || value.as_u64().is_some(),
                "{path}: expected integer"
            ),
            "number" => assert!(value.is_number(), "{path}: expected number"),
            "boolean" => assert!(value.is_boolean(), "{path}: expected boolean"),
            _ => {}
        }
        if let Some(choices) = schema["enum"].as_array() {
            assert!(choices.contains(value), "{path}: invalid enum value");
        }
    }

    #[test]
    fn every_native_command_builds_a_request_matching_google_discovery() {
        let fixtures: Value =
            serde_json::from_str(include_str!("../tests/data/native-requests.json")).unwrap();
        for service in SERVICES {
            let doc: Document = serde_json::from_value(fixtures[service.cli].clone()).unwrap();
            let commands = commands(service.cli);
            assert_eq!(
                commands.len(),
                doc.root.methods.len(),
                "{} command coverage",
                service.cli
            );
            for cmd in commands {
                let action = cmd.get_name();
                let mut args = vec![
                    "joog".to_owned(),
                    service.cli.into(),
                    "sender@example.com".into(),
                    action.into(),
                ];
                for arg in cmd
                    .get_arguments()
                    .filter(|arg| arg.is_required_set() || arg.get_long().is_none())
                {
                    let name = arg.get_id().as_str();
                    if let Some(long) = arg.get_long() {
                        args.push(format!("--{long}"));
                    }
                    args.push(sample(name).into());
                }
                let root =
                    cli::parse_from(&args).unwrap_or_else(|error| panic!("{args:?}: {error}"));
                let context = root.subcommand().unwrap().1;
                let native = context.subcommand().unwrap().1;
                let call = resolve(
                    service.cli,
                    "sender@example.com",
                    action,
                    native,
                    Map::new(),
                )
                .unwrap_or_else(|error| panic!("{args:?}: {error:#}"));
                assert_eq!(
                    call.method, fixtures[service.cli]["x-expected-methods"][action],
                    "{args:?}"
                );
                let spec = &doc.root.methods[action];
                let url = doc
                    .url(spec, &call.params)
                    .unwrap_or_else(|error| panic!("{args:?}: {error:#}"));
                assert_eq!(url.scheme(), "https");
                if let Some(body) = &call.body {
                    check_body(
                        spec.request.as_ref().expect("write request schema"),
                        body,
                        &doc.schemas,
                        &format!("{} {action}", service.cli),
                    );
                }
                let read = spec.http_method == "GET";
                Plan::new(spec, read, None, None).unwrap();
                if cmd.get_arguments().any(|arg| arg.get_id() == "max") {
                    Plan::new(spec, read, Some(2), None)
                        .unwrap_or_else(|error| panic!("{} {action}: {error}", service.cli));
                }
                if cmd.get_arguments().any(|arg| arg.get_id() == "max-pages") {
                    Plan::new(spec, read, None, Some(2))
                        .unwrap_or_else(|error| panic!("{} {action}: {error}", service.cli));
                }
                let scopes = crate::services::scopes(service.cli).unwrap();
                assert!(
                    spec.scopes.iter().any(|scope| scopes.contains(scope)),
                    "{} {action}: missing OAuth scope",
                    service.cli
                );
                let mut json_args = args.clone();
                json_args.insert(3, "--json".into());
                let json = cli::parse_from(&json_args).unwrap();
                assert!(json.get_flag("json"));
                assert!(arguments(json.subcommand().unwrap().1).get_flag("json"));
                let mut misplaced = args.clone();
                misplaced.push("--json".into());
                assert!(
                    cli::parse_from(misplaced).is_err(),
                    "global options after {service_name} {action}",
                    service_name = service.cli
                );
            }
        }
    }

    fn prepare(service: &str, tail: &[&str]) -> Result<Invocation> {
        let args = vec!["joog", service, "sender@example.com"]
            .into_iter()
            .chain(tail.iter().copied());
        let root = cli::parse_from(args)?;
        let context = root.subcommand().unwrap().1;
        let (action, args) = context.subcommand().unwrap();
        resolve(service, "sender@example.com", action, args, Map::new())
    }

    #[test]
    fn command_defaults_do_not_override_parameter_files() {
        let root = cli::parse_from(["joog", "calendar", "sender@example.com", "events"]).unwrap();
        let args = arguments(root.subcommand().unwrap().1);
        let params = json!({"calendarId":"selected-calendar","timeMin":"2026-12-01T00:00:00Z","timeMax":"2026-12-03T00:00:00Z"});
        let call = resolve(
            "calendar",
            "sender@example.com",
            "events",
            args,
            params.as_object().unwrap().clone(),
        )
        .unwrap();
        assert_eq!(call.params["calendarId"], "selected-calendar");
        let root = cli::parse_from([
            "joog",
            "calendar",
            "sender@example.com",
            "events",
            "--calendar",
            "explicit-calendar",
        ])
        .unwrap();
        let args = arguments(root.subcommand().unwrap().1);
        let call = resolve(
            "calendar",
            "sender@example.com",
            "events",
            args,
            params.as_object().unwrap().clone(),
        )
        .unwrap();
        assert_eq!(call.params["calendarId"], "explicit-calendar");
        assert!(!call.custom_fields);
        let root = cli::parse_from([
            "joog",
            "docs",
            "sender@example.com",
            "get",
            "--document-id",
            "document",
            "--fields",
            "title",
        ])
        .unwrap();
        let call = crate::service_call(
            crate::services::SERVICES
                .iter()
                .find(|s| s.cli == "docs")
                .unwrap(),
            root.subcommand().unwrap().1,
        )
        .unwrap();
        assert!(call.custom_fields);
    }

    #[test]
    fn relative_calendar_windows_cache_by_intent_and_do_not_collide_with_unbounded_requests() {
        let fixtures: Value =
            serde_json::from_str(include_str!("../tests/data/native-requests.json")).unwrap();
        let doc: Document = serde_json::from_value(fixtures["calendar"].clone()).unwrap();
        let spec = &doc.root.methods["events"];
        let mut call = prepare("calendar", &["events"]).unwrap();
        let key = |call: &Invocation| {
            crate::cache::request_identity(&doc, spec, &call.params, &call.relative_times).unwrap()
        };
        let first = key(&call);
        assert!(call.params.contains_key("timeMin") && call.params.contains_key("timeMax"));
        call.params
            .insert("timeMin".into(), json!("2026-12-02T00:00:00Z"));
        call.params
            .insert("timeMax".into(), json!("2027-01-01T00:00:00Z"));
        assert_eq!(first, key(&call));
        let mut unbounded = call.params.clone();
        unbounded.remove("timeMin");
        unbounded.remove("timeMax");
        let generic = crate::cache::request_identity(&doc, spec, &unbounded, &[]).unwrap();
        assert_ne!(first, generic);
        let explicit = prepare(
            "calendar",
            &[
                "events",
                "--from",
                "2026-12-01T00:00:00Z",
                "--until",
                "2026-12-04T00:00:00Z",
            ],
        )
        .unwrap();
        assert_eq!(
            key(&explicit),
            doc.url(spec, &explicit.params).unwrap().as_str()
        );
        assert_ne!(first, key(&explicit));
    }
    #[test]
    fn selected_actions_preserve_filtering_label_changes_and_response_ids() {
        let call = prepare("mail", &["read", "message"]).unwrap();
        assert_eq!(call.body.unwrap(), json!({"removeLabelIds":["UNREAD"]}));
        let call = prepare("drive", &["restore", "file"]).unwrap();
        assert_eq!(call.body.unwrap(), json!({"trashed":false}));
        let call = prepare("drive", &["search", "O'Brien", "--parent", "folder"]).unwrap();
        assert!(call.params["q"].as_str().unwrap().contains("O\\'Brien"));
        let call = prepare("forms", &["response", "form", "response"]).unwrap();
        assert_eq!(call.params["formId"], "form");
        assert_eq!(call.params["responseId"], "response");
        assert!(
            prepare(
                "calendar",
                &[
                    "events",
                    "--from",
                    "2026-12-02T00:00:00Z",
                    "--until",
                    "2026-12-01T00:00:00Z"
                ]
            )
            .is_err()
        );
        assert!(
            prepare(
                "forms",
                &["responses", "form", "--since", "2026-12-01T00:00:00+01:00"]
            )
            .is_err()
        );
    }
}
