use super::{Request, backend::Backend, values};
use anyhow::{Context, Result, bail, ensure};
use jiff::{SignedDuration, Timestamp, ToSpan, ZonedDateTime, tz::TimeZone};
use serde_json::{Map, Value, json};

mod mail;

fn parse_instant(text: &str) -> Result<Timestamp> {
    text.parse().context("Google returned an invalid timestamp")
}

fn rows<'a>(value: &'a Value, key: &str) -> &'a [Value] {
    value[key].as_array().map(Vec::as_slice).unwrap_or_default()
}
fn id<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .with_context(|| format!("Google did not return {field}"))
}
fn view(service: &str, command: &str, value: Value) -> Result<Value> {
    crate::presentation::plain_value(
        service,
        crate::presentation::method_action(service, command),
        &value,
    )
}
fn changed(value: Value, message: &str) -> Value {
    if value.is_null() {
        json!({"status":message})
    } else {
        let mut value = value;
        if let Some(object) = value.as_object_mut() {
            object.insert("confirmation".into(), json!(message));
        }
        value
    }
}
fn escaped(text: &str) -> String {
    text.replace('\\', "\\\\").replace('\'', "\\'")
}
fn file_type(text: &str) -> &str {
    match text {
        "folder" => "application/vnd.google-apps.folder",
        "document" => "application/vnd.google-apps.document",
        "spreadsheet" => "application/vnd.google-apps.spreadsheet",
        "presentation" => "application/vnd.google-apps.presentation",
        "form" => "application/vnd.google-apps.form",
        "pdf" => "application/pdf",
        other => other,
    }
}
fn list_files(r: &Request, api: &mut impl Backend, mime: Option<&str>) -> Result<Value> {
    let mut terms = vec!["trashed = false".to_owned()];
    if let Some(folder) = r
        .text("folder")
        .or((r.wrapper == "jdrive" && r.command == "ls").then_some("root"))
    {
        terms.push(format!("'{}' in parents", escaped(folder)));
    }
    if let Some(mime) = mime.or_else(|| r.text("type").map(file_type)) {
        terms.push(if mime == "image" {
            "mimeType contains 'image/'".into()
        } else {
            format!("mimeType = '{}'", escaped(mime))
        });
    }
    if let Some(query) = r.text("query") {
        terms.push(format!("fullText contains '{}'", escaped(query)));
    }
    if let Some(name) = r.text("name") {
        terms.push(format!("name contains '{}'", escaped(name)));
    }
    let value = api.call("drive", "files.list", r.list_params(json!({"q":terms.join(" and "),"fields":"nextPageToken,files(id,name,mimeType,modifiedTime,webViewLink,parents)","orderBy":"modifiedTime desc"})))?;
    view("drive", "files.list", value)
}

fn place(api: &mut impl Backend, target: &str, folder: &str, name: Option<&str>) -> Result<Value> {
    let original = api.call(
        "drive",
        "files.get",
        json!({"fileId":target,"fields":"id,parents"}),
    )?;
    let parents = rows(&original, "parents")
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>();
    let mut params = json!({"fileId":target});
    if !parents.contains(&folder) {
        params["addParents"] = json!(folder);
        if !parents.is_empty() {
            params["removeParents"] = json!(parents.join(","));
        }
    }
    if let Some(name) = name {
        params["name"] = json!(name);
    }
    // Moving to the current parent with no rename needs no mutation.
    if params.as_object().unwrap().len() == 1 {
        return Ok(changed(original, "File is already in this folder."));
    }
    params["body"] = json!({});
    api.call("drive", "files.update", params)
}

fn export(r: &Request, api: &mut impl Backend) -> Result<Value> {
    api.call("drive", "files.export", json!({"fileId":r.target(),"mimeType":values::mime(r.text("format").unwrap_or("pdf"))?,"output":r.text("out").unwrap()}))
}

fn share(r: &Request, api: &mut impl Backend) -> Result<Value> {
    api.call("drive", "permissions.create", json!({"fileId":r.target(),"type":"user","role":r.text("role").unwrap_or("reader"),"emailAddress":r.text("to").unwrap(),"sendNotificationEmail":r.params.get("notify").and_then(Value::as_bool).unwrap_or(true)}))
}

pub fn execute(r: &Request, api: &mut impl Backend) -> Result<Value> {
    match r.wrapper.as_str() {
        "jmail" => mail::execute(r, api),
        "jdrive" => drive(r, api),
        "jcal" => calendar(r, api),
        "jsheets" => sheets(r, api),
        "jdocs" => docs(r, api),
        "jslides" => slides(r, api),
        "jforms" => forms(r, api),
        "jtasks" => tasks(r, api),
        "jcontacts" => contacts(r, api),
        "jyt" => youtube(r, api),
        "jphotos" => photos(r, api),
        _ => bail!("unknown wrapper"),
    }
}

