use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde_json::{Value, json};

use crate::output;

fn collect_text(value: &Value, key: &str, out: &mut String) {
    match value {
        Value::Object(object) => {
            for (name, child) in object {
                if name == key && child.is_string() {
                    out.push_str(child.as_str().unwrap());
                } else {
                    collect_text(child, key, out);
                }
            }
        }
        Value::Array(children) => {
            for child in children {
                collect_text(child, key, out);
            }
        }
        _ => {}
    }
}
fn mime_text(part: &Value, out: &mut String) -> Result<()> {
    if part["mimeType"] == "text/plain" && part["filename"].as_str().is_none_or(str::is_empty) {
        if let Some(data) = part.pointer("/body/data").and_then(Value::as_str) {
            let bytes = URL_SAFE_NO_PAD
                .decode(data.trim_end_matches('='))
                .context("Google returned invalid message body encoding")?;
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&String::from_utf8_lossy(&bytes));
        }
    } else if let Some(parts) = part["parts"].as_array() {
        for part in parts {
            mime_text(part, out)?;
        }
    }
    Ok(())
}
fn message(value: &Value) -> Result<Value> {
    let mut result = serde_json::Map::new();
    result.insert("id".into(), value["id"].clone());
    if let Some(headers) = value.pointer("/payload/headers").and_then(Value::as_array) {
        for header in headers {
            let name = header["name"]
                .as_str()
                .unwrap_or_default()
                .to_ascii_lowercase();
            if ["date", "from", "to", "cc", "bcc", "subject"].contains(&name.as_str()) {
                result.insert(name, header["value"].clone());
            }
        }
    }
    let mut body = String::new();
    mime_text(&value["payload"], &mut body)?;
    if body.is_empty() {
        body = value["snippet"].as_str().unwrap_or_default().into();
    }
    result.insert("text".into(), json!(body));
    Ok(Value::Object(result))
}

fn compact(mut value: Value) -> Value {
    if let Some(fields) = value.as_object_mut() {
        fields.retain(|_, value| !value.is_null() && !value.as_array().is_some_and(Vec::is_empty));
    }
    value
}
fn rows(value: &Value, key: &str, view: impl Fn(&Value) -> Value) -> Value {
    value[key]
        .as_array()
        .map(|rows| Value::Array(rows.iter().map(view).collect()))
        .unwrap_or_else(|| value.clone())
}
fn contact(value: &Value) -> Value {
    let entries = |key: &str| {
        value[key]
            .as_array()
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| entry["value"].as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    };
    compact(
        json!({"id":value["resourceName"],"name":value.pointer("/names/0/displayName"),"email":entries("emailAddresses"),"phone":entries("phoneNumbers")}),
    )
}
fn event(value: &Value) -> Value {
    let endpoint = |key: &str| {
        value[key]["dateTime"]
            .as_str()
            .or_else(|| value[key]["date"].as_str())
    };
    compact(
        json!({"id":value["id"],"title":value["summary"],"start":endpoint("start"),"end":endpoint("end"),"location":value["location"],"status":value["status"],"description":value["description"],"url":value["htmlLink"]}),
    )
}
fn task(value: &Value) -> Value {
    compact(
        json!({"id":value["id"],"title":value["title"],"due":value["due"].as_str().map(|due|due.split('T').next().unwrap_or(due)),"status":value["status"],"notes":value["notes"]}),
    )
}
fn video(value: &Value) -> Value {
    let id = value["id"]
        .as_str()
        .or_else(|| value.pointer("/id/videoId").and_then(Value::as_str))
        .or_else(|| value.pointer("/id/channelId").and_then(Value::as_str))
        .or_else(|| value.pointer("/id/playlistId").and_then(Value::as_str));
    compact(
        json!({"id":id,"title":value.pointer("/snippet/title"),"channel":value.pointer("/snippet/channelTitle"),"published":value.pointer("/snippet/publishedAt"),"video":value.pointer("/contentDetails/videoId"),"description":value.pointer("/snippet/description")}),
    )
}
fn photo(value: &Value) -> Value {
    compact(
        json!({"id":value["id"],"name":value["filename"],"type":value["mimeType"],"created":value.pointer("/mediaMetadata/creationTime"),"url":value["productUrl"]}),
    )
}
fn album(value: &Value) -> Value {
    compact(
        json!({"id":value["id"],"title":value["title"],"items":value["mediaItemsCount"],"url":value["productUrl"]}),
    )
}

