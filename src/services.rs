use anyhow::Result;
use serde_json::{Map, Value};

pub struct Service {
    pub cli: &'static str,
    pub api: &'static str,
    pub version: &'static str,
    pub title: &'static str,
    pub scope: &'static str,
}

macro_rules! services {
    ($($cli:literal, $api:literal, $version:literal, $title:literal, $scope:literal;)*) => {
        pub const SERVICES: &[Service] = &[$(Service { cli: $cli, api: $api, version: $version, title: $title, scope: $scope }),*];
    };
}

services! {
    "mail", "gmail", "v1", "Gmail", "https://mail.google.com/";
    "drive", "drive", "v3", "Google Drive", "drive";
    "calendar", "calendar", "v3", "Google Calendar", "calendar";
    "sheets", "sheets", "v4", "Google Sheets", "spreadsheets";
    "docs", "docs", "v1", "Google Docs", "documents";
    "slides", "slides", "v1", "Google Slides", "presentations";
    "forms", "forms", "v1", "Google Forms", "forms.body";
    "tasks", "tasks", "v1", "Google Tasks", "tasks";
    "contacts", "people", "v1", "Google Contacts", "contacts";
    "youtube", "youtube", "v3", "YouTube", "youtube";
    "photos", "photoslibrary", "v1", "Google Photos app-created media", "photoslibrary.appendonly";
}

pub fn scopes(services: &str) -> Result<Vec<String>> {
    let mut scopes = Vec::new();
    for name in services.split(',') {
        let name = name.trim();
        let name = if name == "gmail" { "mail" } else { name };
        let selected_services = if name == "all" {
            SERVICES.iter().collect::<Vec<_>>()
        } else {
            vec![
                SERVICES
                    .iter()
                    .find(|service| service.cli == name)
                    .ok_or_else(|| anyhow::anyhow!("unsupported OAuth service {name}"))?,
            ]
        };
        for service in selected_services {
            let mut selected = vec![service.scope];
            match service.cli {
                "forms" => selected.push("forms.responses.readonly"),
                "contacts" => selected.extend(["contacts.other.readonly", "directory.readonly"]),
                "youtube" => selected.extend(["youtube.force-ssl", "youtube.readonly"]),
                "photos" => selected.extend([
                    "photoslibrary.readonly.appcreateddata",
                    "photoslibrary.edit.appcreateddata",
                ]),
                _ => {}
            }
            for scope in selected {
                let scope = if scope.starts_with("https://") {
                    scope.to_owned()
                } else {
                    format!("https://www.googleapis.com/auth/{scope}")
                };
                if !scopes.contains(&scope) {
                    scopes.push(scope);
                }
            }
        }
    }
    Ok(scopes)
}

pub struct Invocation {
    pub options: Option<clap::ArgMatches>,
    pub method: String,
    pub params: Map<String, Value>,
    pub body: Option<Value>,
    pub custom_fields: bool,
    pub relative_times: Vec<&'static str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oauth_scopes_come_from_selected_services() {
        assert_eq!(
            scopes("gmail,drive").unwrap(),
            scopes("mail,drive").unwrap()
        );
        assert_eq!(
            scopes("mail, drive").unwrap(),
            [
                "https://mail.google.com/",
                "https://www.googleapis.com/auth/drive",
            ]
        );
        assert!(scopes("").is_err());
        assert!(scopes("https://www.googleapis.com/auth/drive").is_err());
    }

    #[test]
    fn permissions_cover_service_reads_and_deduplicate_overlapping_names() {
        assert_eq!(
            scopes("gmail,mail,drive,drive").unwrap(),
            scopes("mail,drive").unwrap()
        );
        assert!(
            scopes("forms")
                .unwrap()
                .iter()
                .any(|scope| scope.ends_with("/forms.responses.readonly"))
        );
        assert!(
            scopes("photos")
                .unwrap()
                .iter()
                .any(|scope| scope.ends_with("/photoslibrary.readonly.appcreateddata"))
        );
    }

    #[test]
    fn all_selects_every_service_and_its_additional_permissions_once() {
        let explicit = SERVICES
            .iter()
            .map(|service| service.cli)
            .collect::<Vec<_>>()
            .join(",");
        let all = scopes("all").unwrap();
        assert_eq!(all, scopes(&explicit).unwrap());
        assert_eq!(all, scopes("all,gmail,drive,all").unwrap());
        assert_eq!(
            all.len(),
            all.iter().collect::<std::collections::BTreeSet<_>>().len()
        );
        for suffix in [
            "forms.responses.readonly",
            "contacts.other.readonly",
            "directory.readonly",
            "youtube.force-ssl",
            "youtube.readonly",
            "photoslibrary.readonly.appcreateddata",
            "photoslibrary.edit.appcreateddata",
        ] {
            assert!(
                all.iter().any(|scope| scope.ends_with(suffix)),
                "all omitted {suffix}"
            );
        }
        assert!(scopes("all,unknown-service").is_err());
    }
}