fn drive(r: &Request, api: &mut impl Backend) -> Result<Value> {
    Ok(match r.command.as_str() {
        "ls" | "find" => return list_files(r, api, None),
        "info" => api.call("drive", "files.get", json!({"fileId":r.target(),"fields":"id,name,mimeType,size,parents,owners,permissions,webViewLink"}))?,
        "get" => {
            let file = api.call("drive", "files.get", json!({"fileId":r.target(),"fields":"id,mimeType"}))?;
            if file["mimeType"].as_str().is_some_and(|mime| mime.starts_with("application/vnd.google-apps.")) { return export(r, api); }
            ensure!(r.text("format").is_none(), "format: is available for native Google files; omit it to download this file");
            api.call("drive", "files.get", json!({"fileId":r.target(),"output":r.text("out").unwrap()}))?
        }
        "put" => {
            let path = r.text("path").unwrap();
            let name = r.text("name").map(str::to_owned).unwrap_or_else(|| std::path::Path::new(path).file_name().unwrap().to_string_lossy().into_owned());
            let mut params = json!({"name":name,"upload":path,"contentType":values::upload_mime(path)});
            if let Some(folder) = r.text("folder") { params["parents"] = json!([folder]); }
            changed(api.call("drive", "files.create", params)?, "File uploaded.")
        }
        "mkdir" => {
            let mut params = json!({"name":r.text("name").unwrap(),"mimeType":"application/vnd.google-apps.folder"});
            if let Some(folder) = r.text("folder") { params["parents"] = json!([folder]); }
            changed(api.call("drive", "files.create", params)?, "Folder created.")
        }
        "move" => {
            let value = if let Some(folder) = r.text("to") { place(api, r.target(), folder, r.text("name"))? }
            else { api.call("drive", "files.update", json!({"fileId":r.target(),"name":r.text("name").unwrap()}))? };
            changed(value, "File updated.")
        }
        "copy" => {
            let mut params = json!({"fileId":r.target(),"body":{}});
            if let Some(folder) = r.text("to") { params["parents"] = json!([folder]); }
            if let Some(name) = r.text("name") { params["name"] = json!(name); }
            changed(api.call("drive", "files.copy", params)?, "File copied.")
        }
        "share" => changed(share(r, api)?, "Access granted."),
        "trash" | "restore" => changed(api.call("drive", "files.update", json!({"fileId":r.target(),"trashed":r.command == "trash"}))?, if r.command == "trash" { "File moved to Trash." } else { "File restored." }),
        _ => bail!("unknown Drive command"),
    })
}

fn calendar_zone(r: &Request, api: &mut impl Backend) -> Result<TimeZone> {
    if let Some(zone) = r.text("timezone") {
        return TimeZone::get(zone).context("unknown timezone");
    }
    let calendar = api.call(
        "calendar",
        "calendars.get",
        json!({"calendarId":r.text("calendar").unwrap_or("primary")}),
    )?;
    TimeZone::get(id(&calendar, "timeZone")?)
        .context("Google returned an unknown calendar timezone; supply timezone:")
}

