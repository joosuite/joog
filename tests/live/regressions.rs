use super::*;

pub(super) fn calendar_subscription(run: &mut Live) -> Result<()> {
    let public = "en.japanese#holiday@group.v.calendar.google.com";
    run.api("calendar", "calendars.get", json!({"calendarId":public}))?;
    if scoped_or_rejected(
        run,
        "calendar",
        "calendar-list.get",
        json!({"calendarId":public}),
        &[404],
    )?
    .is_some()
    {
        return Ok(());
    }
    let subscribed = run.api(
        "calendar",
        "calendar-list.insert",
        json!({"id":public,"hidden":true,"selected":false}),
    )?;
    let target = id(&subscribed, "id")?;
    run.register(
        "calendar",
        "calendar-list.delete",
        json!({"calendarId":target}),
        &target,
    )?;
    ensure!(target == public, "public calendar subscription ID changed");
    run.api(
        "calendar",
        "calendar-list.delete",
        json!({"calendarId":target}),
    )?;
    removed(run, &target)
}

fn callback(run: &mut Live) -> Result<String> {
    let marker = run.marker.clone();
    let created = run.json("jforms", &["new", &format!("{marker}-callback")])?;
    let form = id(&created, "formId")?;
    own_file(run, &form)?;
    run.json(
        "jforms",
        &["question", &form, "Fixture", "type:text", "required:true"],
    )?;
    run.json("jforms", &["publish", &form])?;
    let uri = created["responderUri"]
        .as_str()
        .context("form responder URI missing")?;
    let uri = url::Url::parse(uri)?;
    ensure!(
        uri.scheme() == "https"
            && uri.host_str() == Some("docs.google.com")
            && uri.path().starts_with("/forms/"),
        "unexpected callback URL"
    );
    Ok(uri
        .as_str()
        .split('?')
        .next()
        .unwrap()
        .trim_end_matches("viewform")
        .to_owned()
        + "formResponse")
}

pub(super) fn drive_watch(run: &mut Live) -> Result<()> {
    let address = callback(run)?;
    let token = run.api("drive", "changes.get-start-page-token", json!({}))?;
    let expiration =
        u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())? + 10_000;
    let channel=run.api("drive","changes.watch",json!({"pageToken":id(&token,"startPageToken")?,"id":format!("{}-drive-watch",run.marker),"type":"web_hook","address":address,"expiration":expiration.to_string()}))?;
    let channel_id = id(&channel, "id")?;
    let resource_id = id(&channel, "resourceId")?;
    run.register(
        "drive",
        "channels.stop",
        json!({"id":channel_id,"resourceId":resource_id}),
        &channel_id,
    )?;
    run.api(
        "drive",
        "channels.stop",
        json!({"id":channel_id,"resourceId":resource_id}),
    )?;
    removed(run, &channel_id)
}

pub(super) fn calendar_watch(run: &mut Live) -> Result<()> {
    let address = callback(run)?;
    let calendar = run.api(
        "calendar",
        "calendars.insert",
        json!({"summary":run.marker,"timeZone":"Etc/UTC"}),
    )?;
    let target = id(&calendar, "id")?;
    run.register(
        "calendar",
        "calendars.delete",
        json!({"calendarId":target}),
        &target,
    )?;
    let expiration =
        u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())? + 10_000;
    let channel=run.api("calendar","events.watch",json!({"calendarId":target,"id":format!("{}-calendar-watch",run.marker),"type":"web_hook","address":address,"expiration":expiration.to_string()}))?;
    let channel_id = id(&channel, "id")?;
    let actual = channel["expiration"]
        .as_str()
        .context("watch expiration missing")?
        .parse::<u64>()?;
    run.register(
        "calendar",
        "channel-expiration",
        json!({"id":channel_id,"expiration":actual,"resourceId":channel["resourceId"]}),
        &channel_id,
    )?;
    ensure!(
        actual <= expiration,
        "server extended the requested watch lifetime"
    );
    Ok(())
}

pub(super) fn mail_draft_delete(run: &mut Live) -> Result<()> {
    let draft = run.api(
        "mail",
        "drafts.create",
        json!({"to":[run.account],"subject":run.marker,"text":"Unsent draft deletion fixture"}),
    )?;
    let draft_id = id(&draft, "id")?;
    run.register("mail", "drafts.delete", json!({"id":draft_id}), &draft_id)?;
    run.api("mail", "drafts.delete", json!({"id":draft_id}))?;
    removed(run, &draft_id)
}

