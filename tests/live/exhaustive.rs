use super::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use std::collections::{BTreeMap, BTreeSet};

#[path = "coverage.rs"]
mod coverage;

pub(super) fn audit() -> Result<()> {
    coverage::audit()
}
#[path = "documents.rs"]
mod documents;
#[path = "media.rs"]
mod media;
#[path = "regressions.rs"]
mod regressions;

type Suite = (&'static str, fn(&mut Live) -> Result<()>);

fn own_file(run: &mut Live, id: &str) -> Result<()> {
    run.register(
        "drive",
        "files.delete",
        json!({"fileId":id,"supportsAllDrives":true}),
        id,
    )
}
fn removed(run: &mut Live, id: &str) -> Result<()> {
    run.pending.retain(|item| item.id != id);
    run.save()
}
fn contains(value: &Value, expected: &str) -> Result<()> {
    ensure!(
        value.to_string().contains(expected),
        "response did not contain the fixture value"
    );
    Ok(())
}
fn nonempty(value: &Value, field: &str) -> Result<()> {
    ensure!(
        value[field]
            .as_array()
            .is_some_and(|items| !items.is_empty()),
        "fixture collection {field} is empty"
    );
    Ok(())
}
fn scoped_or_rejected(
    run: &mut Live,
    service: &str,
    command: &str,
    params: Value,
    statuses: &[u16],
) -> Result<Option<Value>> {
    match run.api(service, command, params) {
        Ok(value) => Ok(Some(value)),
        Err(error)
            if statuses
                .iter()
                .any(|status| error.to_string().contains(&format!("HTTP {status}"))) =>
        {
            if let Some(event) = run.events.last_mut() {
                event["status"] = json!("expected-rejection");
            }
            run.scenario_failed = false;
            run.save()?;
            Ok(None)
        }
        Err(error) => Err(error),
    }
}
fn reason_or_rejected(
    run: &mut Live,
    service: &str,
    command: &str,
    params: Value,
    reasons: &[&str],
) -> Result<Option<Value>> {
    match run.api(service, command, params) {
        Ok(value) => Ok(Some(value)),
        Err(_)
            if run
                .last_remote_reason
                .as_deref()
                .is_some_and(|reason| reasons.contains(&reason)) =>
        {
            run.scenario_failed = false;
            if let Some(event) = run.events.last_mut() {
                event["status"] = json!("expected-rejection");
            }
            run.save()?;
            Ok(None)
        }
        Err(error) => Err(error),
    }
}
fn mime(run: &Live, subject: &str) -> String {
    let (local, domain) = run.account.split_once('@').expect("verified account");
    URL_SAFE_NO_PAD.encode(format!("From: {}\r\nTo: {}\r\nReply-To: {local}+joog-test@{domain}\r\nSubject: {subject}\r\nMessage-ID: <{}-{subject}@example.invalid>\r\nMIME-Version: 1.0\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nUnicode café\r\n",run.account,run.account,run.marker))
}

fn mail(run: &mut Live) -> Result<()> {
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("mail-draft") {
        return regressions::mail_draft_delete(run);
    }
    let marker = run.marker.clone();
    let account = run.account.clone();
    let profile = run.api("mail", "profile", json!({}))?;
    let history = id(&profile, "historyId")?;
    let label = run.api(
        "mail",
        "labels.create",
        json!({"name":marker,"labelListVisibility":"labelShow","messageListVisibility":"show"}),
    )?;
    let label_id = id(&label, "id")?;
    run.register("mail", "labels.delete", json!({"id":label_id}), &label_id)?;
    contains(
        &run.api("mail", "labels.get", json!({"id":label_id}))?,
        &marker,
    )?;
    run.api(
        "mail",
        "labels.patch",
        json!({"id":label_id,"name":format!("{marker}-patched")}),
    )?;
    run.api("mail","labels.update",json!({"id":label_id,"bodyId":label_id,"name":marker,"labelListVisibility":"labelShow","messageListVisibility":"show"}))?;
    contains(&run.api("mail", "labels.list", json!({}))?, &label_id)?;
    let mut messages = Vec::new();
    for method in ["messages.insert", "messages.import"] {
        let raw = if method == "messages.insert" {
            use base64::engine::general_purpose::STANDARD;
            let attachment = STANDARD.encode(vec![42u8; 32768]);
            let (local, domain) = account.split_once('@').expect("verified account");
            URL_SAFE_NO_PAD.encode(format!("From: {account}\r\nTo: {account}\r\nReply-To: {local}+joog-test@{domain}\r\nSubject: {marker}\r\nMessage-ID: <{marker}-attachment@example.invalid>\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=joog-fixture\r\n\r\n--joog-fixture\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nUnicode café\r\n--joog-fixture\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=fixture.bin\r\nContent-Transfer-Encoding: base64\r\n\r\n{attachment}\r\n--joog-fixture--\r\n"))
        } else {
            mime(run, &marker)
        };
        let result = run.api(
            "mail",
            method,
            json!({"raw":raw,"labelIds":["INBOX",label_id]}),
        )?;
        let message = id(&result, "id")?;
        run.register("mail", "messages.delete", json!({"id":message}), &message)?;
        messages.push(message);
    }
    let message = &messages[0];
    let value = run.api(
        "mail",
        "messages.get",
        json!({"id":message,"format":"full"}),
    )?;
    let thread = id(&value, "threadId")?;
    let attachment = value
        .pointer("/payload/parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find(|part| part["filename"] == "fixture.bin"))
        .and_then(|part| part.pointer("/body/attachmentId"))
        .and_then(Value::as_str)
        .context("fixture attachment ID missing")?;
    let fetched = run.api(
        "mail",
        "messages.attachments.get",
        json!({"messageId":message,"id":attachment}),
    )?;
    let bytes = URL_SAFE_NO_PAD.decode(id(&fetched, "data")?.trim_end_matches('='))?;
    ensure!(bytes == vec![42u8; 32768], "attachment bytes changed");
    run.api(
        "mail",
        "messages.modify",
        json!({"id":message,"addLabelIds":["UNREAD",label_id]}),
    )?;
    run.api(
        "mail",
        "messages.batch-modify",
        json!({"ids":messages,"addLabelIds":["STARRED"]}),
    )?;
    for action in ["read", "unread", "star", "unstar"] {
        run.api("mail", action, json!({"id":message}))?;
    }
    run.api("mail", "messages.trash", json!({"id":message}))?;
    run.api("mail", "messages.untrash", json!({"id":message}))?;
    contains(
        &run.api(
            "mail",
            "messages.list",
            json!({"q":format!("subject:{marker}"),"max":2}),
        )?,
        message,
    )?;
    run.api(
        "mail",
        "history.list",
        json!({"startHistoryId":history,"max":2}),
    )?;
    for action in [
        "threads.get",
        "threads.modify",
        "threads.trash",
        "threads.untrash",
    ] {
        let mut params = json!({"id":thread});
        if action == "threads.modify" {
            params["addLabelIds"] = json!([label_id]);
        }
        run.api("mail", action, params)?;
    }
    run.api(
        "mail",
        "threads.list",
        json!({"q":format!("subject:{marker}"),"max":2}),
    )?;
    let draft = run.json(
        "jmail",
        &[
            "draft",
            &format!("to:{account}"),
            &format!("subject:{marker}"),
            "body:Draft fixture",
        ],
    )?;
    let draft_id = id(&draft, "id")?;
    run.register("mail", "drafts.delete", json!({"id":draft_id}), &draft_id)?;
    run.api("mail", "drafts.get", json!({"id":draft_id}))?;
    run.api("mail", "drafts.list", json!({"max":1}))?;
    run.api(
        "mail",
        "drafts.update",
        json!({"id":draft_id,"to":[account],"subject":marker,"text":"Updated draft"}),
    )?;
    run.json("jmail", &["drafts", "limit:1"])?;
    run.json("jmail", &["inbox", "limit:2"])?;
    contains(
        &run.json("jmail", &["find", &format!("subject:{marker}"), "limit:2"])?,
        message,
    )?;
    run.json("jmail", &["read", message])?;
    run.json("jmail", &["mark", message, "unread"])?;
    run.json("jmail", &["trash", message])?;
    run.api("mail", "messages.untrash", json!({"id":message}))?;
    let replied = run.json(
        "jmail",
        &["reply", message, "body:Self-addressed reply fixture"],
    )?;
    let replied_id = id(&replied, "id")?;
    run.register(
        "mail",
        "messages.delete",
        json!({"id":replied_id}),
        &replied_id,
    )?;
    // Each delivery command is invoked once, addressed only to the test account.
    let sent = run.json(
        "jmail",
        &[
            "send",
            &format!("to:{account}"),
            &format!("subject:{marker}"),
            "body:Self-addressed CLI test",
        ],
    )?;
    let sent_id = id(&sent, "id")?;
    run.register("mail", "messages.delete", json!({"id":sent_id}), &sent_id)?;
    let direct_sent = run.api(
        "mail",
        "messages.send",
        json!({"raw":mime(run,&format!("{marker}-direct"))}),
    )?;
    let direct_sent_id = id(&direct_sent, "id")?;
    run.register(
        "mail",
        "messages.delete",
        json!({"id":direct_sent_id}),
        &direct_sent_id,
    )?;
    let direct_draft=run.api("mail","drafts.create",json!({"to":[account],"subject":format!("{marker}-direct-draft"),"text":"Self-addressed draft send"}))?;
    let direct_draft_id = id(&direct_draft, "id")?;
    run.register(
        "mail",
        "drafts.delete",
        json!({"id":direct_draft_id}),
        &direct_draft_id,
    )?;
    let direct_delivered = run.api("mail", "drafts.send", json!({"id":direct_draft_id}))?;
    removed(run, &direct_draft_id)?;
    let direct_delivered_id = id(&direct_delivered, "id")?;
    run.register(
        "mail",
        "messages.delete",
        json!({"id":direct_delivered_id}),
        &direct_delivered_id,
    )?;
    let sent_draft = run.json("jmail", &["send-draft", &draft_id])?;
    removed(run, &draft_id)?;
    let sent_draft_id = id(&sent_draft, "id")?;
    run.register(
        "mail",
        "messages.delete",
        json!({"id":sent_draft_id}),
        &sent_draft_id,
    )?;
    let forwarded = run.json(
        "jmail",
        &[
            "forward",
            message,
            &format!("to:{account}"),
            "body:Self-addressed forwarding test",
        ],
    )?;
    let forwarded_id = id(&forwarded, "id")?;
    run.register(
        "mail",
        "messages.delete",
        json!({"id":forwarded_id}),
        &forwarded_id,
    )?;
    let raw = run.api(
        "mail",
        "messages.insert",
        json!({"raw":mime(run,&format!("{marker}-delete"))}),
    )?;
    let delete_id = id(&raw, "id")?;
    run.register(
        "mail",
        "messages.delete",
        json!({"id":delete_id}),
        &delete_id,
    )?;
    run.api("mail", "messages.batch-delete", json!({"ids":[delete_id]}))?;
    removed(run, &delete_id)?;
    let isolated = run.api(
        "mail",
        "messages.insert",
        json!({"raw":mime(run,&format!("{marker}-thread-delete"))}),
    )?;
    let isolated_message = id(&isolated, "id")?;
    let isolated_thread = id(&isolated, "threadId")?;
    run.register(
        "mail",
        "messages.delete",
        json!({"id":isolated_message}),
        &isolated_message,
    )?;
    run.api("mail", "threads.delete", json!({"id":isolated_thread}))?;
    removed(run, &isolated_message)?;
    regressions::mail_draft_delete(run)?;
    Ok(())
}

fn drive(run: &mut Live) -> Result<()> {
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("drive-watch") {
        return regressions::drive_watch(run);
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("drive-shared") {
        return regressions::drive_shared(run);
    }
    let marker = run.marker.clone();
    let account = run.account.clone();
    let path = run.directory.path().join("fixture.txt");
    fs::write(&path, b"joog fixture: Unicode caf\xc3\xa9\n")?;
    let uploaded = run.json(
        "jdrive",
        &["put", path.to_str().unwrap(), &format!("name:{marker}.txt")],
    )?;
    let file = id(&uploaded, "id")?;
    own_file(run, &file)?;
    let folder = run.json("jdrive", &["mkdir", &format!("{marker}-folder")])?;
    let folder_id = id(&folder, "id")?;
    own_file(run, &folder_id)?;
    let root = run.api(
        "drive",
        "files.get",
        json!({"fileId":file,"fields":"id,parents"}),
    )?;
    let parent = root
        .pointer("/parents/0")
        .and_then(Value::as_str)
        .context("fixture parent missing")?
        .to_owned();
    run.json("jdrive", &["move", &file, &format!("to:{folder_id}")])?;
    contains(
        &run.json("jdrive", &["ls", &format!("folder:{folder_id}"), "limit:2"])?,
        &file,
    )?;
    run.json("jdrive", &["find", &marker, "limit:2"])?;
    run.api("drive", "search", json!({"query":marker,"max":2}))?;
    let copy = run.json("jdrive", &["copy", &file, &format!("name:{marker}-copy")])?;
    let copied = id(&copy, "id")?;
    own_file(run, &copied)?;
    let out = run.directory.path().join("download.txt");
    run.json("jdrive", &["get", &file, &format!("out:{}", out.display())])?;
    ensure!(
        fs::read(&out)? == fs::read(&path)?,
        "Drive download changed bytes"
    );
    run.json("jdrive", &["info", &file])?;
    run.json("jdrive", &["trash", &copied])?;
    run.api("drive", "restore", json!({"id":copied}))?;
    run.json("jdrive", &["trash", &copied])?;
    run.json("jdrive", &["restore", &copied])?;
    run.json("jdrive", &["move", &file, &format!("to:{parent}")])?;
    // Sharing with the same approved account sends no notification.
    run.json(
        "jdrive",
        &[
            "share",
            &file,
            &format!("to:{account}"),
            "role:reader",
            "notify:false",
        ],
    )?;
    let permissions = run.api(
        "drive",
        "permissions.list",
        json!({"fileId":file,"fields":"permissions(id,role,emailAddress)"}),
    )?;
    nonempty(&permissions, "permissions")?;
    let permission = permissions
        .pointer("/permissions/0/id")
        .and_then(Value::as_str)
        .context("permission missing")?;
    run.api(
        "drive",
        "permissions.get",
        json!({"fileId":file,"permissionId":permission}),
    )?;
    // Link sharing applies only to this synthetic file and sends no invitations.
    let shared = run.api("drive","permissions.create",json!({"fileId":file,"type":"anyone","role":"reader","allowFileDiscovery":false,"fields":"id,role"}))?;
    let shared_id = id(&shared, "id")?;
    let updated = run.api("drive","permissions.update",json!({"fileId":file,"permissionId":shared_id,"role":"reader","allowFileDiscovery":false,"fields":"id,role"}))?;
    ensure!(
        updated["role"] == "reader",
        "fixture sharing role changed unexpectedly"
    );
    run.api(
        "drive",
        "permissions.delete",
        json!({"fileId":file,"permissionId":shared_id}),
    )?;
    let comment = run.api(
        "drive",
        "comments.create",
        json!({"fileId":file,"content":marker,"fields":"id,content"}),
    )?;
    let comment_id = id(&comment, "id")?;
    contains(
        &run.api(
            "drive",
            "comments.get",
            json!({"fileId":file,"commentId":comment_id,"fields":"id,content"}),
        )?,
        &marker,
    )?;
    run.api("drive","comments.update",json!({"fileId":file,"commentId":comment_id,"content":"Updated fixture comment","fields":"id,content"}))?;
    run.api(
        "drive",
        "comments.list",
        json!({"fileId":file,"fields":"comments(id)","max":2}),
    )?;
    let reply=run.api("drive","replies.create",json!({"fileId":file,"commentId":comment_id,"content":"Fixture reply","fields":"id,content"}))?;
    let reply_id = id(&reply, "id")?;
    for method in [
        "replies.get",
        "replies.update",
        "replies.list",
        "replies.delete",
    ] {
        let mut params = json!({"fileId":file,"commentId":comment_id});
        if method != "replies.list" {
            params["replyId"] = json!(reply_id);
        }
        if method == "replies.update" {
            params["content"] = json!("Updated fixture reply");
        }
        if method != "replies.delete" {
            params["fields"] = json!(if method == "replies.list" {
                "replies(id)"
            } else {
                "id,content"
            });
        }
        run.api("drive", method, params)?;
    }
    run.api(
        "drive",
        "comments.delete",
        json!({"fileId":file,"commentId":comment_id}),
    )?;
    let token = run.api("drive", "changes.get-start-page-token", json!({}))?;
    run.api(
        "drive",
        "changes.list",
        json!({"pageToken":id(&token,"startPageToken")?,"max":1}),
    )?;
    run.api(
        "drive",
        "files.generate-ids",
        json!({"count":1,"space":"drive","type":"files"}),
    )?;
    run.api("drive", "files.list-labels", json!({"fileId":file,"max":1}))?;
    let revisions = run.api(
        "drive",
        "revisions.list",
        json!({"fileId":file,"fields":"revisions(id)","max":2}),
    )?;
    if let Some(revision) = revisions.pointer("/revisions/0/id").and_then(Value::as_str) {
        run.api(
            "drive",
            "revisions.get",
            json!({"fileId":file,"revisionId":revision,"fields":"id,mimeType"}),
        )?;
        run.api("drive","revisions.update",json!({"fileId":file,"revisionId":revision,"keepForever":true,"fields":"id,keepForever"}))?;
        fs::write(&path, b"Second synthetic revision\n")?;
        run.api(
            "drive",
            "files.update",
            json!({"fileId":file,"upload":path,"contentType":"text/plain","fields":"id"}),
        )?;
        run.api(
            "drive",
            "revisions.delete",
            json!({"fileId":file,"revisionId":revision}),
        )?;
    }
    if let Some(operation) = scoped_or_rejected(
        run,
        "drive",
        "files.download",
        json!({"fileId":file}),
        &[400, 403],
    )? {
        let name = id(&operation, "name")?;
        run.api("drive", "operations.get", json!({"name":name}))?;
    }
    regressions::drive_shared(run)?;
    regressions::drive_watch(run)?;
    Ok(())
}

fn calendar(run: &mut Live) -> Result<()> {
    match std::env::var("JOOG_LIVE_CASE").as_deref() {
        Ok("calendar-subscription") => return regressions::calendar_subscription(run),
        Ok("calendar-watch") => return regressions::calendar_watch(run),
        _ => {}
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("calendar-owner") {
        return regressions::calendar_owner(run);
    }
    let marker = run.marker.clone();
    let created = run.api(
        "calendar",
        "calendars.insert",
        json!({"summary":marker,"timeZone":"Etc/UTC"}),
    )?;
    let calendar = id(&created, "id")?;
    run.register(
        "calendar",
        "calendars.delete",
        json!({"calendarId":calendar}),
        &calendar,
    )?;
    run.api(
        "calendar",
        "calendars.patch",
        json!({"calendarId":calendar,"description":"Fixture calendar"}),
    )?;
    run.api(
        "calendar",
        "calendars.update",
        json!({"calendarId":calendar,"summary":marker,"timeZone":"Etc/UTC"}),
    )?;
    run.api("calendar", "calendars.get", json!({"calendarId":calendar}))?;
    run.api("calendar", "colors.get", json!({}))?;
    run.api(
        "calendar",
        "calendar-list.get",
        json!({"calendarId":calendar}),
    )?;
    run.api(
        "calendar",
        "calendar-list.patch",
        json!({"calendarId":calendar,"hidden":true}),
    )?;
    run.api(
        "calendar",
        "calendar-list.update",
        json!({"calendarId":calendar,"hidden":false,"selected":true}),
    )?;
    scoped_or_rejected(
        run,
        "calendar",
        "calendar-list.delete",
        json!({"calendarId":calendar}),
        &[403],
    )?;
    scoped_or_rejected(
        run,
        "calendar",
        "calendar-list.insert",
        json!({"id":calendar}),
        &[409],
    )?;
    let calendar_arg = format!("calendar:{calendar}");
    let event = run.json(
        "jcal",
        &[
            "add",
            &marker,
            "at:tomorrow 10:00",
            "for:30m",
            &calendar_arg,
            "timezone:Etc/UTC",
        ],
    )?;
    let event_id = id(&event, "id")?;
    run.json("jcal", &["show", &event_id, &calendar_arg])?;
    run.json(
        "jcal",
        &[
            "move",
            &event_id,
            "at:tomorrow 11:00",
            &calendar_arg,
            "timezone:Etc/UTC",
        ],
    )?;
    for action in ["today", "week"] {
        run.json("jcal", &[action, &calendar_arg, "timezone:Etc/UTC"])?;
    }
    contains(
        &run.json(
            "jcal",
            &[
                "find",
                &marker,
                &calendar_arg,
                "limit:2",
                "timezone:Etc/UTC",
            ],
        )?,
        &event_id,
    )?;
    run.json(
        "jcal",
        &[
            "free",
            &format!("with:{}", run.account.clone()),
            &calendar_arg,
            "on:tomorrow",
            "timezone:Etc/UTC",
        ],
    )?;
    let start = jiff::Timestamp::now().checked_add(jiff::SignedDuration::from_hours(24))?;
    let end = start.checked_add(jiff::SignedDuration::from_hours(1))?;
    let recurring=run.api("calendar","events.insert",json!({"calendarId":calendar,"summary":marker,"startTime":start.to_string(),"endTime":end.to_string(),"startTimeZone":"Etc/UTC","endTimeZone":"Etc/UTC","recurrence":["RRULE:FREQ=DAILY;COUNT=2"],"remindersUseDefault":false,"sendUpdates":"none"}))?;
    let recurring_id = id(&recurring, "id")?;
    nonempty(
        &run.api(
            "calendar",
            "events.instances",
            json!({"calendarId":calendar,"eventId":recurring_id,"max":2}),
        )?,
        "items",
    )?;
    run.api(
        "calendar",
        "events.patch",
        json!({"calendarId":calendar,"eventId":event_id,"description":"Patched fixture"}),
    )?;
    run.api("calendar","events.update",json!({"calendarId":calendar,"eventId":event_id,"summary":marker,"startTime":start.to_string(),"endTime":end.to_string(),"sendUpdates":"none"}))?;
    run.api("calendar","events.import",json!({"calendarId":calendar,"summary":marker,"iCalUid":format!("{}@example.invalid",marker),"startTime":start.to_string(),"endTime":end.to_string()}))?;
    run.api("calendar","events.quick-add",json!({"calendarId":calendar,"text":format!("{marker} tomorrow 14:00"),"sendUpdates":"none"}))?;
    let destination = run.api(
        "calendar",
        "calendars.insert",
        json!({"summary":format!("{marker}-destination"),"timeZone":"Etc/UTC"}),
    )?;
    let destination_id = id(&destination, "id")?;
    run.register(
        "calendar",
        "calendars.delete",
        json!({"calendarId":destination_id}),
        &destination_id,
    )?;
    let moved = run.api("calendar","events.move",json!({"calendarId":calendar,"eventId":recurring_id,"destination":destination_id,"sendUpdates":"none"}))?;
    ensure!(
        moved["id"] == recurring_id,
        "moved recurring event changed its ID"
    );
    run.json("jcal", &["calendars", "limit:1"])?;
    run.api(
        "calendar",
        "events",
        json!({"calendar":calendar,"from":start.to_string(),"max":2}),
    )?;
    run.json(
        "jcal",
        &["cancel", &event_id, &calendar_arg, "notify:false"],
    )?;
    scoped_or_rejected(
        run,
        "calendar",
        "calendars.clear",
        json!({"calendarId":calendar}),
        &[400, 403],
    )?;
    scoped_or_rejected(
        run,
        "calendar",
        "calendars.transfer-ownership",
        json!({"calendarId":calendar,"newDataOwner":run.account,"useAdminAccess":true}),
        &[400, 403],
    )?;
    regressions::calendar_subscription(run)?;
    regressions::calendar_watch(run)?;
    Ok(())
}

fn sheets(run: &mut Live) -> Result<()> {
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("csv") {
        return regressions::sheets_csv(run);
    }
    let marker = run.marker.clone();
    let value = run.json("jsheets", &["new", &marker, "tab:Test"])?;
    let target = id(&value, "spreadsheetId")?;
    own_file(run, &target)?;
    run.api(
        "sheets",
        "get",
        json!({"spreadsheetId":target,"fields":"spreadsheetId,sheets.properties"}),
    )?;
    run.json("jsheets", &["ls", "limit:1"])?;
    run.json(
        "jsheets",
        &["set", &target, "cell:A1", "value:literal =1+1", "tab:Test"],
    )?;
    let csv = run.directory.path().join("cells.csv");
    fs::write(&csv, "one,two\nthree,four\n")?;
    run.json(
        "jsheets",
        &[
            "load",
            &target,
            &format!("csv:{}", csv.display()),
            "at:A1",
            "tab:Test",
        ],
    )?;
    run.json("jsheets", &["append", &target, "row:five,six", "tab:Test"])?;
    let range = "'Test'!A1:B3";
    run.api("sheets","values.update",json!({"spreadsheetId":target,"range":"'Test'!C1","valueInputOption":"RAW","values":[["Unicode café"]]}))?;
    run.api("sheets","values.append",json!({"spreadsheetId":target,"range":"'Test'!A:B","valueInputOption":"RAW","values":[["Appended via API","fixture"]]}))?;
    for method in ["values.batch-get", "values.batch-get-by-data-filter"] {
        let params = if method == "values.batch-get" {
            json!({"spreadsheetId":target,"ranges":[range]})
        } else {
            json!({"spreadsheetId":target,"dataFilters":[{"a1Range":range}]})
        };
        contains(&run.api("sheets", method, params)?, "one")?;
    }
    run.api("sheets","values.batch-update",json!({"spreadsheetId":target,"valueInputOption":"RAW","data":[{"range":"'Test'!D1","values":[["batch"]]}]}))?;
    run.api("sheets","values.batch-update-by-data-filter",json!({"spreadsheetId":target,"valueInputOption":"RAW","data":[{"dataFilter":{"a1Range":"'Test'!D2"},"values":[["filtered"]]}]}))?;
    let other = run.api(
        "sheets",
        "create",
        json!({"title":format!("{marker}-copy")}),
    )?;
    let other_id = id(&other, "spreadsheetId")?;
    own_file(run, &other_id)?;
    run.api(
        "sheets",
        "sheets.copy-to",
        json!({"spreadsheetId":target,"sheetId":value.pointer("/sheets/0/properties/sheetId").context("created tab ID missing")?,"destinationSpreadsheetId":other_id}),
    )?;
    let out = run.directory.path().join("export.csv");
    run.json(
        "jsheets",
        &[
            "export",
            &target,
            "tab:Test",
            "format:csv",
            &format!("out:{}", out.display()),
        ],
    )?;
    ensure!(
        fs::read_to_string(out)?.contains("one,two"),
        "CSV export content missing"
    );
    run.api(
        "sheets",
        "values.batch-clear",
        json!({"spreadsheetId":target,"ranges":["'Test'!C1:D2"]}),
    )?;
    run.api(
        "sheets",
        "values.batch-clear-by-data-filter",
        json!({"spreadsheetId":target,"dataFilters":[{"a1Range":"'Test'!A2:B3"}]}),
    )?;
    run.json("jsheets", &["clear", &target, "range:A1:B3", "tab:Test"])?;
    Ok(())
}

fn tasks(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let list = run.json("jtasks", &["new-list", &marker])?;
    let list_id = id(&list, "id")?;
    run.register(
        "tasks",
        "tasklists.delete",
        json!({"tasklist":list_id}),
        &list_id,
    )?;
    run.api("tasks", "tasklists.get", json!({"tasklist":list_id}))?;
    run.api(
        "tasks",
        "tasklists.patch",
        json!({"tasklist":list_id,"title":format!("{marker}-patch")}),
    )?;
    run.api(
        "tasks",
        "tasklists.update",
        json!({"tasklist":list_id,"id":list_id,"title":marker}),
    )?;
    let list = format!("list:{list_id}");
    let task = run.json("jtasks", &["add", &marker, &list])?;
    let target = id(&task, "id")?;
    run.json(
        "jtasks",
        &["set", &target, &list, "notes:fixture notes", "due:tomorrow"],
    )?;
    run.api("tasks","tasks.update",json!({"tasklist":list_id,"task":target,"id":target,"title":marker,"notes":"Updated via put"}))?;
    run.api(
        "tasks",
        "tasks.move",
        json!({"tasklist":list_id,"task":target}),
    )?;
    contains(&run.json("jtasks", &["ls", &list, "limit:2"])?, &target)?;
    run.json("jtasks", &["done", &target, &list])?;
    run.json("jtasks", &["undo", &target, &list])?;
    run.json("jtasks", &["show", &target, &list])?;
    run.json("jtasks", &["delete", &target, &list])?;
    let done = run.api(
        "tasks",
        "tasks.insert",
        json!({"tasklist":list_id,"title":"clear test","status":"completed"}),
    )?;
    ensure!(
        done["status"] == "completed",
        "task completion was not saved"
    );
    run.api("tasks", "tasks.clear", json!({"tasklist":list_id}))?;
    run.json("jtasks", &["lists", "limit:1"])?;
    Ok(())
}

fn contacts(run: &mut Live) -> Result<()> {
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("contacts-other") {
        return regressions::contacts_other(run);
    }
    let marker = run.marker.clone();
    let contact = run.json("jcontacts", &["new", &marker, "phone:+33123456789"])?;
    let target = id(&contact, "id")?;
    run.register(
        "contacts",
        "people.delete-contact",
        json!({"resourceName":target}),
        &target,
    )?;
    run.json("jcontacts", &["set", &target, "company:Fixture company"])?;
    contains(&run.json("jcontacts", &["show", &target])?, &marker)?;
    run.json("jcontacts", &["find", &marker, "limit:1"])?;
    run.json("jcontacts", &["ls", "limit:1"])?;
    use base64::engine::general_purpose::STANDARD;
    run.api("contacts","people.update-contact-photo",json!({"resourceName":target,"photoBytes":STANDARD.encode(include_bytes!("../data/live-fixture.png")),"personFields":"photos"}))?;
    run.api(
        "contacts",
        "people.delete-contact-photo",
        json!({"resourceName":target,"personFields":"photos"}),
    )?;
    run.api(
        "contacts",
        "other-contacts.list",
        json!({"readMask":"names,emailAddresses","max":1}),
    )?;
    run.api(
        "contacts",
        "other-contacts.search",
        json!({"query":marker,"readMask":"names,emailAddresses","max":1}),
    )?;
    scoped_or_rejected(
        run,
        "contacts",
        "people.list-directory-people",
        json!({"sources":["DIRECTORY_SOURCE_TYPE_DOMAIN_PROFILE"],"readMask":"names,emailAddresses","max":1}),
        &[400, 403],
    )?;
    scoped_or_rejected(
        run,
        "contacts",
        "people.search-directory-people",
        json!({"query":marker,"sources":["DIRECTORY_SOURCE_TYPE_DOMAIN_PROFILE"],"readMask":"names,emailAddresses","max":1}),
        &[400, 403],
    )?;
    run.api(
        "contacts",
        "people.get-batch-get",
        json!({"resourceNames":[target],"personFields":"names,phoneNumbers,organizations"}),
    )?;
    let group = run.api(
        "contacts",
        "contact-groups.create",
        json!({"contactGroup":{"name":marker},"readGroupFields":"name,metadata"}),
    )?;
    let group_id = id(&group, "resourceName")?;
    run.register(
        "contacts",
        "contact-groups.delete",
        json!({"resourceName":group_id}),
        &group_id,
    )?;
    run.api(
        "contacts",
        "contact-groups.members.modify",
        json!({"resourceName":group_id,"resourceNamesToAdd":[target]}),
    )?;
    let current_group = run.api(
        "contacts",
        "contact-groups.get",
        json!({"resourceName":group_id,"maxMembers":2}),
    )?;
    run.api(
        "contacts",
        "contact-groups.batch-get",
        json!({"resourceNames":[group_id],"maxMembers":2}),
    )?;
    run.api("contacts", "contact-groups.list", json!({"max":1}))?;
    run.api("contacts","contact-groups.update",json!({"resourceName":group_id,"contactGroup":{"resourceName":group_id,"etag":current_group["etag"],"name":format!("{marker}-changed")},"updateGroupFields":"name"}))?;
    run.api(
        "contacts",
        "contact-groups.members.modify",
        json!({"resourceName":group_id,"resourceNamesToRemove":[target]}),
    )?;
    let batch=run.api("contacts","people.batch-create-contacts",json!({"contacts":[{"contactPerson":{"names":[{"givenName":format!("{marker}-batch")}]}}],"readMask":"names,metadata"}))?;
    let person = batch
        .pointer("/createdPeople/0/person")
        .context("batch contact missing")?;
    let batch_id = id(person, "resourceName")?;
    run.register(
        "contacts",
        "people.delete-contact",
        json!({"resourceName":batch_id}),
        &batch_id,
    )?;
    let mut person = person.clone();
    person["names"] = json!([{"givenName":format!("{marker}-updated")}]);
    run.api(
        "contacts",
        "people.batch-update-contacts",
        json!({"contacts":{batch_id.clone():person},"updateMask":"names","readMask":"names"}),
    )?;
    run.api(
        "contacts",
        "people.batch-delete-contacts",
        json!({"resourceNames":[batch_id]}),
    )?;
    removed(run, &batch_id)?;
    run.json("jcontacts", &["delete", &target])?;
    removed(run, &target)?;
    regressions::contacts_other(run)?;
    Ok(())
}

fn forms(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let form = run.json("jforms", &["new", &marker])?;
    let target = id(&form, "formId")?;
    own_file(run, &target)?;
    run.json(
        "jforms",
        &[
            "question",
            &target,
            "Test question",
            "type:text",
            "required:true",
        ],
    )?;
    run.json("jforms", &["show", &target])?;
    run.json("jforms", &["publish", &target])?;
    let form = run.api("forms", "get", json!({"formId":target}))?;
    let question = form
        .pointer("/items/0/questionItem/question/questionId")
        .and_then(Value::as_str)
        .context("fixture question ID missing")?;
    let entry = u64::from_str_radix(question, 16).context("invalid fixture question ID")?;
    let responder = url::Url::parse(
        form["responderUri"]
            .as_str()
            .context("fixture responder URI missing")?,
    )?;
    ensure!(
        responder.scheme() == "https"
            && responder.host_str() == Some("docs.google.com")
            && responder.path().starts_with("/forms/"),
        "unexpected responder URL"
    );
    let endpoint = responder
        .as_str()
        .split('?')
        .next()
        .unwrap()
        .trim_end_matches("viewform")
        .to_owned()
        + "formResponse";
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let response = client
        .post(endpoint)
        .form(&[
            (format!("entry.{entry}"), marker.clone()),
            ("pageHistory".into(), "0".into()),
            ("fvv".into(), "1".into()),
        ])
        .send()
        .map_err(|error| error.without_url())?;
    ensure!(
        response.status().is_success(),
        "fixture form submission returned HTTP {}",
        response.status().as_u16()
    );
    let responses = run.api("forms", "responses.list", json!({"formId":target,"max":1}))?;
    let response_id = responses
        .pointer("/responses/0/responseId")
        .and_then(Value::as_str)
        .context("fixture form response missing")?;
    contains(
        &run.api(
            "forms",
            "responses.get",
            json!({"formId":target,"responseId":response_id}),
        )?,
        &marker,
    )?;
    contains(
        &run.api(
            "forms",
            "response",
            json!({"id":target,"responseId":response_id}),
        )?,
        &marker,
    )?;
    run.json("jforms", &["close", &target])?;
    run.json("jforms", &["open", &target])?;
    run.json("jforms", &["close", &target])?;
    run.json("jforms", &["answers", &target, "limit:1"])?;
    run.api("forms", "responses", json!({"id":target,"max":1}))?;
    run.json("jforms", &["ls", "limit:1"])?;
    Ok(())
}

fn docs(run: &mut Live) -> Result<()> {
    match std::env::var("JOOG_LIVE_CASE").as_deref() {
        Ok("dropdown") => regressions::docs_dropdown(run),
        Ok("docs-import") => regressions::docs_import(run),
        _ => {
            documents::docs(run)?;
            regressions::docs_import(run)
        }
    }
}

fn slides(run: &mut Live) -> Result<()> {
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("slides-media") {
        regressions::slides_media(run)
    } else {
        documents::slides(run)?;
        regressions::slides_media(run)
    }
}

pub(super) fn run() -> Result<()> {
    let selected_value = std::env::var("JOOG_LIVE_SERVICES").ok();
    let case = std::env::var("JOOG_LIVE_CASE").ok();
    let selected = coverage::selection(selected_value.as_deref(), case.as_deref())?;
    let mut run = Live::new()?;
    verify_account(&mut run)?;
    let suites: [Suite; 11] = [
        ("mail", mail),
        ("drive", drive),
        ("calendar", calendar),
        ("sheets", sheets),
        ("tasks", tasks),
        ("contacts", contacts),
        ("forms", forms),
        ("docs", docs),
        ("slides", slides),
        ("youtube", media::youtube),
        ("photos", media::photos),
    ];
    for (name, suite) in suites {
        if selected.as_ref().is_some_and(|names| !names.contains(name)) {
            continue;
        }
        let before = run.events.len();
        run.scenario_failed = false;
        let result = if run.stop {
            Err(anyhow::anyhow!(
                "stopped after a previous account, quota, or cleanup error"
            ))
        } else {
            suite(&mut run)
        };
        let passed = result.is_ok()
            && !run.events[before..]
                .iter()
                .any(|event| event["status"] == "failed");
        writeln!(
            std::io::stderr().lock(),
            "full live {name}: {}",
            if passed { "passed" } else { "failed" }
        )?;
        run.events.push(json!({"kind":"scenario","service":name,"status":if passed{"passed"}else{"failed"},"error":result.err().map(|error|error.to_string())}));
        run.cleanup()?;
    }
    // This token does not create a file. Check it after fixture cleanup so an
    // account/provider failure cannot strand otherwise completed resources.
    if !run.stop
        && case.is_none()
        && selected
            .as_ref()
            .is_none_or(|names| names.contains("drive"))
    {
        run.scenario_failed = false;
        let _ = scoped_or_rejected(
            &mut run,
            "drive",
            "files.generate-cse-token",
            json!({}),
            &[400, 403],
        );
    }
    let coverage = coverage::summarize(&run.events, selected.as_ref())?;
    let complete = coverage["status"] == "passed";
    run.events.push(coverage);
    run.save()?;
    ensure!(
        run.pending.is_empty(),
        "cleanup remains pending; inspect {}",
        run.report.display()
    );
    ensure!(
        !run.events.iter().any(|event| event["status"] == "failed"),
        "live scenarios failed; inspect {}",
        run.report.display()
    );
    ensure!(
        selected.is_some() || complete,
        "live command coverage is incomplete; inspect the missing commands in {}",
        run.report.display()
    );
    Ok(())
}