fn calendar(r: &Request, api: &mut impl Backend) -> Result<Value> {
    let calendar = r.text("calendar").unwrap_or("primary");
    if r.command == "calendars" {
        let value = api.call("calendar", "calendars.list", r.list_params(json!({})))?;
        return view("calendar", "calendars.list", value);
    }
    if r.command == "show" {
        let value = api.call(
            "calendar",
            "events.get",
            json!({"calendarId":calendar,"eventId":r.target()}),
        )?;
        return view("calendar", "events.get", value);
    }
    if r.command == "cancel" {
        return Ok(changed(api.call("calendar", "events.delete", json!({"calendarId":calendar,"eventId":r.target(),"sendUpdates":if r.flag("notify") { "all" } else { "none" }}))?, "Event cancelled."));
    }
    let zone = calendar_zone(r, api)?;
    let now = Timestamp::now();
    let today = now.to_zoned_datetime(zone.clone()).date();
    let zone_name = zone
        .iana_name()
        .context("calendar timezone has no IANA name")?;
    match r.command.as_str() {
        "today" | "week" | "find" => {
            let mut params = json!({"calendarId":calendar,"singleEvents":true,"orderBy":"startTime","timeZone":zone_name});
            if r.command != "find" {
                let start = if r.command == "week" {
                    today.checked_sub(i64::from(today.weekday().to_monday_zero_offset()).days())?
                } else {
                    today
                };
                let end = start.checked_add(if r.command == "week" {
                    7.days()
                } else {
                    1.day()
                })?;
                params["timeMin"] = json!(values::midnight(start, &zone)?.to_string());
                params["timeMax"] = json!(values::midnight(end, &zone)?.to_string());
            } else {
                params["q"] = json!(r.text("query").unwrap());
                if let Some(from) = r.text("from") {
                    params["timeMin"] =
                        json!(values::midnight(values::date(from, today)?, &zone)?.to_string());
                }
                if let Some(until) = r.text("until") {
                    params["timeMax"] = json!(
                        values::midnight(values::date(until, today)?.tomorrow()?, &zone)?
                            .to_string()
                    );
                }
                if let (Some(from), Some(until)) =
                    (params["timeMin"].as_str(), params["timeMax"].as_str())
                {
                    ensure!(
                        parse_instant(from)? < parse_instant(until)?,
                        "until: must be on or after from: in the calendar timezone"
                    );
                }
                params = r.list_params(params);
            }
            view(
                "calendar",
                "events.list",
                api.call("calendar", "events.list", params)?,
            )
        }
        "add" => {
            let mut body = json!({"summary":r.text("title").unwrap()});
            if let Some(on) = r.text("on") {
                let day = values::date(on, today)?;
                body["start"] = json!({"date":day.to_string()});
                body["end"] = json!({"date":day.tomorrow()?.to_string()});
            } else {
                let start = values::timestamp(r.text("at").unwrap(), &zone, now)?;
                let end = start
                    .checked_add(values::duration(
                        r.text("for").expect("timed event duration default"),
                    )?)
                    .context("event end is outside its allowed range")?;
                body["start"] = json!({"dateTime":start.to_string(),"timeZone":zone_name});
                body["end"] = json!({"dateTime":end.to_string(),"timeZone":zone_name});
            }
            if let Some(where_) = r.text("where") {
                body["location"] = json!(where_);
            }
            let attendees = r.strings("with");
            if !attendees.is_empty() {
                body["attendees"] = json!(
                    attendees
                        .iter()
                        .map(|email| json!({"email":email}))
                        .collect::<Vec<_>>()
                );
            }
            Ok(changed(
                api.call(
                    "calendar",
                    "events.insert",
                    json!({"calendarId":calendar,"sendUpdates":"all","body":body}),
                )?,
                "Event created.",
            ))
        }
        "move" => {
            let original = api.call(
                "calendar",
                "events.get",
                json!({"calendarId":calendar,"eventId":r.target()}),
            )?;
            ensure!(
                original.pointer("/start/date").is_none(),
                "use an all-day event's date fields with joog to reschedule it as all-day; at: moves timed events"
            );
            let duration = if let Some(duration) = r.text("for") {
                values::duration(duration)?
            } else {
                parse_instant(
                    original
                        .pointer("/end/dateTime")
                        .and_then(Value::as_str)
                        .context("event has no end time")?,
                )? - parse_instant(
                    original
                        .pointer("/start/dateTime")
                        .and_then(Value::as_str)
                        .context("event has no start time")?,
                )?
            };
            let start = values::timestamp(r.text("at").unwrap(), &zone, now)?;
            let end = start
                .checked_add(duration)
                .context("event end is outside its allowed range")?;
            Ok(changed(api.call("calendar", "events.patch", json!({"calendarId":calendar,"eventId":r.target(),"sendUpdates":"all","body":{"start":{"dateTime":start.to_string(),"timeZone":zone_name},"end":{"dateTime":end.to_string(),"timeZone":zone_name}}}))?, "Event rescheduled."))
        }
        "free" => {
            let day = values::date(r.text("on").unwrap(), today)?;
            let start = zone
                .to_ambiguous_zoned_datetime(day.at(9, 0, 0, 0))
                .unambiguous()
                .context("working hours are ambiguous in this timezone")?
                .timestamp();
            let end = zone
                .to_ambiguous_zoned_datetime(day.at(18, 0, 0, 0))
                .unambiguous()
                .context("working hours are ambiguous in this timezone")?
                .timestamp();
            let duration = values::duration(r.text("for").unwrap())?;
            let mut calendars = r.strings("with");
            calendars.push(calendar.into());
            calendars.sort();
            calendars.dedup();
            let result = api.call("calendar", "freebusy.query", json!({"body":{"timeMin":start.to_string(),"timeMax":end.to_string(),"timeZone":zone_name,"items":calendars.iter().map(|id| json!({"id":id})).collect::<Vec<_>>()}}))?;
            let mut busy = Vec::new();
            for calendar in &calendars {
                let item = result["calendars"]
                    .get(calendar)
                    .context("Google omitted availability for a requested calendar")?;
                ensure!(
                    rows(item, "errors").is_empty(),
                    "availability is unavailable for {calendar}; check calendar sharing"
                );
                for slot in rows(item, "busy") {
                    busy.push((
                        parse_instant(id(slot, "start")?)?,
                        parse_instant(id(slot, "end")?)?,
                    ));
                }
            }
            let mut slots = Vec::new();
            let mut cursor = start;
            while let Ok(finish) = cursor.checked_add(duration) {
                if finish > end {
                    break;
                }
                if busy
                    .iter()
                    .all(|(start, end)| finish <= *start || cursor >= *end)
                {
                    slots.push(json!({"start":values::rfc3339(cursor, &zone),"end":values::rfc3339(finish, &zone)}));
                }
                cursor = cursor.checked_add(SignedDuration::from_secs(900))?;
            }
            Ok(json!(slots))
        }
        _ => bail!("unknown Calendar command"),
    }
}

fn range(r: &Request, api: &mut impl Backend, selected: &str) -> Result<String> {
    if selected.contains('!') {
        ensure!(
            r.text("tab").is_none(),
            "select a tab with tab: or inside the A1 range, choosing one"
        );
        return Ok(selected.into());
    }
    let tab = if let Some(tab) = r.text("tab") {
        tab.to_owned()
    } else {
        let spreadsheet = api.call(
            "sheets",
            "get",
            json!({"spreadsheetId":r.target(),"fields":"sheets.properties"}),
        )?;
        spreadsheet
            .pointer("/sheets/0/properties/title")
            .and_then(Value::as_str)
            .context("spreadsheet has no tabs")?
            .into()
    };
    let tab = format!("'{}'", tab.replace('\'', "''"));
    Ok(if selected.is_empty() {
        tab
    } else {
        format!("{tab}!{selected}")
    })
}

