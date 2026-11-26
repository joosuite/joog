use crate::services::SERVICES;
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::ffi::OsString;

fn switch(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::SetTrue)
}

#[cfg(test)]
pub fn command() -> Command {
    build(None, false, true)
}

#[cfg(test)]
pub fn verbose_command() -> Command {
    build(None, true, true)
}

fn build(selected: Option<&str>, verbose: bool, style_help: bool) -> Command {
    let auth = Command::new("auth")
        .about("Manage Google sign-in and client credentials")
        .long_about("Google client credentials and account tokens are encrypted locally. Use the OS keychain on a desktop, or a private key file with --headless on a server.")
        .after_help("Setup:\n  joog auth init\n  joog auth enroll\n  joog auth test\n  joog auth you@example.com login all\n\nUse all for every supported service, or a comma-separated list such as gmail,drive.\nCreate Desktop app OAuth credentials in Google Cloud Console.\n\nGuide: docs/authentication.md")
        .override_usage("joog auth [OPTIONS] init|enroll|test|list\n       joog auth [OPTIONS] <ACCOUNT> login <SERVICE_LIST>\n       joog auth [OPTIONS] <ACCOUNT> all\n       joog auth [OPTIONS] [ACCOUNT] export <FILE>\n       joog auth [OPTIONS] [ACCOUNT] import <FILE> [--headless] [--replace]\n       joog auth [OPTIONS] <ACCOUNT> logout\n       joog auth help [COMMAND]")
        .subcommand_required(true)
        .subcommand_precedence_over_arg(true)
        .arg(Arg::new("account").value_name("ACCOUNT").help("Google account email"))
        .subcommand(Command::new("init")
            .about("Create or reuse the credential vault")
            .long_about("Create a 256-bit master key in the OS keychain, or reuse the existing vault. Use --headless for a private local key file on a server without a desktop keychain. Existing encrypted credentials require their original key.")
            .arg(switch("headless", "Use a private local key file without an OS keychain"))
            .after_help("Guide: docs/authentication.md"))
        .subcommand(
            Command::new("enroll")
                .about("Save Google client credentials")
                .long_about("Save your Google Desktop app client ID and secret in the encrypted vault. Replaces previously enrolled client credentials.")
                .after_help("Google Cloud Console > Google Auth Platform > Clients > Create client > Desktop app\n\n--stdin accepts a JSON object with client_id and client_secret fields.\n\nGuide: docs/authentication.md")
                .arg(switch(
                    "stdin",
                    "Read client_id and client_secret as JSON from stdin",
                )),
        )
        .subcommand(
            Command::new("test")
                .about("Test Google OAuth connectivity with the enrolled client")
                .long_about("Check connectivity to Google's OAuth service using the enrolled client credentials.")
                .after_help("Example:\n  joog auth test\n\nGuide: docs/authentication.md"),
        )
        .subcommand(
            Command::new("list")
                .override_usage("joog [OPTIONS] auth list")
                .about("List registered Google accounts")
                .long_about("Show registered account emails in the selected vault, one per line. Use --json for a JSON array. Listing does not unlock the credential vault or check authorization with Google.")
                .after_help("Examples:\n  joog auth list\n  joog --json auth list\n\nGuide: docs/authentication.md"),
        )
        .subcommand(
            Command::new("login")
                .override_usage("joog [OPTIONS] auth <ACCOUNT> login <SERVICE_LIST>")
                .about("Sign in to a Google account")
                .long_about("Sign in through your default browser and authorize the requested Google services. Tokens are encrypted locally. Google must verify the requested account email. Sign-in expires after three minutes.")
                .after_help("Examples:\n  joog auth you@example.com login all\n  joog auth you@example.com login gmail,drive\n\nGuide: docs/authentication.md")
                .arg(
                    Arg::new("services")
                        .value_name("SERVICE_LIST")
                        .required(true)
                        .value_parser(|value: &str| {
                            crate::services::scopes(value).map(|_| value.to_owned()).map_err(|e| e.to_string())
                        })
                        .help("Use all for every supported service, or a comma-separated list such as gmail,drive"),
                ),
        )
        .subcommand(
            Command::new("all")
                .override_usage("joog [OPTIONS] auth <ACCOUNT> all")
                .about("Sign in and authorize all supported services")
                .long_about("Open Google sign-in and request every configured scope for all supported services. Equivalent to login all.")
                .after_help("Example:\n  joog auth you@example.com all\n\nGuide: docs/authentication.md"),
        )
        .subcommand(
            Command::new("export")
                .override_usage("joog [OPTIONS] auth [ACCOUNT] export <FILE> [COMMAND_OPTIONS]")
                .about("Export sign-in credentials for another machine")
                .long_about("Save the OAuth client and refresh tokens in a passphrase-encrypted transfer file. Include ACCOUNT to export one account, or omit it to export all signed-in accounts. Prompts for a passphrase of at least 12 characters. Existing files are never overwritten. No Google requests are made.")
                .arg(Arg::new("transfer-file").value_name("FILE").required(true).help("New encrypted transfer file to create"))
                .arg(Arg::new("passphrase-file").long("passphrase-file").value_name("PATH").help("Read the passphrase from a private UTF-8 file instead of prompting").hide(true))
                .after_help("Example:\n  joog auth you@example.com export credentials.joog\n  scp credentials.joog user@vps:~/\n\nGuide: docs/authentication.md"),
        )
        .subcommand(
            Command::new("import")
                .override_usage("joog [OPTIONS] auth [ACCOUNT] import <FILE> [COMMAND_OPTIONS]")
                .about("Import sign-in credentials without a browser")
                .long_about("Unlock an encrypted transfer file with its passphrase and save its OAuth client and refresh tokens in this machine's vault. Include ACCOUNT to import only that account. No browser or Google request is needed; access tokens refresh automatically when you next use a service. Existing credentials are preserved unless --replace is supplied.")
                .arg(Arg::new("transfer-file").value_name("FILE").required(true).help("Encrypted transfer file to import"))
                .arg(switch("headless", "Create a vault with a private local key file; no desktop keychain required"))
                .arg(switch("replace", "Replace credentials for imported accounts; other accounts are preserved"))
                .arg(Arg::new("passphrase-file").long("passphrase-file").value_name("PATH").help("Read the passphrase from a private UTF-8 file instead of prompting").hide(true))
                .after_help("Example on the server:\n  joog auth import credentials.joog --headless\n\nThe headless vault stores its master key in master.key with owner-only permissions.\nUse --home with a new directory to keep it separate from an existing keychain vault.\n\nGuide: docs/authentication.md"),
        )
        .subcommand(
            Command::new("logout")
                .override_usage("joog [OPTIONS] auth <ACCOUNT> logout")
                .about("Delete the account's local tokens")
                .after_help("Google permissions remain active until revoked in your Google Account settings.\n\nGuide: docs/authentication.md"),
        );

    let mut cmd = Command::new("joog").version(env!("CARGO_PKG_VERSION"))
        .about("Google services from the terminal")
        .subcommand_required(true).arg_required_else_help(true)
        .after_help("Examples:\n  joog drive you@example.com files.list --max 10\n  joog mail you@example.com messages.list --q is:unread\n  joog sheets you@example.com values.get --spreadsheet-id SPREADSHEET_ID --range 'Sheet1!A1:D10'\n\nCommands: docs/grammar.md\nParameter files: docs/file-schemas.md\nSign-in setup: docs/authentication.md")
        .arg(Arg::new("home").long("home").help("Store state in PATH/joog").value_name("PATH").help_heading("Joog options").global(true))
        .arg(Arg::new("timeout").long("timeout").value_name("SECONDS").default_value("30").value_parser(value_parser!(u64).range(1..=600)).help("Google request timeout in seconds").help_heading("Joog options").global(true))
        .arg(switch("json", "Output JSON instead of human-readable text").help_heading("Joog options").global(true))
        .arg(switch("verbose", "Include advanced parameters in help").global(true))
        .subcommand(auth);
    for service in SERVICES {
        let mut service_cmd = Command::new(service.cli).about(service.title);
        if selected.is_none() || selected == Some(service.cli) {
            service_cmd=service_cmd
                .override_usage(format!("joog [OPTIONS] {} <ACCOUNT> <COMMAND> [COMMAND_OPTIONS]",service.cli))
                .after_help(format!("Commands: docs/grammar.md\nMethod reference: docs/services/{}.md\nRequest files: docs/file-schemas.md",service.cli))
                .arg(Arg::new("account").value_name("ACCOUNT").required(true).index(1).help("Google account email"))
                .subcommand_precedence_over_arg(true).subcommand_required(true);
            for native in crate::native::commands(service.cli) {
                service_cmd = service_cmd.subcommand(native);
            }
            for operation in crate::schema_commands::commands(service.cli) {
                service_cmd = service_cmd.subcommand(operation);
            }
        }
        cmd = cmd.subcommand(service_cmd);
    }
    if style_help {
        cmd.build();
        crate::help::configure(cmd, None, verbose)
    } else {
        cmd
    }
}