pub(super) fn drive_shared(run: &mut Live) -> Result<()> {
    run.api("drive", "drives.list", json!({"max":1}))?;
    let mut random = [0u8; 16];
    getrandom::fill(&mut random)?;
    random[6] = (random[6] & 0x0f) | 0x40;
    random[8] = (random[8] & 0x3f) | 0x80;
    let hex = random
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let request_id = format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    );
    if let Some(drive) = scoped_or_rejected(
        run,
        "drive",
        "drives.create",
        json!({"requestId":request_id,"name":run.marker}),
        &[403],
    )? {
        let shared = id(&drive, "id")?;
        run.register("drive", "drives.delete", json!({"driveId":shared}), &shared)?;
        run.api("drive", "drives.get", json!({"driveId":shared}))?;
        run.api(
            "drive",
            "drives.update",
            json!({"driveId":shared,"name":format!("{}-updated",run.marker)}),
        )?;
        run.api("drive", "drives.hide", json!({"driveId":shared}))?;
        run.api("drive", "drives.unhide", json!({"driveId":shared}))?;
        let trashed = run.api("drive","files.create",json!({"name":run.marker,"mimeType":"text/plain","parents":[shared],"trashed":true,"supportsAllDrives":true,"fields":"id,trashed,driveId"}))?;
        let file = id(&trashed, "id")?;
        own_file(run, &file)?;
        ensure!(
            trashed["trashed"] == true && trashed["driveId"] == shared,
            "trash fixture is outside its temporary shared drive"
        );
        run.api("drive", "files.empty-trash", json!({"driveId":shared}))?;
        removed(run, &file)?;
    } else {
        for command in [
            "drives.get",
            "drives.update",
            "drives.hide",
            "drives.unhide",
            "drives.delete",
            "files.empty-trash",
        ] {
            run.events.push(json!({"kind":"prerequisite","service":"drive","command":command,"status":"blocked","reason":"Google rejected the valid drives.create request with HTTP 403. No unrelated shared-drive IDs or account-wide Trash were used."}));
        }
        run.save()?;
    }
    Ok(())
}

pub(super) fn contacts_other(run: &mut Live) -> Result<()> {
    let (local, domain) = run.account.split_once('@').expect("verified account");
    let alias = format!("{local}+joog-test@{domain}");
    let found = run.api(
        "contacts",
        "other-contacts.search",
        json!({"query":alias,"readMask":"names,emailAddresses","max":1}),
    )?;
    let source = found["results"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|result| result.get("person"))
        .find(|person| {
            person["emailAddresses"].as_array().is_some_and(|emails| {
                emails.iter().any(|email| {
                    email["value"]
                        .as_str()
                        .is_some_and(|address| address.eq_ignore_ascii_case(&alias))
                })
            })
        });
    if let Some(source) = source {
        let source_id = id(source, "resourceName")?;
        let copied=run.api("contacts","other-contacts.copy-other-contact-to-my-contacts-group",json!({"resourceName":source_id,"copyMask":"names,emailAddresses","readMask":"names,emailAddresses"}))?;
        let copied_id = id(&copied, "resourceName")?;
        run.register(
            "contacts",
            "people.delete-contact",
            json!({"resourceName":copied_id}),
            &copied_id,
        )?;
        ensure!(
            copied_id != source_id,
            "copy returned the source contact's resource ID"
        );
        contains(
            &run.api(
                "contacts",
                "people.get",
                json!({"resourceName":copied_id,"personFields":"names,emailAddresses"}),
            )?,
            &alias,
        )?;
        run.api(
            "contacts",
            "people.delete-contact",
            json!({"resourceName":copied_id}),
        )?;
        removed(run, &copied_id)?;
    } else {
        run.events.push(json!({"kind":"prerequisite","service":"contacts","command":"other-contacts.copy-other-contact-to-my-contacts-group","status":"blocked","reason":"Google returned no Other contact for the self-addressed test alias. Unrelated saved correspondents were not copied or modified."}));
        run.save()?;
    }
    Ok(())
}

pub(super) fn calendar_owner(run: &mut Live) -> Result<()> {
    let calendar = run.api(
        "calendar",
        "calendars.insert",
        json!({"summary":run.marker,"timeZone":"Etc/UTC"}),
    )?;
    let target = id(&calendar, "id")?;
    run.register(
        "calendar",
        "calendars.delete",
        json!({"calendarId":target}),
        &target,
    )?;
    scoped_or_rejected(
        run,
        "calendar",
        "calendars.transfer-ownership",
        json!({"calendarId":target,"newDataOwner":run.account,"useAdminAccess":true}),
        &[400, 403],
    )?;
    let current = run.api(
        "calendar",
        "calendar-list.get",
        json!({"calendarId":target,"fields":"id,accessRole"}),
    )?;
    ensure!(
        current["accessRole"] == "owner",
        "self-transfer changed fixture ownership"
    );
    Ok(())
}