fn sheets(r: &Request, api: &mut impl Backend) -> Result<Value> {
    if r.command == "ls" {
        return list_files(r, api, Some(file_type("spreadsheet")));
    }
    if r.command == "new" {
        let mut params = json!({"title":r.text("title").unwrap()});
        if let Some(tab) = r.text("tab") {
            params["sheets"] = json!([{"properties":{"title":tab}}]);
        }
        return Ok(changed(
            api.call("sheets", "create", params)?,
            "Spreadsheet created.",
        ));
    }
    if r.command == "export" && r.text("format") != Some("csv") {
        return export(r, api);
    }
    let selected = r
        .text("range")
        .or_else(|| r.text("cell"))
        .or_else(|| r.text("at"))
        .unwrap_or(if matches!(r.command.as_str(), "export" | "append") {
            ""
        } else {
            "A1:Z20"
        });
    let selected = range(r, api, selected)?;
    let base = json!({"spreadsheetId":r.target(),"range":selected});
    match r.command.as_str() {
        "read" => view(
            "sheets",
            "values.get",
            api.call("sheets", "values.get", base)?,
        ),
        "export" => {
            let value = api.call("sheets", "values.get", base)?;
            let rows = rows(&value, "values")
                .iter()
                .map(|row| {
                    row.as_array()
                        .into_iter()
                        .flatten()
                        .map(|cell| {
                            cell.as_str()
                                .map(str::to_owned)
                                .unwrap_or_else(|| cell.to_string())
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            values::save(&values::csv_bytes(&rows)?, r.text("out").unwrap())
        }
        "clear" => Ok(changed(
            api.call("sheets", "values.clear", base)?,
            "Cells cleared.",
        )),
        "set" | "append" | "load" => {
            let mut params = base;
            let data = if r.command == "set" {
                vec![vec![r.text("value").unwrap().to_owned()]]
            } else {
                values::csv_rows(&r.params)?
            };
            params["values"] = json!(data);
            params["valueInputOption"] = json!("RAW");
            if r.command == "append" {
                params["insertDataOption"] = json!("INSERT_ROWS");
            }
            let method = if r.command == "append" {
                "values.append"
            } else {
                "values.update"
            };
            Ok(changed(
                api.call("sheets", method, params)?,
                "Cells updated.",
            ))
        }
        _ => bail!("unknown Sheets command"),
    }
}

fn text_content(value: &Value, result: &mut String) {
    if let Some(content) = value.pointer("/textRun/content").and_then(Value::as_str) {
        result.push_str(content);
    }
    match value {
        Value::Object(object) => {
            for (name, child) in object {
                if name != "textRun" {
                    text_content(child, result);
                }
            }
        }
        Value::Array(children) => {
            for child in children {
                text_content(child, result);
            }
        }
        _ => {}
    }
}

fn document_tabs<'a>(tabs: &'a [Value], all: &mut Vec<&'a Value>) {
    for tab in tabs {
        all.push(tab);
        document_tabs(rows(tab, "childTabs"), all);
    }
}

fn docs(r: &Request, api: &mut impl Backend) -> Result<Value> {
    match r.command.as_str() {
        "ls" => list_files(r, api, Some(file_type("document"))),
        "export" => export(r, api), "share" => Ok(changed(share(r, api)?, "Access granted.")),
        "new" => {
            let document = api.call("docs", "create", json!({"title":r.text("title").unwrap()}))?;
            let target = id(&document, "documentId")?;
            if let Some(text) = values::text(&r.params, "text")? { api.call("docs", "insert-text", json!({"documentId":target,"text":text,"endOfSegmentLocation":{}})).with_context(|| format!("document {target} was created, but adding its text failed"))?; }
            if let Some(folder) = r.text("folder") { place(api, target, folder, None).with_context(|| format!("document {target} was created, but moving it failed"))?; }
            Ok(changed(document, "Document created."))
        }
        "read" => {
            let value = api.call("docs", "get", json!({"documentId":r.target(),"includeTabsContent":true}))?;
            let mut tabs = Vec::new(); document_tabs(rows(&value, "tabs"), &mut tabs);
            if let Some(tab) = r.text("tab") {
                let selected = tabs.iter().find(|item| item.pointer("/tabProperties/tabId").and_then(Value::as_str) == Some(tab)).context("document tab was not found")?;
                let mut text = String::new(); text_content(&selected["documentTab"], &mut text);
                Ok(json!({"id":r.target(),"title":value["title"],"tab":tab,"text":text}))
            } else {
                let mut text = String::new();
                if tabs.is_empty() { text_content(&value["body"], &mut text); } else { for tab in tabs { text_content(&tab["documentTab"], &mut text); } }
                Ok(json!({"id":r.target(),"title":value["title"],"text":text}))
            }
        }
        "append" => {
            let mut location = json!({}); if let Some(tab) = r.text("tab") { location["tabId"] = json!(tab); }
            Ok(changed(api.call("docs", "insert-text", json!({"documentId":r.target(),"text":values::text(&r.params,"text")?.unwrap(),"endOfSegmentLocation":location}))?, "Text appended."))
        }
        "replace" => Ok(changed(api.call("docs", "replace-all-text", json!({"documentId":r.target(),"containsText":{"text":r.text("find").unwrap(),"matchCase":r.flag("matchCase")},"replaceText":r.text("with").unwrap()}))?, "Text replaced.")),
        _ => bail!("unknown Docs command"),
    }
}

fn placeholder<'a>(slide: &'a Value, kind: &str) -> Option<&'a str> {
    rows(slide, "pageElements")
        .iter()
        .find(|element| {
            let kind_ = element
                .pointer("/shape/placeholder/type")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if kind == "title" {
                matches!(kind_, "TITLE" | "CENTERED_TITLE")
            } else {
                matches!(kind_, "BODY" | "SUBTITLE")
            }
        })
        .and_then(|element| element["objectId"].as_str())
}

fn slides(r: &Request, api: &mut impl Backend) -> Result<Value> {
    if r.command == "ls" {
        return list_files(r, api, Some(file_type("presentation")));
    }
    if r.command == "export" {
        return export(r, api);
    }
    if r.command == "new" {
        let value = if let Some(template) = r.text("template") {
            api.call("slides", "get", json!({"presentationId":template}))?;
            let value = api.call(
                "drive",
                "files.copy",
                json!({"fileId":template,"name":r.text("title").unwrap()}),
            )?;
            json!({"presentationId":id(&value,"id")?,"title":r.text("title").unwrap()})
        } else {
            api.call(
                "slides",
                "create",
                json!({"title":r.text("title").unwrap()}),
            )?
        };
        if let Some(folder) = r.text("folder") {
            let target = id(&value, "presentationId")?;
            place(api, target, folder, None).with_context(|| {
                format!("presentation {target} was created, but moving it failed")
            })?;
        }
        return Ok(changed(value, "Presentation created."));
    }
    if r.command == "replace" && r.params.get("slide").is_none() {
        return Ok(changed(api.call("slides", "replace-all-text", json!({"presentationId":r.target(),"containsText":{"text":r.text("find").unwrap(),"matchCase":false},"replaceText":r.text("with").unwrap()}))?, "Text replaced."));
    }
    let presentation = api.call("slides", "get", json!({"presentationId":r.target()}))?;
    let slides = rows(&presentation, "slides");
    if r.command == "slides" {
        return view("slides", "get", presentation);
    }
    let selected = if let Some(index) = r.params.get("slide").and_then(Value::as_u64) {
        Some(
            slides
                .get((index - 1) as usize)
                .context("slide number is outside this presentation")?,
        )
    } else {
        None
    };
    let mut requests = Vec::new();
    if r.command == "add" {
        let after = r
            .params
            .get("after")
            .and_then(Value::as_u64)
            .unwrap_or(slides.len() as u64);
        ensure!(
            after <= slides.len() as u64,
            "after: is outside this presentation"
        );
        let mut random = [0u8; 12];
        getrandom::fill(&mut random).context("cannot generate slide IDs")?;
        let slide_id = format!(
            "joog_{:024x}",
            u128::from_le_bytes({
                let mut bytes = [0u8; 16];
                bytes[..12].copy_from_slice(&random);
                bytes
            })
        );
        let title_id = format!("{slide_id}_title");
        let body_id = format!("{slide_id}_body");
        let layout = match r.text("layout").unwrap() {
            "title-body" => "TITLE_AND_BODY",
            "title" => "TITLE",
            "section" => "SECTION_HEADER",
            "blank" => "BLANK",
            _ => unreachable!(),
        };
        let mut mappings = Vec::new();
        if r.text("title").is_some() {
            mappings.push(json!({"layoutPlaceholder":{"type":if layout == "TITLE" {"CENTERED_TITLE"} else {"TITLE"},"index":0},"objectId":title_id}));
        }
        if r.text("body").is_some() {
            mappings.push(json!({"layoutPlaceholder":{"type":if layout == "TITLE_AND_BODY" {"BODY"} else {"SUBTITLE"},"index":0},"objectId":body_id}));
        }
        requests.push(json!({"createSlide":{"objectId":slide_id,"insertionIndex":after,"slideLayoutReference":{"predefinedLayout":layout},"placeholderIdMappings":mappings}}));
        for (field, target) in [("title", &title_id), ("body", &body_id)] {
            if let Some(text) = r.text(field)
                && !text.is_empty()
            {
                requests
                    .push(json!({"insertText":{"objectId":target,"text":text,"insertionIndex":0}}));
            }
        }
    } else if r.command == "set" {
        let selected = selected.context("provide slide:")?;
        for field in ["title", "body"] {
            if let Some(text) = r.text(field) {
                let target = placeholder(selected, field)
                    .with_context(|| format!("this slide has no {field} placeholder"))?;
                let existing = rows(selected, "pageElements")
                    .iter()
                    .find(|element| element["objectId"] == target)
                    .and_then(|element| element.pointer("/shape/text/textElements"))
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(|element| {
                        element.pointer("/textRun/content").and_then(Value::as_str)
                    })
                    .collect::<String>();
                if !existing.is_empty() && existing != "\n" {
                    requests
                        .push(json!({"deleteText":{"objectId":target,"textRange":{"type":"ALL"}}}));
                }
                if !text.is_empty() {
                    requests.push(
                        json!({"insertText":{"objectId":target,"text":text,"insertionIndex":0}}),
                    );
                }
            }
        }
    } else if r.command == "replace" {
        requests.push(json!({"replaceAllText":{"containsText":{"text":r.text("find").unwrap(),"matchCase":false},"replaceText":r.text("with").unwrap(),"pageObjectIds":[id(selected.unwrap(),"objectId")?]}}));
    } else {
        bail!("unknown Slides command");
    }
    let mut params = json!({"presentationId":r.target(),"requests":requests});
    if let Some(revision) = presentation["revisionId"].as_str() {
        params["writeControl"] = json!({"requiredRevisionId":revision});
    }
    Ok(changed(
        api.call("slides", "batch-update", params)?,
        "Presentation updated.",
    ))
}

fn forms(r: &Request, api: &mut impl Backend) -> Result<Value> {
    if r.command == "ls" {
        return list_files(r, api, Some(file_type("form")));
    }
    if r.command == "new" {
        let form = api.call("forms", "create", json!({"title":r.text("title").unwrap()}))?;
        if let Some(description) = r.text("description") {
            let target = id(&form, "formId")?;
            api.call("forms", "batch-update", json!({"formId":target,"requests":[{"updateFormInfo":{"info":{"description":description},"updateMask":"description"}}]})).with_context(|| format!("form {target} was created, but setting its description failed"))?;
        }
        return Ok(changed(form, "Form created."));
    }
    if r.command == "show" {
        return api.call("forms", "get", json!({"formId":r.target()}));
    }
    if matches!(r.command.as_str(), "publish" | "open" | "close") {
        let form = api.call("forms", "get", json!({"formId":r.target()}))?;
        let published = form
            .pointer("/publishSettings/publishState/isPublished")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        ensure!(
            r.command != "open" || published,
            "publish this form before opening it for responses"
        );
        let state = json!({"isPublished":r.command == "publish" || published,"isAcceptingResponses":r.command != "close"});
        api.call("forms", "set-publish-settings", json!({"formId":r.target(),"publishSettings":{"publishState":state},"updateMask":"publishState"}))?;
        return Ok(
            json!({"formId":r.target(),"responderUri":form["responderUri"],"publishState":state}),
        );
    }
    if r.command == "question" {
        let form = api.call("forms", "get", json!({"formId":r.target()}))?;
        let mut question = json!({"required":r.flag("required")});
        match r.text("type").unwrap() {
            "text" | "paragraph" => {
                question["textQuestion"] = json!({"paragraph":r.text("type") == Some("paragraph")})
            }
            "date" => question["dateQuestion"] = json!({"includeYear":true,"includeTime":false}),
            "choice" | "checkboxes" => {
                question["choiceQuestion"] = json!({"type":if r.text("type") == Some("choice") { "RADIO" } else { "CHECKBOX" },"options":r.strings("options").iter().map(|value| json!({"value":value})).collect::<Vec<_>>()})
            }
            _ => unreachable!(),
        }
        let mut params = json!({"formId":r.target(),"requests":[{"createItem":{"item":{"title":r.text("title").unwrap(),"questionItem":{"question":question}},"location":{"index":rows(&form,"items").len()}}}]});
        if let Some(revision) = form["revisionId"].as_str() {
            params["writeControl"] = json!({"requiredRevisionId":revision});
        }
        return Ok(changed(
            api.call("forms", "batch-update", params)?,
            "Question added.",
        ));
    }
    if r.command == "answers" {
        let mut params = json!({"formId":r.target()});
        if let Some(since) = r.text("since") {
            params["filter"] = json!(format!("timestamp >= {since}"));
        }
        let value = api.call("forms", "responses.list", r.list_params(params))?;
        if let Some(path) = r.text("out") {
            let form = api.call("forms", "get", json!({"formId":r.target()}))?;
            let mut questions = Vec::<(String, String)>::new();
            for item in rows(&form, "items") {
                if let Some(question) = item
                    .pointer("/questionItem/question/questionId")
                    .and_then(Value::as_str)
                {
                    questions.push((
                        question.into(),
                        item["title"].as_str().unwrap_or(question).into(),
                    ));
                }
                for question in item
                    .pointer("/questionGroupItem/questions")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                {
                    if let Some(question_id) = question["questionId"].as_str() {
                        questions.push((
                            question_id.into(),
                            format!(
                                "{} {}",
                                item["title"].as_str().unwrap_or_default(),
                                question
                                    .pointer("/rowQuestion/title")
                                    .and_then(Value::as_str)
                                    .unwrap_or(question_id)
                            ),
                        ));
                    }
                }
            }
            for response in rows(&value, "responses") {
                for key in response["answers"]
                    .as_object()
                    .into_iter()
                    .flatten()
                    .map(|(key, _)| key)
                {
                    if !questions.iter().any(|(id, _)| id == key) {
                        questions.push((key.clone(), key.clone()));
                    }
                }
            }
            let mut headers = vec!["responseId".into(), "submitted".into(), "email".into()];
            headers.extend(questions.iter().map(|(_, title)| title.clone()));
            let mut data = vec![headers];
            for response in rows(&value, "responses") {
                let mut row = vec![
                    response["responseId"].as_str().unwrap_or_default().into(),
                    response["lastSubmittedTime"]
                        .as_str()
                        .unwrap_or_default()
                        .into(),
                    response["respondentEmail"]
                        .as_str()
                        .unwrap_or_default()
                        .into(),
                ];
                for (question, _) in &questions {
                    let answer = &response["answers"][question];
                    let values = answer
                        .pointer("/textAnswers/answers")
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                        .filter_map(|value| value["value"].as_str())
                        .map(str::to_owned)
                        .chain(
                            answer
                                .pointer("/fileUploadAnswers/answers")
                                .and_then(Value::as_array)
                                .into_iter()
                                .flatten()
                                .filter_map(|value| value["fileId"].as_str())
                                .map(str::to_owned),
                        )
                        .collect::<Vec<_>>();
                    row.push(values.join("\n"));
                }
                data.push(row);
            }
            return values::save(&values::csv_bytes(&data)?, path);
        }
        return Ok(value);
    }
    bail!("unknown Forms command")
}

fn tasks(r: &Request, api: &mut impl Backend) -> Result<Value> {
    let list = r.text("list").unwrap_or("@default");
    let command = match r.command.as_str() {
        "ls" => "tasks.list",
        "add" => "tasks.insert",
        "show" => "tasks.get",
        "done" | "undo" | "set" => "tasks.patch",
        "delete" => "tasks.delete",
        "lists" => "tasklists.list",
        "new-list" => "tasklists.insert",
        _ => bail!("unknown Tasks command"),
    };
    let mut params = if matches!(r.command.as_str(), "lists" | "new-list") {
        json!({})
    } else {
        json!({"tasklist":list})
    };
    if let Some(target) = r.text("target") {
        params["task"] = json!(target);
    }
    if let Some(title) = r.text("title") {
        params["title"] = json!(title);
    }
    if let Some(notes) = r.text("notes") {
        params["notes"] = json!(notes);
    }
    if let Some(due) = r.text("due") {
        let due = values::date(due, ZonedDateTime::now().date())?;
        if r.command == "ls" {
            params["dueMin"] = json!(format!("{due}T00:00:00Z"));
            params["dueMax"] = json!(format!("{}T00:00:00Z", due.tomorrow()?));
        } else {
            params["due"] = json!(format!("{due}T00:00:00Z"));
        }
    }
    if r.command == "ls" {
        params["showCompleted"] = json!(r.flag("completed"));
        params["showHidden"] = json!(r.flag("completed"));
    }
    if r.command == "done" {
        params["status"] = json!("completed");
    }
    if r.command == "undo" {
        params["status"] = json!("needsAction");
        params["completed"] = Value::Null;
    }
    if matches!(r.command.as_str(), "ls" | "lists") {
        params = r.list_params(params);
    }
    let value = api.call("tasks", command, params)?;
    if matches!(r.command.as_str(), "ls" | "lists" | "show") {
        view("tasks", command, value)
    } else {
        Ok(changed(value, "Task updated."))
    }
}

fn contacts(r: &Request, api: &mut impl Backend) -> Result<Value> {
    const FIELDS: &str = "names,emailAddresses,phoneNumbers,organizations,metadata";
    if r.command == "ls" {
        let value = api.call(
            "contacts",
            "people.connections.list",
            r.list_params(json!({"resourceName":"people/me","personFields":FIELDS})),
        )?;
        return view("contacts", "people.connections.list", value);
    }
    if r.command == "find" {
        let value = api.call(
            "contacts",
            "people.search-contacts",
            r.list_params(json!({"query":r.text("query").unwrap(),"readMask":FIELDS})),
        )?;
        return view("contacts", "people.search-contacts", value);
    }
    if r.command == "show" {
        let value = api.call(
            "contacts",
            "people.get",
            json!({"resourceName":r.target(),"personFields":FIELDS}),
        )?;
        return view("contacts", "people.get", value);
    }
    if r.command == "delete" {
        return Ok(changed(
            api.call(
                "contacts",
                "people.delete-contact",
                json!({"resourceName":r.target()}),
            )?,
            "Contact deleted.",
        ));
    }
    let mut body = Map::new();
    let mut mask = Vec::new();
    if let Some(name) = r.text("name") {
        body.insert("names".into(), json!([{"unstructuredName":name}]));
        mask.push("names");
    }
    for (field, wire) in [("email", "emailAddresses"), ("phone", "phoneNumbers")] {
        if r.params.contains_key(field) {
            body.insert(
                wire.into(),
                json!(
                    r.strings(field)
                        .iter()
                        .map(|value| json!({"value":value}))
                        .collect::<Vec<_>>()
                ),
            );
            mask.push(wire);
        }
    }
    if let Some(company) = r.text("company") {
        body.insert("organizations".into(), json!([{"name":company}]));
        mask.push("organizations");
    }
    let value = if r.command == "new" {
        api.call(
            "contacts",
            "people.create-contact",
            json!({"body":body,"personFields":FIELDS}),
        )?
    } else {
        let current = api.call(
            "contacts",
            "people.get",
            json!({"resourceName":r.target(),"personFields":FIELDS}),
        )?;
        let sources = rows(&current["metadata"], "sources")
            .iter()
            .filter(|source| source["type"] == "CONTACT")
            .cloned()
            .collect::<Vec<_>>();
        ensure!(
            !sources.is_empty(),
            "this person has no editable contact source"
        );
        body.insert("metadata".into(), json!({"sources":sources}));
        if let Some(etag) = current["etag"].as_str() {
            body.insert("etag".into(), json!(etag));
        }
        api.call("contacts","people.update-contact",json!({"resourceName":r.target(),"updatePersonFields":mask.join(","),"personFields":FIELDS,"body":body}))?
    };
    view("contacts", "people.get", changed(value, "Contact saved."))
}

fn youtube(r: &Request, api: &mut impl Backend) -> Result<Value> {
    let (method, params) = match r.command.as_str() {
        "find" => { let mut p=json!({"q":r.text("query").unwrap(),"part":"snippet","type":r.text("type").unwrap()}); if let Some(channel)=r.text("channel") { p["channelId"]=json!(channel); } ("search.list",r.list_params(p)) }
        "info" => if let Some(target)=r.text("target") { ("videos.list",json!({"id":target,"part":"snippet,contentDetails,statistics"})) } else { ("channels.list",json!({"mine":true,"part":"snippet,contentDetails,statistics"})) },
        "comments" => ("comment-threads.list",r.list_params(json!({"videoId":r.target(),"part":"snippet","order":if r.text("order")==Some("newest") {"time"} else {"relevance"}}))),
        "like" | "unlike" => ("videos.rate",json!({"id":r.target(),"rating":if r.command=="like" {"like"} else {"none"}})),
        "playlists" => { let mut p=json!({"part":"snippet,contentDetails"}); if let Some(channel)=r.text("channel") {p["channelId"]=json!(channel);} else {p["mine"]=json!(true);} ("playlists.list",r.list_params(p)) }
        "playlist" => ("playlist-items.list",r.list_params(json!({"playlistId":r.target(),"part":"snippet,contentDetails"}))),
        "new-playlist" => { let mut body=json!({"snippet":{"title":r.text("title").unwrap()},"status":{"privacyStatus":r.text("privacy").unwrap()}}); if let Some(description)=r.text("description") {body["snippet"]["description"]=json!(description);} ("playlists.insert",json!({"part":"snippet,status","body":body})) }
        "add" => {
            let mut added=Vec::new();
            for video in r.strings("videos") { added.push(api.call("youtube","playlist-items.insert",json!({"part":"snippet","body":{"snippet":{"playlistId":r.target(),"resourceId":{"kind":"youtube#video","videoId":video}}}})).with_context(||format!("{} videos were added before this failure; do not retry the complete list without checking it",added.len()))?); }
            return Ok(json!({"added":added}));
        }
        "upload" => {
            let category=r.text("category").unwrap();
            let category=if category.chars().all(|ch|ch.is_ascii_digit()) {category.into()} else {
                let available=api.call("youtube","video-categories.list",json!({"part":"snippet","regionCode":"US","hl":"en"}))?;
                let normal=|text:&str|text.to_lowercase().chars().filter(|ch|ch.is_alphanumeric()).collect::<String>();
                let selected=rows(&available,"items").iter().find(|item| {
                    let title=item.pointer("/snippet/title").and_then(Value::as_str).unwrap_or_default();
                    normal(title)==normal(category) || (category.eq_ignore_ascii_case("education") && item["id"]=="27")
                }).context("category was not found; supply an available category name or numeric ID")?;
                id(selected,"id")?.to_owned()
            };
            let mut snippet=json!({"title":r.text("title").unwrap(),"categoryId":category});
            if let Some(description)=r.text("description") {snippet["description"]=json!(description);}
            ("videos.insert",json!({"part":"snippet,status","body":{"snippet":snippet,"status":{"privacyStatus":r.text("privacy").unwrap()}},"upload":r.text("path").unwrap(),"contentType":values::upload_mime(r.text("path").unwrap())}))
        }
        _ => bail!("unknown YouTube command"),
    };
    let value = api.call("youtube", method, params)?;
    if matches!(
        r.command.as_str(),
        "find" | "info" | "comments" | "playlists" | "playlist"
    ) {
        view("youtube", method, value)
    } else {
        Ok(changed(value, "YouTube updated."))
    }
}

fn photos(r: &Request, api: &mut impl Backend) -> Result<Value> {
    let (method, params) = match r.command.as_str() {
        "ls" => {
            if let Some(album) = r.text("album") {
                (
                    "media-items.search",
                    r.list_params(json!({"albumId":album})),
                )
            } else {
                ("media-items.list", r.list_params(json!({})))
            }
        }
        "find" => {
            let mut filters = json!({});
            if let Some(kind) = r.text("type") {
                filters["mediaTypeFilter"] =
                    json!({"mediaTypes":[if kind=="photo" {"PHOTO"} else {"VIDEO"}]});
            }
            if r.text("from").is_some() || r.text("until").is_some() {
                let today = ZonedDateTime::now().date();
                let from = values::date(r.text("from").unwrap_or("0001-01-01"), today)?;
                let until = values::date(r.text("until").unwrap_or("9999-12-31"), today)?;
                let date = |day: jiff::civil::Date| json!({"year":day.year(),"month":day.month(),"day":day.day()});
                filters["dateFilter"] =
                    json!({"ranges":[{"startDate":date(from),"endDate":date(until)}]});
            }
            (
                "media-items.search",
                r.list_params(json!({"filters":filters})),
            )
        }
        "show" => ("media-items.get", json!({"mediaItemId":r.target()})),
        "get" => {
            let value = api.call(
                "photos",
                "media-items.get",
                json!({"mediaItemId":r.target()}),
            )?;
            let address = format!(
                "{}={}",
                id(&value, "baseUrl")?,
                if value["mimeType"]
                    .as_str()
                    .is_some_and(|mime| mime.starts_with("video/"))
                {
                    "dv"
                } else {
                    "d"
                }
            );
            return api.download_photo(&address, r.text("out").unwrap());
        }
        "put" => {
            let path = r.text("path").unwrap();
            let token = api.upload_photo(path)?;
            let mut params = json!({"newMediaItems":[{"description":r.text("caption").unwrap_or_default(),"simpleMediaItem":{"uploadToken":token,"fileName":std::path::Path::new(path).file_name().unwrap().to_string_lossy()}}]});
            if let Some(album) = r.text("album") {
                params["albumId"] = json!(album);
            }
            let value = api.call("photos", "media-items.batch-create", params)?;
            let result = value
                .pointer("/newMediaItemResults/0")
                .context("Photos did not return a media creation result")?;
            ensure!(
                result
                    .pointer("/status/code")
                    .and_then(Value::as_i64)
                    .unwrap_or(0)
                    == 0,
                "Photos rejected the uploaded media item; check its file type, size, and album access"
            );
            return Ok(changed(result["mediaItem"].clone(), "Media uploaded."));
        }
        "albums" => ("albums.list", r.list_params(json!({}))),
        "new-album" => ("albums.create", json!({"title":r.text("title").unwrap()})),
        "add" | "remove" => (
            if r.command == "add" {
                "albums.batch-add-media-items"
            } else {
                "albums.batch-remove-media-items"
            },
            json!({"albumId":r.target(),"mediaItemIds":r.strings("photos")}),
        ),
        _ => bail!("unknown Photos command"),
    };
    let value = api.call("photos", method, params)?;
    if matches!(r.command.as_str(), "ls" | "find" | "show" | "albums") {
        view("photos", method, value)
    } else {
        Ok(changed(value, "Album updated."))
    }
}