pub fn plain_value(service: &str, action: &str, value: &Value) -> Result<Value> {
    Ok(match (service, action) {
        ("mail", "labels") => rows(value, "labels", |label| {
            compact(json!({"id":label["id"],"name":label["name"],"type":label["type"]}))
        }),
        ("calendar", "calendars") => rows(value, "items", |calendar| {
            compact(
                json!({"id":calendar["id"],"name":calendar["summary"],"timezone":calendar["timeZone"],"access":calendar["accessRole"],"primary":calendar["primary"]}),
            )
        }),
        ("calendar", "events") => rows(value, "items", |value| {
            let mut row = event(value);
            row.as_object_mut().unwrap().remove("description");
            row
        }),
        ("calendar", "get") => event(value),
        ("tasks", "lists") => rows(value, "items", |list| {
            compact(json!({"id":list["id"],"title":list["title"]}))
        }),
        ("tasks", "list") => rows(value, "items", task),
        ("tasks", "get") => task(value),
        ("contacts", "list") => rows(value, "connections", contact),
        ("contacts", "search") => rows(value, "results", |result| contact(&result["person"])),
        ("contacts", "get") => contact(value),
        ("youtube", "search" | "get" | "playlists" | "playlist" | "subscriptions") => {
            rows(value, "items", |value| {
                let mut row = video(value);
                if action != "get" {
                    row.as_object_mut().unwrap().remove("description");
                }
                row
            })
        }
        ("photos", "list") => rows(value, "mediaItems", photo),
        ("photos", "get") => photo(value),
        ("photos", "albums") => rows(value, "albums", album),
        ("photos", "album") => album(value),
        ("mail", "get") => message(value)?,
        ("mail", "thread") => {
            json!({"id":value["id"],"messages":value["messages"].as_array().map(|messages|messages.iter().map(message).collect::<Result<Vec<_>>>()).transpose()?.unwrap_or_default()})
        }
        ("sheets", "values") => value.get("values").cloned().unwrap_or_else(|| json!([])),
        ("sheets", "get") if value.get("values").is_some() => value["values"].clone(),
        ("sheets", "get") => compact(
            json!({"id":value["spreadsheetId"],"title":value.pointer("/properties/title"),"url":value["spreadsheetUrl"],"sheets":value["sheets"].as_array().map(|sheets|sheets.iter().map(|sheet|compact(json!({"id":sheet.pointer("/properties/sheetId"),"title":sheet.pointer("/properties/title"),"rows":sheet.pointer("/properties/gridProperties/rowCount"),"columns":sheet.pointer("/properties/gridProperties/columnCount")}))).collect::<Vec<_>>())}),
        ),
        ("docs", "get") => {
            let mut text = String::new();
            collect_text(value, "content", &mut text);
            json!({"id":value["documentId"],"title":value["title"],"text":text})
        }
        ("slides", "get") => {
            let slides = value["slides"]
                .as_array()
                .map(|slides| {
                    slides
                        .iter()
                        .enumerate()
                        .map(|(index, slide)| {
                            let mut text = String::new();
                            collect_text(slide, "content", &mut text);
                            json!({"slide":index+1,"id":slide["objectId"],"text":text})
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            json!({"id":value["presentationId"],"title":value["title"],"slides":slides})
        }
        _ => value.clone(),
    })
}
fn success(service: &str, action: &str) -> Option<&'static str> {
    Some(match (service, action) {
        ("mail", "send") => "Message sent.",
        ("mail", "trash") => "Message moved to Trash.",
        ("mail", "restore") => "Message restored.",
        ("mail", "read") => "Message marked as read.",
        ("mail", "unread") => "Message marked as unread.",
        ("mail", "star") => "Message starred.",
        ("mail", "unstar") => "Message unstarred.",
        ("drive", "upload") => "File uploaded.",
        ("drive", "mkdir") => "Folder created.",
        ("drive", "rename") => "File renamed.",
        ("drive", "copy") => "File copied.",
        ("drive", "trash") => "File moved to Trash.",
        ("drive", "restore") => "File restored.",
        ("calendar", "create") => "Event created.",
        ("calendar", "update") => "Event updated.",
        ("calendar", "delete") => "Event deleted.",
        ("sheets", "create") => "Spreadsheet created.",
        ("sheets", "write") => "Cells updated.",
        ("sheets", "append") => "Rows appended.",
        ("sheets", "clear") => "Cells cleared.",
        ("docs", "create") => "Document created.",
        ("docs", "append") => "Text appended.",
        ("slides", "create") => "Presentation created.",
        ("forms", "create") => "Form created.",
        ("tasks", "create-list") => "Task list created.",
        ("tasks", "add") => "Task created.",
        ("tasks", "update") => "Task updated.",
        ("tasks", "complete") => "Task completed.",
        ("tasks", "reopen") => "Task reopened.",
        ("tasks", "delete") => "Task deleted.",
        ("contacts", "create") => "Contact created.",
        ("contacts", "delete") => "Contact deleted.",
        ("youtube", "create-playlist") => "Playlist created.",
        ("youtube", "add") => "Video added to playlist.",
        ("youtube", "remove") => "Playlist entry removed.",
        ("photos", "create-album") => "Album created.",
        _ => return None,
    })
}
pub(crate) fn method_action<'a>(service: &str, action: &'a str) -> &'a str {
    match (service, action) {
        ("mail", "messages.get") => "get",
        ("mail", "threads.get") => "thread",
        ("mail", "labels.list") => "labels",
        ("mail", "messages.send" | "drafts.send") => "send",
        ("mail", "messages.trash") => "trash",
        ("mail", "messages.untrash") => "restore",
        ("calendar", "calendars.list") => "calendars",
        ("calendar", "events.list") => "events",
        ("calendar", "events.get") => "get",
        ("sheets", "values.get") => "values",
        ("sheets", "get") => "get",
        ("docs", "get") => "get",
        ("slides", "get") => "get",
        ("tasks", "tasklists.list") => "lists",
        ("tasks", "tasks.list") => "list",
        ("tasks", "tasks.get") => "get",
        ("contacts", "people.connections.list") => "list",
        ("contacts", "people.search-contacts") => "search",
        ("contacts", "people.get") => "get",
        ("youtube", "videos.list") => "get",
        ("youtube", "search.list") => "search",
        ("youtube", "playlist-items.list") => "playlist",
        ("youtube", "playlists.list") => "playlists",
        ("youtube", "subscriptions.list") => "subscriptions",
        ("photos", "media-items.get") => "get",
        ("photos", "media-items.list" | "media-items.search") => "list",
        ("photos", "albums.get") => "album",
        ("photos", "albums.list") => "albums",
        _ => action,
    }
}

pub fn emit(
    service: &str,
    action: Option<&str>,
    value: &Value,
    json: bool,
    paginated: bool,
) -> Result<()> {
    if json {
        return output::emit(value, true, paginated);
    }
    if let Some(action) = action {
        if crate::schema_commands::contract(service)
            .document
            .root
            .methods
            .get(action)
            .is_some_and(|method| method.batch_operation.is_some())
        {
            return output::status(
                if service == "docs" {
                    "Document updated."
                } else {
                    "Presentation updated."
                },
                value,
                false,
            );
        }
        let action = method_action(service, action);
        if let Some(message) = success(service, action) {
            let id = [
                "id",
                "name",
                "documentId",
                "presentationId",
                "spreadsheetId",
                "formId",
            ]
            .iter()
            .find_map(|key| value[*key].as_str());
            let message = match id {
                Some(id) => format!("{message} ID: {id}"),
                None => message.into(),
            };
            return output::status(&message, value, false);
        }
        return output::emit(&plain_value(service, action, value)?, false, paginated);
    }
    if value.is_null() {
        output::status("Done.", value, false)
    } else {
        output::emit(value, false, paginated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_lists_show_actionable_ids_and_details_without_transport_metadata() {
        let cases = [
            (
                "mail",
                "labels",
                json!({"labels":[{"id":"INBOX","name":"INBOX","type":"system","labelListVisibility":"labelShow"}]}),
                "INBOX",
            ),
            (
                "calendar",
                "events",
                json!({"kind":"calendar#events","items":[{"id":"event","summary":"Meeting","start":{"dateTime":"2026-12-01T10:00:00Z"},"end":{"dateTime":"2026-12-01T11:00:00Z"}}]}),
                "event",
            ),
            (
                "tasks",
                "list",
                json!({"etag":"transport","items":[{"id":"task","title":"Review","due":"2026-12-01T00:00:00Z","status":"needsAction"}]}),
                "task",
            ),
            (
                "contacts",
                "list",
                json!({"connections":[{"resourceName":"people/contact","etag":"transport","names":[{"displayName":"Alice"}],"emailAddresses":[{"value":"alice@example.com"}]}]}),
                "people/contact",
            ),
            (
                "youtube",
                "search",
                json!({"kind":"youtube#searchListResponse","items":[{"id":{"kind":"youtube#video","videoId":"video"},"snippet":{"title":"Example","channelTitle":"Channel","thumbnails":{"default":{"url":"transport"}}}}]}),
                "video",
            ),
            (
                "photos",
                "list",
                json!({"mediaItems":[{"id":"photo","filename":"photo.jpg","mimeType":"image/jpeg","baseUrl":"transport","productUrl":"https://photos.google.com/photo"}]}),
                "photo",
            ),
        ];
        for (service, action, response, id) in cases {
            let view = plain_value(service, action, &response).unwrap();
            assert_eq!(view[0]["id"], id, "{service} {action}");
            let text = serde_json::to_string(&view).unwrap();
            assert!(!text.contains("transport"), "{service} {action}: {text}");
        }
    }
    #[test]
    fn reads_decode_content_and_keep_json_fields_untouched() {
        let mail = json!({"id":"message","snippet":"preview","payload":{"mimeType":"multipart/mixed","headers":[{"name":"Subject","value":"Hello"}],"parts":[{"mimeType":"text/plain","body":{"data":URL_SAFE_NO_PAD.encode("Hello\nworld")}}, {"mimeType":"text/plain","filename":"secret.txt","body":{"data":URL_SAFE_NO_PAD.encode("attachment")}}]}});
        let view = plain_value("mail", "get", &mail).unwrap();
        assert_eq!(view["subject"], "Hello");
        assert_eq!(view["text"], "Hello\nworld");
        assert!(mail["payload"].is_object());
        let doc = json!({"documentId":"doc","title":"Title","body":{"content":[{"paragraph":{"elements":[{"textRun":{"content":"Readable text"}}]}}]}});
        assert_eq!(
            plain_value("docs", "get", &doc).unwrap()["text"],
            "Readable text"
        );
        let cells = json!({"values":[["Name",1],["Second",2]],"range":"A1:B2"});
        assert_eq!(
            plain_value("sheets", "get", &cells).unwrap(),
            cells["values"]
        );
    }
    #[test]
    fn cleared_sheet_ranges_return_an_empty_row_list() {
        let action = method_action("sheets", "values.get");
        for response in [
            json!({"range":"Fixture!A1:D3","majorDimension":"ROWS"}),
            json!({"values":[]}),
            json!({"values":[["text"]]}),
        ] {
            assert_eq!(
                plain_value("sheets", action, &response).unwrap(),
                response.get("values").cloned().unwrap_or_else(|| json!([]))
            );
        }
        assert_eq!(
            plain_value(
                "sheets",
                method_action("sheets", "get"),
                &json!({"spreadsheetId":"sheet","properties":{"title":"Title"}})
            )
            .unwrap()["title"],
            "Title"
        );
    }
    #[test]
    fn every_write_command_has_a_human_confirmation() {
        for service in crate::services::SERVICES {
            for cmd in crate::native::commands(service.cli) {
                if !cmd
                    .get_arguments()
                    .any(|arg| arg.get_id() == "ttl" || arg.get_id() == "output")
                {
                    assert!(
                        success(service.cli, cmd.get_name()).is_some(),
                        "{} {}",
                        service.cli,
                        cmd.get_name()
                    );
                }
            }
        }
    }
}