pub(super) fn sheets_csv(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let created = run.json("jsheets", &["new", &marker, "tab:Fixture"])?;
    let target = id(&created, "spreadsheetId")?;
    own_file(run, &target)?;
    run.api("sheets","values.update",json!({"spreadsheetId":target,"range":"'Fixture'!A1:D3","valueInputOption":"RAW","values":[["one","two","three","=1+1"],["Unicode café","quoted, cell"],["last"]]}))?;
    let out = run.directory.path().join("ragged.csv");
    run.json(
        "jsheets",
        &[
            "export",
            &target,
            "tab:Fixture",
            "format:csv",
            &format!("out:{}", out.display()),
        ],
    )?;
    let csv = fs::read_to_string(out)?;
    ensure!(
        csv == "one,two,three,=1+1\nUnicode café,\"quoted, cell\",,\nlast,,,\n",
        "CSV did not preserve ragged rows, literal formulas and quoted cells"
    );
    run.api(
        "sheets",
        "values.batch-clear",
        json!({"spreadsheetId":target,"ranges":["'Fixture'!C1:D3"]}),
    )?;
    run.api(
        "sheets",
        "values.batch-clear-by-data-filter",
        json!({"spreadsheetId":target,"dataFilters":[{"a1Range":"'Fixture'!A2:B3"}]}),
    )?;
    run.json("jsheets", &["clear", &target, "range:A1:D3", "tab:Fixture"])?;
    let cleared = run.json("jsheets", &["read", &target, "tab:Fixture", "range:A1:D3"])?;
    ensure!(
        cleared == json!([]) || cleared.is_null(),
        "cleared sheet still contains data"
    );
    Ok(())
}

pub(super) fn docs_dropdown(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let created = run.api("docs", "create", json!({"title":marker}))?;
    let target = id(&created, "documentId")?;
    own_file(run, &target)?;
    run.api("docs","create-dropdown-definition",json!({"documentId":target,"body":{"dropdownDefinition":{"dropdownDefinitionId":"kix.fixture01","dropdownDefinitionProperties":{"title":"Fixture","options":[{"optionId":"dropdownItem.one","displayValue":"One"},{"optionId":"dropdownItem.two","displayValue":"Two"}]}}}}))?;
    run.api("docs","insert-dropdown",json!({"documentId":target,"body":{"dropdownDefinitionId":"kix.fixture01","selectedOptionId":"dropdownItem.one","location":{"index":1}}}))?;
    let current = run.api("docs", "get", json!({"documentId":target}))?;
    fn span(value: &Value) -> Option<(i64, i64)> {
        if value.get("dropdown").is_some() {
            return Some((value["startIndex"].as_i64()?, value["endIndex"].as_i64()?));
        }
        match value {
            Value::Object(fields) => fields.values().find_map(span),
            Value::Array(items) => items.iter().find_map(span),
            _ => None,
        }
    }
    let (start, end) = span(&current).context("inserted dropdown span missing")?;
    run.api(
        "docs",
        "delete-content-range",
        json!({"documentId":target,"body":{"range":{"startIndex":start,"endIndex":end}}}),
    )?;
    run.api(
        "docs",
        "delete-dropdown-definition",
        json!({"documentId":target,"body":{"dropdownDefinitionId":"kix.fixture01"}}),
    )?;
    let removed = run.api("docs", "get", json!({"documentId":target}))?;
    ensure!(
        !removed.to_string().contains("kix.fixture01"),
        "dropdown definition was not removed"
    );
    Ok(())
}

