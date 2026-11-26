use anyhow::Result;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    io::{self, Write},
};

pub fn emit(value: &Value, json: bool, compact: bool) -> Result<()> {
    emit_to(value, json, compact, &mut io::stdout().lock())
}

pub fn status(message: &str, value: &Value, json: bool) -> Result<()> {
    status_to(message, value, json, &mut io::stdout().lock())
}

fn status_to(message: &str, value: &Value, json: bool, out: &mut impl Write) -> Result<()> {
    if json {
        emit_to(value, true, false, out)
    } else {
        writeln!(out, "{}", escaped(&Value::String(message.into())))?;
        Ok(())
    }
}

fn emit_to(value: &Value, json: bool, compact: bool, out: &mut impl Write) -> Result<()> {
    if !json {
        plain(value, out, 0)?;
    } else {
        if compact {
            serde_json::to_writer(&mut *out, value)?;
        } else {
            serde_json::to_writer_pretty(&mut *out, value)?;
        }
        writeln!(out)?;
    }
    Ok(())
}

fn escaped(value: &Value) -> String {
    match value {
        Value::String(text) => escaped_text(text),
        Value::Null => String::new(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::Array(values) => values.iter().map(escaped).collect::<Vec<_>>().join(", "),
        Value::Object(fields) => fields
            .iter()
            .map(|(key, value)| format!("{}: {}", escaped_text(key), escaped(value)))
            .collect::<Vec<_>>()
            .join("; "),
    }
}

fn escaped_text(raw: &str) -> String {
    raw.chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect(),
            '\n' => "\\n".chars().collect(),
            '\r' => "\\r".chars().collect(),
            c if c.is_control()
                || matches!(c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}') =>
            {
                format!("\\u{:04x}", c as u32).chars().collect()
            }
            c => vec![c],
        })
        .collect()
}