fn select_service(raw: &[OsString]) -> Option<(usize, String)> {
    let mut index = 1;
    while index < raw.len() {
        let word = raw[index].to_str()?;
        match word {
            "--home" | "--timeout" => index += 2,
            "--json" | "--verbose" => index += 1,
            "help" => {
                return raw
                    .get(index + 1)
                    .and_then(|word| word.to_str())
                    .filter(|name| SERVICES.iter().any(|service| service.cli == *name))
                    .map(|service| (index + 1, service.to_owned()));
            }
            _ if word.starts_with("--home=") || word.starts_with("--timeout=") => index += 1,
            _ => {
                return SERVICES
                    .iter()
                    .find(|service| service.cli == word)
                    .map(|service| (index, service.cli.to_owned()));
            }
        }
    }
    None
}
/// Parse context before the action and reject options after it.
pub fn parse_from<I, T>(args: I) -> Result<ArgMatches, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let mut raw = args.into_iter().map(Into::into).collect::<Vec<OsString>>();
    let selected = select_service(&raw);
    let verbose = raw
        .iter()
        .take_while(|arg| arg.as_os_str() != "--")
        .any(|arg| arg.as_os_str() == "--verbose");
    let style_help = verbose
        || raw
            .iter()
            .any(|arg| matches!(arg.to_str(), Some("--help" | "-h" | "help")));
    if verbose {
        let end = raw
            .iter()
            .position(|arg| arg.as_os_str() == "--")
            .unwrap_or(raw.len());
        raw = raw
            .into_iter()
            .enumerate()
            .filter_map(|(index, arg)| {
                (index >= end || arg.as_os_str() != "--verbose").then_some(arg)
            })
            .collect();
    }
    let mut definition = build(
        Some(
            selected
                .as_ref()
                .map(|(_, service)| service.as_str())
                .unwrap_or(""),
        ),
        verbose,
        style_help,
    );
    let matches = definition.try_get_matches_from_mut(&raw).map_err(|error| {
        if error.kind() == clap::error::ErrorKind::DisplayHelp {
            Command::new("joog")
                .override_help(crate::help::order_sections(&error.to_string()))
                .try_get_matches_from(["joog", "--help"])
                .unwrap_err()
        } else {
            error
        }
    })?;
    if verbose {
        return Err(clap::Error::raw(
            clap::error::ErrorKind::ArgumentConflict,
            "--verbose is a help modifier; use --help --verbose\n",
        )
        .with_cmd(&definition));
    }
    let (namespace, context) = matches.subcommand().expect("required namespace");
    let account = context.get_one::<String>("account");
    let action = context.subcommand_name().expect("required command");
    let error = |kind, message: String| clap::Error::raw(kind, message).with_cmd(&definition);
    if namespace == "auth" {
        if matches!(action, "login" | "all" | "logout") && account.is_none() {
            return Err(error(
                clap::error::ErrorKind::MissingRequiredArgument,
                format!(
                    "use joog auth <account> {action}{}\n",
                    if action == "login" {
                        " <service-list>"
                    } else {
                        ""
                    }
                ),
            ));
        }
        if matches!(action, "init" | "enroll" | "test" | "list") && account.is_some() {
            return Err(error(
                clap::error::ErrorKind::ArgumentConflict,
                format!("auth {action} applies to the shared vault; omit the account\n"),
            ));
        }
    }
    let context_definition = definition.find_subcommand(namespace).unwrap();
    let action_definition = context_definition.find_subcommand(action);
    let definitions = definition
        .get_arguments()
        .chain(context_definition.get_arguments())
        .chain(
            action_definition
                .into_iter()
                .flat_map(Command::get_arguments),
        );
    let mut long_options = std::collections::BTreeMap::new();
    let mut short_options = std::collections::BTreeMap::new();
    for arg in definitions {
        let before_action = arg.is_global_set();
        let value = arg.get_action().takes_values() && !arg.is_require_equals_set();
        if let Some(long) = arg.get_long() {
            long_options.insert(long, (value, before_action));
        }
        if let Some(short) = arg.get_short() {
            short_options.insert(short, (value, before_action));
        }
    }
    let mut positional = Vec::new();
    let mut options = Vec::new();
    let mut skip_value = false;
    let mut literal = false;
    for (index, arg) in raw.iter().enumerate().skip(1) {
        let arg = arg.to_string_lossy();
        if skip_value {
            skip_value = false;
            continue;
        }
        if !literal && arg == "--" {
            literal = true;
            continue;
        }
        if !literal && let Some(option) = arg.strip_prefix("--") {
            let name = option.split('=').next().unwrap();
            let &(value, before_action) = long_options.get(name).expect("option validated by clap");
            options.push((index, name.to_owned(), before_action));
            skip_value = value && !option.contains('=');
        } else if !literal && arg.starts_with('-') && arg.len() > 1 {
            let short = arg.chars().nth(1).unwrap();
            let &(value, before_action) = short_options
                .get(&short)
                .expect("short option validated by clap");
            options.push((index, short.to_string(), before_action));
            skip_value = value && arg.len() == 2;
        } else {
            positional.push(index);
        }
    }
    let verb = positional[if account.is_some() { 2 } else { 1 }];
    for (index, option, before_action) in options {
        if before_action && index > verb {
            return Err(error(
                clap::error::ErrorKind::ArgumentConflict,
                format!("joog option --{option} must precede the action {action}\n"),
            ));
        }
        if !before_action && index < verb {
            return Err(error(
                clap::error::ErrorKind::ArgumentConflict,
                format!("command option --{option} must follow the action {action}\n"),
            ));
        }
    }
    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn representative(service: &str) -> &'static str {
        crate::schema_commands::contract(service)
            .document
            .root
            .methods
            .iter()
            .find(|(_, spec)| {
                spec.http_method == "GET" && !crate::schema_commands::direct_download(spec)
            })
            .map(|(name, _)| name.as_str())
            .unwrap()
    }

    #[test]
    fn read_methods_default_to_all_results_and_only_offer_supported_limits() {
        for service in SERVICES {
            let document = &crate::schema_commands::contract(service.cli).document;
            for (method, spec) in document.root.methods.iter().filter(|(_, spec)| {
                spec.http_method == "GET"
                    && !spec.fresh_response
                    && !crate::schema_commands::direct_download(spec)
            }) {
                let base = [
                    "joog",
                    service.cli,
                    "reader@example.com",
                    method,
                    "--file",
                    "request.json",
                ];
                let parsed = parse_from(base).unwrap();
                let args = crate::native::arguments(parsed.subcommand().unwrap().1);
                assert!(args.try_get_one::<u32>("max").ok().flatten().is_none());
                assert!(
                    args.try_get_one::<u32>("max-pages")
                        .ok()
                        .flatten()
                        .is_none()
                );
                assert_eq!(args.get_one::<u32>("ttl"), Some(&0));
                let json = parse_from(base.into_iter().chain(["--ttl", "60"])).unwrap();
                assert_eq!(
                    crate::native::arguments(json.subcommand().unwrap().1).get_one::<u32>("ttl"),
                    Some(&60)
                );
                for (flag, supported) in [
                    (
                        "--max",
                        crate::pagination::Plan::new(spec, true, Some(2), None).is_ok(),
                    ),
                    (
                        "--max-pages",
                        crate::pagination::Plan::new(spec, true, None, Some(2)).is_ok(),
                    ),
                ] {
                    assert_eq!(
                        parse_from(base.into_iter().chain([flag, "2"])).is_ok(),
                        supported,
                        "{} {method} {flag}",
                        service.cli
                    );
                }
                for flag in ["--all", "--no-cache"] {
                    assert!(parse_from(base.into_iter().chain([flag])).is_err());
                }
            }
        }
    }

    #[test]
    fn service_commands_accept_parameter_files_after_the_action() {
        for service in SERVICES {
            let method = representative(service.cli);
            let args = parse_from([
                "joog",
                service.cli,
                "reader@example.com",
                method,
                "--file",
                "request.json",
            ])
            .unwrap();
            assert_eq!(
                crate::native::arguments(args.subcommand().unwrap().1)
                    .get_one::<String>("file")
                    .unwrap(),
                "request.json"
            );
            assert!(
                parse_from([
                    "joog",
                    service.cli,
                    "reader@example.com",
                    "--file",
                    "request.json",
                    method
                ])
                .is_err()
            );
            let help = parse_from(["joog", "help", service.cli, method])
                .unwrap_err()
                .to_string();
            assert!(help.contains("--file <PATH>"));
        }
    }

    #[test]
    fn every_command_defaults_to_plain_and_propagates_explicit_json() {
        let mut commands = SERVICES
            .iter()
            .map(|service| {
                vec![
                    service.cli,
                    "reader@example.com",
                    representative(service.cli),
                    "--file",
                    "request.json",
                ]
            })
            .collect::<Vec<_>>();
        commands.extend([
            vec!["auth", "init"],
            vec!["auth", "enroll"],
            vec!["auth", "test"],
            vec!["auth", "list"],
            vec!["auth", "reader@example.com", "login", "gmail,drive"],
            vec!["auth", "reader@example.com", "logout"],
        ]);
        for command in commands {
            let defaults =
                parse_from(std::iter::once("joog").chain(command.iter().copied())).unwrap();
            assert!(!defaults.get_flag("json"));
            let action = if command[0] == "auth" && command.len() == 2 {
                1
            } else {
                2
            };
            for index in 0..=action {
                let mut args = command.clone();
                args.insert(index, "--json");
                let parsed =
                    parse_from(std::iter::once("joog").chain(args.iter().copied())).unwrap();
                let mut current = &parsed;
                loop {
                    assert!(current.get_flag("json"));
                    if let Some((_, child)) = current.subcommand() {
                        current = child;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    #[test]
    fn definition_and_documentation_match() {
        command().debug_assert();
        let grammar = include_str!("../docs/grammar.md");
        for service in SERVICES {
            assert!(grammar.contains(&format!("| `{}` | {} |", service.cli, service.title)));
        }
        fn check(cmd: &Command, grammar: &str) {
            for arg in cmd.get_arguments() {
                if let Some(long) = arg.get_long() {
                    assert!(
                        grammar.contains(&format!("--{long}")),
                        "undocumented --{long}"
                    );
                }
            }
            for child in cmd.get_subcommands() {
                if !child.get_name().contains('.') {
                    check(child, grammar);
                }
            }
        }
        let mut cli = command();
        cli.build();
        let mut reference = grammar.to_owned();
        for service in SERVICES {
            reference.push_str(
                &std::fs::read_to_string(
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join(format!("docs/services/{}.md", service.cli)),
                )
                .unwrap(),
            );
        }
        check(&cli, &reference);
    }

    #[test]
    fn auth_commands_require_the_appropriate_account_context() {
        for action in ["login", "all", "logout"] {
            assert!(parse_from(["joog", "auth", action]).is_err());
            let mut args = vec!["joog", "auth", "alice@example.com", action];
            if action == "login" {
                args.push("gmail");
            }
            assert!(parse_from(args).is_ok());
            assert!(
                parse_from(["joog", "auth", "alice@example.com", action, "--help"])
                    .unwrap_err()
                    .to_string()
                    .contains(&format!("auth <ACCOUNT> {action}"))
            );
        }
        for action in ["init", "enroll", "test", "list"] {
            assert!(parse_from(["joog", "auth", action]).is_ok());
            assert!(parse_from(["joog", "auth", "alice@example.com", action]).is_err());
        }
        assert_eq!(
            parse_from(["joog", "auth", "help"]).unwrap_err().kind(),
            clap::error::ErrorKind::DisplayHelp
        );
    }

    #[test]
    fn credential_transfers_accept_optional_accounts_and_require_files() {
        for action in ["export", "import"] {
            assert!(parse_from(["joog", "auth", action]).is_err());
            assert!(parse_from(["joog", "auth", "me@example.com", action]).is_err());
            assert!(parse_from(["joog", "auth", action, "credentials.joog"]).is_ok());
            assert!(
                parse_from(["joog", "auth", "me@example.com", action, "credentials.joog"]).is_ok()
            );
            assert!(
                parse_from([
                    "joog",
                    "auth",
                    action,
                    "credentials.joog",
                    "--passphrase-file",
                    "private.txt"
                ])
                .is_ok()
            );
            assert!(
                parse_from([
                    "joog",
                    "auth",
                    action,
                    "credentials.joog",
                    "--passphrase",
                    "secret"
                ])
                .is_err()
            );
            let common = parse_from(["joog", "auth", action, "--help"])
                .unwrap_err()
                .to_string();
            assert!(!common.contains("--passphrase-file"));
            let full = parse_from(["joog", "auth", action, "--help", "--verbose"])
                .unwrap_err()
                .to_string();
            assert!(full.contains("--passphrase-file"));
            assert!(
                full.split("Optional parameters (advanced):")
                    .nth(1)
                    .unwrap()
                    .contains("--passphrase-file")
            );
        }
        assert!(parse_from(["joog", "auth", "init", "--headless"]).is_ok());
        assert!(
            parse_from([
                "joog",
                "auth",
                "import",
                "credentials.joog",
                "--headless",
                "--replace"
            ])
            .is_ok()
        );
        assert!(parse_from(["joog", "auth", "export", "credentials.joog", "--headless"]).is_err());
    }

    #[test]
    fn options_precede_actions_and_accounts_are_context() {
        let parsed = parse_from([
            "joog",
            "mail",
            "alice@example.com",
            "--json",
            "messages.list",
            "--q",
            "is:unread",
        ])
        .unwrap();
        assert!(parsed.get_flag("json"));
        let mail = parsed.subcommand_matches("mail").unwrap();
        assert_eq!(
            mail.get_one::<String>("account").unwrap(),
            "alice@example.com"
        );
        assert_eq!(mail.subcommand_name(), Some("messages.list"));
        for args in [
            vec![
                "joog",
                "mail",
                "alice@example.com",
                "messages.list",
                "--json",
            ],
            vec![
                "joog",
                "drive",
                "alice@example.com",
                "--max",
                "5",
                "files.list",
            ],
            vec![
                "joog",
                "--account",
                "alice@example.com",
                "drive",
                "files.list",
            ],
        ] {
            assert!(parse_from(args.clone()).is_err(), "{args:?}");
        }
        assert!(parse_from(["joog", "drive", "-", "files.list"]).is_ok());
    }

    #[test]
    fn login_requires_a_nonempty_supported_positional_service_list() {
        assert!(parse_from(["joog", "auth", "me@example.com", "login"]).is_err());
        for services in ["", ",", "gmail,", "gmail,,drive", "admin"] {
            assert!(
                parse_from(["joog", "auth", "me@example.com", "login", services]).is_err(),
                "{services}"
            );
        }
        for services in [
            "gmail",
            "gmail,drive",
            "mail",
            "mail,drive",
            "all",
            "all,gmail",
        ] {
            let parsed = parse_from(["joog", "auth", "me@example.com", "login", services]).unwrap();
            assert_eq!(
                parsed
                    .subcommand_matches("auth")
                    .unwrap()
                    .subcommand_matches("login")
                    .unwrap()
                    .get_one::<String>("services")
                    .unwrap(),
                services
            );
        }
        assert!(
            parse_from([
                "joog",
                "auth",
                "me@example.com",
                "login",
                "--services",
                "gmail,drive"
            ])
            .is_err()
        );
    }

    #[test]
    fn authentication_guide_commands_parse() {
        let mut shell = false;
        let mut checked = 0;
        for line in include_str!("../docs/authentication.md").lines() {
            if line == "```sh" {
                shell = true;
                continue;
            }
            if line == "```" {
                shell = false;
                continue;
            }
            if shell && line.starts_with("./bin/joog ") {
                parse_from(line.split_whitespace())
                    .unwrap_or_else(|e| panic!("invalid documented command: {line}\n{e}"));
                checked += 1;
            }
        }
        assert!(checked > 0);
    }

    #[test]
    fn documented_help_paths_do_not_require_accounts_or_credentials() {
        let mut paths = vec![
            vec!["joog", "--help"],
            vec!["joog", "-h"],
            vec!["joog", "help", "auth"],
        ];
        for action in [
            "init", "enroll", "test", "list", "login", "all", "export", "import", "logout",
        ] {
            paths.push(vec!["joog", "help", "auth", action]);
            paths.push(vec!["joog", "auth", "help", action]);
        }
        for action in ["login", "logout"] {
            paths.push(vec!["joog", "auth", "reader@example.com", action, "--help"]);
        }
        for service in SERVICES {
            paths.push(vec!["joog", "help", service.cli]);
            paths.push(vec!["joog", service.cli, "--help"]);
            paths.push(vec![
                "joog",
                service.cli,
                "reader@example.com",
                representative(service.cli),
                "--help",
            ]);
        }
        for path in paths {
            assert_eq!(
                parse_from(path.clone()).unwrap_err().kind(),
                clap::error::ErrorKind::DisplayHelp,
                "{path:?}"
            );
        }
        for version in ["--version", "-V"] {
            assert_eq!(
                parse_from(["joog", version]).unwrap_err().kind(),
                clap::error::ErrorKind::DisplayVersion
            );
        }
    }
}

#[cfg(test)]
mod native_help_tests {
    use super::*;

    #[test]
    fn every_native_command_and_its_options_are_documented_and_have_specific_help() {
        let doc = include_str!("../docs/grammar.md");
        let common = ["file", "fields", "ttl", "max", "max-pages"];
        let mut count = 0;
        for service in SERVICES {
            for cmd in crate::native::commands(service.cli) {
                let prefix = format!("joog {} ACCOUNT {}", service.cli, cmd.get_name());
                let forms = doc
                    .lines()
                    .filter(|line| {
                        *line == prefix
                            || line
                                .strip_prefix(&prefix)
                                .is_some_and(|tail| tail.starts_with(' '))
                    })
                    .collect::<Vec<_>>();
                assert_eq!(forms.len(), 1, "{prefix}");
                assert_eq!(
                    forms[0],
                    crate::native::documentation_form(service.cli, &cmd),
                    "{prefix}: command grammar differs from the parser"
                );
                let help = parse_from(["joog", "help", service.cli, cmd.get_name(), "--verbose"])
                    .unwrap_err();
                assert_eq!(help.kind(), clap::error::ErrorKind::DisplayHelp);
                let help = help.to_string();
                assert!(
                    help.contains(&format!(
                        "joog [OPTIONS] {} <ACCOUNT> {}",
                        service.cli,
                        cmd.get_name()
                    )),
                    "{help}"
                );
                for arg in cmd.get_arguments() {
                    if let Some(long) = arg.get_long() {
                        assert!(
                            help.contains(&format!("--{long}")),
                            "{prefix}: missing help for --{long}"
                        );
                        if !common.contains(&long) {
                            assert!(
                                forms[0].contains(&format!("--{long}")),
                                "{prefix}: undocumented --{long}"
                            );
                        }
                    } else if let Some(labels) = arg.get_value_names() {
                        for label in labels {
                            assert!(
                                forms[0].contains(label.as_str()),
                                "{prefix}: undocumented {label}"
                            );
                        }
                    }
                }
                for option in [
                    "body",
                    "upload",
                    "output",
                    "max",
                    "max-pages",
                    "ttl",
                    "parent",
                    "to",
                    "cc",
                    "bcc",
                    "calendar",
                    "list",
                ] {
                    let advertised = help.contains(&format!("--{option} "));
                    let accepted = cmd
                        .get_arguments()
                        .any(|arg| arg.get_long() == Some(option));
                    assert_eq!(advertised, accepted, "{prefix}: --{option}");
                }
                count += 1;
            }
        }
        assert_eq!(count, 10);
    }
}