pub(super) fn docs_import(run: &mut Live) -> Result<()> {
    let path = run.directory.path().join("tracked-changes.docx");
    fs::write(&path, include_bytes!("../data/live-document-fixture.docx"))?;
    let uploaded = run.api("drive","files.create",json!({"name":run.marker,"mimeType":"application/vnd.google-apps.document","upload":path,"contentType":"application/vnd.openxmlformats-officedocument.wordprocessingml.document","fields":"id,mimeType"}))?;
    let target = id(&uploaded, "id")?;
    own_file(run, &target)?;
    ensure!(
        uploaded["mimeType"] == "application/vnd.google-apps.document",
        "Word fixture was not converted to a Google document"
    );
    let document = run.api(
        "docs",
        "get",
        json!({"documentId":target,"suggestionsViewMode":"SUGGESTIONS_INLINE"}),
    )?;
    let positioned = document["positionedObjects"]
        .as_object()
        .and_then(|objects| objects.keys().next())
        .context("imported floating image missing")?
        .clone();
    run.api(
        "docs",
        "delete-positioned-object",
        json!({"documentId":target,"objectId":positioned}),
    )?;
    fn suggestions(value: &Value, output: &mut BTreeSet<String>) {
        match value {
            Value::Object(fields) => {
                for (key, value) in fields {
                    if key == "suggestedInsertionIds" {
                        output.extend(
                            value
                                .as_array()
                                .into_iter()
                                .flatten()
                                .filter_map(Value::as_str)
                                .map(str::to_owned),
                        );
                    }
                    suggestions(value, output);
                }
            }
            Value::Array(items) => {
                for item in items {
                    suggestions(item, output);
                }
            }
            _ => {}
        }
    }
    let mut ids = BTreeSet::new();
    suggestions(&document, &mut ids);
    ensure!(
        ids.len() >= 3,
        "imported tracked insertions did not produce three independent suggestions"
    );
    for (command, suggestion) in ["accept-suggestion", "reject-suggestion"]
        .into_iter()
        .zip(ids.iter())
    {
        run.api(
            "docs",
            command,
            json!({"documentId":target,"suggestionId":suggestion}),
        )?;
    }
    let third = ids.iter().nth(2).expect("three checked suggestions");
    let deleted = scoped_or_rejected(
        run,
        "docs",
        "delete-suggestion",
        json!({"documentId":target,"suggestionId":third}),
        &[403],
    )?
    .is_some();
    if !deleted && let Some(event) = run.events.last_mut() {
        event["reason"] = json!(
            "Google rejected deleting a suggestion retained under its imported external author. Accepting and rejecting suggestions were allowed for the document owner."
        );
    }
    let changed = run.api(
        "docs",
        "get",
        json!({"documentId":target,"suggestionsViewMode":"SUGGESTIONS_INLINE"}),
    )?;
    let mut remaining = BTreeSet::new();
    suggestions(&changed, &mut remaining);
    ensure!(
        ids.iter()
            .take(if deleted { 3 } else { 2 })
            .all(|id| !remaining.contains(id)),
        "suggestions were not resolved"
    );
    ensure!(
        deleted || remaining.contains(third),
        "rejected suggestion deletion unexpectedly changed the document"
    );
    ensure!(
        !changed["positionedObjects"]
            .as_object()
            .is_some_and(|objects| objects.contains_key(&positioned)),
        "floating image was not deleted"
    );
    Ok(())
}