fn plain(value: &Value, out: &mut impl Write, indent: usize) -> Result<()> {
    let array = value.as_array().or_else(|| {
        let object = value.as_object()?;
        let mut data = object.iter().filter(|(key, _)| {
            !matches!(
                key.as_str(),
                "kind" | "etag" | "nextPageToken" | "nextSyncToken" | "resultSizeEstimate"
            )
        });
        let (_, collection) = data.next()?;
        if data.next().is_none() {
            collection.as_array()
        } else {
            None
        }
    });
    let padding = " ".repeat(indent);
    if let Some(rows) = array {
        if rows.iter().all(Value::is_object) {
            let columns = rows
                .iter()
                .filter_map(Value::as_object)
                .flat_map(|row| row.keys())
                .collect::<BTreeSet<_>>();
            let mut columns = columns.into_iter().collect::<Vec<_>>();
            columns.sort_by_key(|name| match name.as_str() {
                "id" => 0,
                "name" => 1,
                "title" => 2,
                "type" => 3,
                _ => 4,
            });
            if !columns.is_empty() {
                writeln!(
                    out,
                    "{padding}{}",
                    columns
                        .iter()
                        .map(|name| escaped_text(name))
                        .collect::<Vec<_>>()
                        .join("\t")
                )?;
            }
            for row in rows {
                let row = row.as_object().expect("object rows checked above");
                writeln!(
                    out,
                    "{padding}{}",
                    columns
                        .iter()
                        .map(|key| escaped(row.get(*key).unwrap_or(&Value::Null)))
                        .collect::<Vec<_>>()
                        .join("\t")
                )?;
            }
        } else {
            for row in rows {
                if let Some(cells) = row.as_array() {
                    writeln!(
                        out,
                        "{padding}{}",
                        cells.iter().map(escaped).collect::<Vec<_>>().join("\t")
                    )?;
                } else {
                    plain(row, out, indent)?;
                }
            }
        }
    } else if let Some(object) = value.as_object() {
        for (key, value) in object {
            let key = escaped_text(key);
            if key == "text" && value.as_str().is_some_and(|text| text.contains('\n')) {
                writeln!(out, "{padding}{key}:")?;
                for line in value.as_str().unwrap().lines() {
                    writeln!(out, "{padding}  {}", escaped_text(line))?;
                }
            } else if value.is_array() || value.is_object() {
                writeln!(out, "{padding}{key}:")?;
                plain(value, out, indent + 2)?;
            } else {
                writeln!(out, "{padding}{key}\t{}", escaped(value))?;
            }
        }
    } else {
        writeln!(out, "{padding}{}", escaped(value))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plain_output_has_no_json_fallback_for_nested_response_shapes() {
        let mut responses = vec![
            json!({}),
            json!([]),
            json!({"unrecognizedCollection":[{"name":"entry","nested":{"enabled":true,"tags":["one","two"]}}],"nextPageToken":"cursor"}),
            json!({"first":[{"name":"one"}],"second":[{"name":"two"}],"details":{"active":true}}),
            json!([{"name":"entry"},"text",42,true,null,["cell",{"nested":"value"}]]),
            json!({"spreadsheetId":"sheet","values":[["name",1,true,null],["second",2,false,"value"]]}),
        ];
        for _depth in 0..4 {
            for response in &responses {
                for paginated in [false, true] {
                    let mut bytes = Vec::new();
                    emit_to(response, false, paginated, &mut bytes).unwrap();
                    let text = String::from_utf8(bytes).unwrap();
                    assert!(
                        !text.contains(['{', '}', '[', ']', '"']),
                        "JSON leaked into plain output: {text}"
                    );
                    let mut bytes = Vec::new();
                    emit_to(response, true, paginated, &mut bytes).unwrap();
                    assert_eq!(&serde_json::from_slice::<Value>(&bytes).unwrap(), response);
                }
            }
            responses = responses
                .into_iter()
                .map(|value| json!({"section":[{"value":value}]}))
                .collect();
        }
    }

    #[test]
    fn gmail_labels_are_rows_in_plain_output_and_unchanged_in_json() {
        let value = json!({"labels":[
            {"id":"INBOX","name":"INBOX","type":"system"},
            {"id":"CHAT","name":"CHAT","type":"system","labelListVisibility":"labelHide","messageListVisibility":"hide"},
            {"id":"Label_1","name":"Projects","type":"user","color":{"backgroundColor":"#ffffff","textColor":"#000000"}}
        ]});
        let mut bytes = Vec::new();
        emit_to(&value, false, false, &mut bytes).unwrap();
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            "id\tname\ttype\tcolor\tlabelListVisibility\tmessageListVisibility\nINBOX\tINBOX\tsystem\t\t\t\nCHAT\tCHAT\tsystem\t\tlabelHide\thide\nLabel_1\tProjects\tuser\tbackgroundColor: #ffffff; textColor: #000000\t\t\n"
        );
        let mut bytes = Vec::new();
        emit_to(&value, true, false, &mut bytes).unwrap();
        assert_eq!(serde_json::from_slice::<Value>(&bytes).unwrap(), value);
    }

    #[test]
    fn collections_are_detected_without_service_specific_field_names() {
        let mut bytes = Vec::new();
        emit_to(
            &json!({"notes":[{"id":"n1","title":"Checklist"}],"nextPageToken":"cursor"}),
            false,
            false,
            &mut bytes,
        )
        .unwrap();
        assert_eq!(bytes, b"id\ttitle\nn1\tChecklist\n");
    }

    #[test]
    fn nested_records_keep_their_fields_and_never_fall_back_to_json() {
        let value = json!({"id":"m1","labelIds":["INBOX","UNREAD"],"payload":{"headers":[{"name":"From","value":"sender@example.com"}],"mimeType":"text/plain"}});
        let mut bytes = Vec::new();
        emit_to(&value, false, false, &mut bytes).unwrap();
        assert_eq!(bytes, b"id\tm1\nlabelIds:\n  INBOX\n  UNREAD\npayload:\n  headers:\n    name\tvalue\n    From\tsender@example.com\n  mimeType\ttext/plain\n");
        assert_eq!(
            escaped(&json!({"items":[{"text":"unsafe\n\u{1b}[2J"}]})),
            "items: text: unsafe\\n\\u001b[2J"
        );
    }

    #[test]
    fn status_uses_a_sentence_by_default_and_preserves_explicit_json() {
        let value = json!({"google":"reachable","oauth_client":"not_rejected"});
        let mut bytes = Vec::new();
        status_to("Google connection successful.", &value, false, &mut bytes).unwrap();
        assert_eq!(bytes, b"Google connection successful.\n");
        bytes.clear();
        status_to("Google connection successful.", &value, true, &mut bytes).unwrap();
        assert_eq!(serde_json::from_slice::<Value>(&bytes).unwrap(), value);
        bytes.clear();
        status_to(
            "Signed out forged\u{1b}[2J@example.com.",
            &value,
            false,
            &mut bytes,
        )
        .unwrap();
        assert_eq!(bytes, b"Signed out forged\\u001b[2J@example.com.\n");
    }

    #[test]
    fn default_output_is_tsv_including_paginated_responses() {
        let value = json!({"items":[{"name":"one"}],"nextPageToken":"next"});
        for paginated in [false, true] {
            let mut bytes = Vec::new();
            emit_to(&value, false, paginated, &mut bytes).unwrap();
            assert_eq!(bytes, b"name\none\n");
        }
    }

    #[test]
    fn json_option_preserves_response_and_uses_one_line_per_page() {
        let value = json!({"items":[{"name":"one"}],"nextPageToken":"next"});
        for paginated in [false, true] {
            let mut bytes = Vec::new();
            emit_to(&value, true, paginated, &mut bytes).unwrap();
            assert_eq!(serde_json::from_slice::<Value>(&bytes).unwrap(), value);
            if paginated {
                assert_eq!(bytes.iter().filter(|&&b| b == b'\n').count(), 1);
            }
        }
    }

    #[test]
    fn table_escapes_terminal_sequences_and_multiline_fields() {
        let mut bytes = Vec::new();
        emit_to(
            &json!({"items":[{"name":"x\t\n\u{1b}[2J"}]}),
            false,
            false,
            &mut bytes,
        )
        .unwrap();
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            "name\nx\\t\\n\\u001b[2J\n"
        );
    }
}