pub(super) fn slides_media(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let deck = run.json("jslides", &["new", &marker])?;
    let target = id(&deck, "presentationId")?;
    own_file(run, &target)?;
    run.json(
        "jslides",
        &["add", &target, "title:Media fixtures", "body:Placeholder"],
    )?;
    // A newly created empty title/body placeholder must be writable.
    run.json(
        "jslides",
        &[
            "set",
            &target,
            "slide:1",
            "title:Updated fixture",
            "body:Updated body",
        ],
    )?;
    let deck = run.api("slides", "get", json!({"presentationId":target}))?;
    let page = deck
        .pointer("/slides/0/objectId")
        .and_then(Value::as_str)
        .context("fixture slide missing")?;
    let element = json!({"pageObjectId":page,"size":{"width":{"magnitude":120,"unit":"PT"},"height":{"magnitude":80,"unit":"PT"}},"transform":{"scaleX":1,"scaleY":1,"translateX":10,"translateY":10,"unit":"PT"}});
    for object in ["shape_start", "shape_end", "shape_chart", "shape_image"] {
        run.api("slides","create-shape",json!({"presentationId":target,"body":{"objectId":object,"shapeType":"RECTANGLE","elementProperties":element}}))?;
    }
    run.api("slides","create-line",json!({"presentationId":target,"body":{"objectId":"connector_fixture","lineCategory":"BENT","elementProperties":element}}))?;
    run.api("slides","update-line-properties",json!({"presentationId":target,"body":{"objectId":"connector_fixture","lineProperties":{"startConnection":{"connectedObjectId":"shape_start","connectionSiteIndex":0},"endConnection":{"connectedObjectId":"shape_end","connectionSiteIndex":2}},"fields":"startConnection,endConnection"}}))?;
    run.api(
        "slides",
        "reroute-line",
        json!({"presentationId":target,"body":{"objectId":"connector_fixture"}}),
    )?;
    run.api("slides","create-image",json!({"presentationId":target,"body":{"objectId":"image_fixture","url":"https://www.gstatic.com/images/branding/product/1x/docs_2020q4_48dp.png","elementProperties":element}}))?;
    run.api("slides","update-image-properties",json!({"presentationId":target,"body":{"objectId":"image_fixture","imageProperties":{"link":{"url":"https://developers.google.com/slides"}},"fields":"link"}}))?;
    run.api("slides","update-slide-properties",json!({"presentationId":target,"body":{"objectId":page,"slideProperties":{"isSkipped":true},"fields":"isSkipped"}}))?;
    for (object, text) in [
        ("shape_chart", "Replace chart fixture"),
        ("shape_image", "Replace image fixture"),
    ] {
        run.api("slides","insert-text",json!({"presentationId":target,"body":{"objectId":object,"text":text,"insertionIndex":0}}))?;
    }
    let chart = run.api("sheets","create",json!({"title":format!("{marker}-chart"),"sheets":[{"properties":{"sheetId":0,"title":"Chart"},"data":[{"rowData":[{"values":[{"userEnteredValue":{"stringValue":"Label"}},{"userEnteredValue":{"stringValue":"Value"}}]},{"values":[{"userEnteredValue":{"stringValue":"Fixture"}},{"userEnteredValue":{"numberValue":42}}]}]}],"charts":[{"chartId":700001,"spec":{"title":"Synthetic fixture chart","basicChart":{"chartType":"COLUMN","legendPosition":"NO_LEGEND","headerCount":1,"domains":[{"domain":{"sourceRange":{"sources":[{"sheetId":0,"startRowIndex":0,"endRowIndex":2,"startColumnIndex":0,"endColumnIndex":1}]}}}],"series":[{"series":{"sourceRange":{"sources":[{"sheetId":0,"startRowIndex":0,"endRowIndex":2,"startColumnIndex":1,"endColumnIndex":2}]}},"targetAxis":"LEFT_AXIS"}]}},"position":{"overlayPosition":{"anchorCell":{"sheetId":0,"rowIndex":4,"columnIndex":0}}}}]}]}))?;
    let spreadsheet = id(&chart, "spreadsheetId")?;
    own_file(run, &spreadsheet)?;
    let chart_id = chart
        .pointer("/sheets/0/charts/0/chartId")
        .and_then(Value::as_i64)
        .context("fixture chart ID missing")?;
    run.api("slides","create-sheets-chart",json!({"presentationId":target,"body":{"objectId":"chart_fixture","spreadsheetId":spreadsheet,"chartId":chart_id,"linkingMode":"LINKED","elementProperties":element}}))?;
    run.api(
        "slides",
        "refresh-sheets-chart",
        json!({"presentationId":target,"body":{"objectId":"chart_fixture"}}),
    )?;
    let replaced = run.api("slides","replace-all-shapes-with-sheets-chart",json!({"presentationId":target,"body":{"spreadsheetId":spreadsheet,"chartId":chart_id,"linkingMode":"LINKED","containsText":{"text":"Replace chart fixture","matchCase":true}}}))?;
    ensure!(
        replaced.to_string().contains("occurrencesChanged"),
        "chart replacement result missing"
    );
    run.api("slides","replace-all-shapes-with-image",json!({"presentationId":target,"body":{"imageUrl":"https://www.gstatic.com/images/branding/product/1x/docs_2020q4_48dp.png","containsText":{"text":"Replace image fixture","matchCase":true},"replaceMethod":"CENTER_INSIDE"}}))?;
    let video_file = run.directory.path().join("slide-video.mp4");
    fs::write(&video_file, include_bytes!("../data/live-fixture.mp4"))?;
    let video = run.json(
        "jdrive",
        &[
            "put",
            video_file.to_str().unwrap(),
            &format!("name:{marker}.mp4"),
        ],
    )?;
    let video_id = id(&video, "id")?;
    own_file(run, &video_id)?;
    run.api("slides","create-video",json!({"presentationId":target,"body":{"objectId":"video_fixture","id":video_id,"source":"DRIVE","elementProperties":element}}))?;
    run.api("slides","update-video-properties",json!({"presentationId":target,"body":{"objectId":"video_fixture","videoProperties":{"mute":true,"autoPlay":false},"fields":"mute,autoPlay"}}))?;
    let verified = run.api("slides", "get", json!({"presentationId":target}))?;
    contains(&verified, "chart_fixture")?;
    contains(&verified, "video_fixture")?;
    Ok(())
}
