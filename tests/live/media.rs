use super::*;

fn blocked(run: &mut Live, service: &str, commands: &[&str], reason: &str) -> Result<()> {
    for command in commands {
        run.events.push(json!({"kind":"prerequisite","service":service,"command":command,"status":"blocked","reason":reason}));
    }
    run.save()
}
fn attempt(run: &mut Live, command: &str, params: Value) -> Option<Value> {
    if run.stop || run.scenario_failed {
        return None;
    }
    scoped_or_rejected(run, "youtube", command, params, &[403])
        .ok()
        .flatten()
}

pub(super) fn youtube(run: &mut Live) -> Result<()> {
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("youtube-channel") {
        return channel_round_trip(run);
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("youtube-banner") {
        return youtube_banner(run);
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("youtube-watermark") {
        return youtube_watermark(run);
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("youtube-sections") {
        return youtube_sections(run);
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("youtube-image-update") {
        return youtube_image_update(run);
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("youtube-capabilities") {
        return youtube_capabilities(run);
    }
    if std::env::var("JOOG_LIVE_CASE").as_deref() == Ok("youtube-remaining") {
        return youtube_remaining(run);
    }
    let channels = run.api(
        "youtube",
        "channels.list",
        json!({"mine":true,"part":"snippet,contentDetails,brandingSettings","max":1}),
    )?;
    run.json("jyt", &["info"])?;
    let search=run.api("youtube","search.list",json!({"q":"Google Developers","type":"video","videoEmbeddable":"true","part":"snippet","max":1}))?;
    let public_video = search
        .pointer("/items/0/id/videoId")
        .and_then(Value::as_str)
        .context("YouTube search returned no video")?
        .to_owned();
    run.api(
        "youtube",
        "search",
        json!({"query":"Google Developers","max":1}),
    )?;
    run.json("jyt", &["find", "Google Developers", "limit:1"])?;
    run.json("jyt", &["info", &public_video])?;
    run.json("jyt", &["comments", &public_video, "limit:1"])?;
    run.api(
        "youtube",
        "videos.list",
        json!({"id":public_video,"part":"snippet,statistics"}),
    )?;
    run.api(
        "youtube",
        "comment-threads.list",
        json!({"videoId":public_video,"part":"snippet","max":1}),
    )?;
    run.api(
        "youtube",
        "video-categories.list",
        json!({"part":"snippet","regionCode":"US"}),
    )?;
    scoped_or_rejected(
        run,
        "youtube",
        "video-trainability.get",
        json!({"id":public_video}),
        &[403],
    )?;
    scoped_or_rejected(
        run,
        "youtube",
        "videos.batch-get-stats",
        json!({"id":[public_video],"part":"statistics"}),
        &[403],
    )?;
    let Some(channel) = channels.pointer("/items/0/id").and_then(Value::as_str) else {
        blocked(
            run,
            "youtube",
            &[
                "captions.delete",
                "captions.download",
                "captions.insert",
                "captions.list",
                "captions.update",
                "channel-banners.insert",
                "channel-sections.delete",
                "channel-sections.insert",
                "channel-sections.list",
                "channel-sections.update",
                "channels.update",
                "comment-threads.insert",
                "comments.delete",
                "comments.insert",
                "comments.list",
                "comments.set-moderation-status",
                "comments.update",
                "playlist-images.delete",
                "playlist-images.insert",
                "playlist-images.list",
                "playlist-images.update",
                "playlist-items.delete",
                "playlist-items.insert",
                "playlist-items.list",
                "playlist-items.update",
                "playlists.delete",
                "playlists.insert",
                "playlists.list",
                "playlists.update",
                "subscriptions.delete",
                "subscriptions.insert",
                "subscriptions.list",
                "super-chat-events.list",
                "thumbnails.set",
                "videos.delete",
                "videos.get-rating",
                "videos.insert",
                "videos.rate",
                "videos.update",
                "watermarks.set",
                "watermarks.unset",
            ],
            "The successful channels.list mine=true response returned no owned channel. No channel ID was invented and no public channel was mutated.",
        )?;
        return Ok(());
    };
    let channel = channel.to_owned();
    let marker = run.marker.clone();
    let playlist = run.json("jyt", &["new-playlist", &marker, "privacy:private"])?;
    let playlist_id = id(&playlist, "id")?;
    run.register(
        "youtube",
        "playlists.delete",
        json!({"id":playlist_id}),
        &playlist_id,
    )?;
    run.json("jyt", &["playlists", "limit:1"])?;
    run.api(
        "youtube",
        "playlists.list",
        json!({"id":playlist_id,"part":"snippet,status"}),
    )?;
    run.api("youtube","playlists.update",json!({"part":"snippet,status","id":playlist_id,"snippet":{"title":format!("{marker}-updated")},"status":{"privacyStatus":"private"}}))?;
    let video_file = run.directory.path().join("fixture.mp4");
    fs::write(&video_file, include_bytes!("../data/live-fixture.mp4"))?;
    let video = run.json(
        "jyt",
        &[
            "upload",
            video_file.to_str().unwrap(),
            &format!("title:{marker}"),
            "category:27",
            "privacy:private",
        ],
    )?;
    let video_id = id(&video, "id")?;
    run.register(
        "youtube",
        "videos.delete",
        json!({"id":video_id}),
        &video_id,
    )?;
    let status = run.api(
        "youtube",
        "videos.list",
        json!({"id":video_id,"part":"status,processingDetails"}),
    )?;
    let state = status
        .pointer("/items/0/status")
        .context("uploaded video is unavailable; no processing retries were attempted")?;
    run.events.push(json!({"kind":"fixture-status","service":"youtube","uploadStatus":state["uploadStatus"],"failureReason":state["failureReason"],"rejectionReason":state["rejectionReason"],"processingStatus":status.pointer("/items/0/processingDetails/processingStatus")}));
    run.save()?;
    ensure!(
        state["privacyStatus"] == "private",
        "test video is not private"
    );
    ensure!(
        state["uploadStatus"] != "failed" && state["uploadStatus"] != "rejected",
        "Google rejected the synthetic video; inspect its recorded processing status"
    );
    run.json("jyt", &["like", &video_id])?;
    run.api("youtube", "videos.get-rating", json!({"id":[video_id]}))?;
    run.json("jyt", &["unlike", &video_id])?;
    run.api(
        "youtube",
        "videos.rate",
        json!({"id":video_id,"rating":"none"}),
    )?;
    run.api("youtube","videos.update",json!({"part":"snippet,status","id":video_id,"snippet":{"title":marker,"categoryId":"27","description":"Temporary synthetic test video"},"status":{"privacyStatus":"private"}}))?;
    run.json("jyt", &["add", &playlist_id, &format!("video:{video_id}")])?;
    run.json("jyt", &["playlist", &playlist_id, "limit:1"])?;
    let items = run.api(
        "youtube",
        "playlist-items.list",
        json!({"playlistId":playlist_id,"part":"snippet","max":1}),
    )?;
    let item = items.pointer("/items/0").context("playlist item missing")?;
    let item_id = id(item, "id")?;
    run.api("youtube","playlist-items.update",json!({"part":"snippet","id":item_id,"snippet":{"playlistId":playlist_id,"position":0,"resourceId":{"kind":"youtube#video","videoId":video_id}}}))?;
    run.api("youtube", "playlist-items.delete", json!({"id":item_id}))?;
    let second=run.api("youtube","playlist-items.insert",json!({"part":"snippet","snippet":{"playlistId":playlist_id,"resourceId":{"kind":"youtube#video","videoId":public_video}}}))?;
    ensure!(
        second["id"].is_string(),
        "playlist item creation did not return ID"
    );
    let caption_file = run.directory.path().join("fixture.srt");
    fs::write(
        &caption_file,
        "1\n00:00:00,000 --> 00:00:00,900\nSynthetic CLI test\n",
    )?;
    if let Some(caption) = attempt(
        run,
        "captions.insert",
        json!({"part":"snippet","snippet":{"videoId":video_id,"language":"en","name":"Fixture","isDraft":true},"upload":caption_file,"contentType":"application/octet-stream"}),
    ) && let Ok(caption_id) = id(&caption, "id")
    {
        run.register(
            "youtube",
            "captions.delete",
            json!({"id":caption_id}),
            &caption_id,
        )?;
        attempt(
            run,
            "captions.list",
            json!({"videoId":video_id,"part":"snippet"}),
        );
        attempt(
            run,
            "captions.update",
            json!({"part":"snippet","id":caption_id,"snippet":{"name":"Updated fixture","isDraft":true}}),
        );
        let out = run.directory.path().join("caption.srt");
        attempt(
            run,
            "captions.download",
            json!({"id":caption_id,"tfmt":"srt","output":out}),
        );
    }
    let image = run.directory.path().join("fixture.png");
    fs::write(&image, include_bytes!("../data/live-fixture.png"))?;
    let mut image_to_update = None;
    if let Some(image_result) = attempt(
        run,
        "playlist-images.insert",
        json!({"part":"snippet","snippet":{"playlistId":playlist_id,"type":"hero"},"upload":image,"contentType":"image/png"}),
    ) && let Ok(image_id) = id(&image_result, "id")
    {
        run.register(
            "youtube",
            "playlist-images.delete",
            json!({"id":image_id}),
            &image_id,
        )?;
        attempt(
            run,
            "playlist-images.list",
            json!({"parent":playlist_id,"part":"snippet","max":1}),
        );
        image_to_update = Some(image_id);
    }
    let thumbnail = run.directory.path().join("thumbnail.png");
    fs::write(&thumbnail, include_bytes!("../data/live-thumbnail.png"))?;
    attempt(
        run,
        "thumbnails.set",
        json!({"videoId":video_id,"upload":thumbnail,"contentType":"image/png"}),
    );
    youtube_sections(run)?;
    attempt(
        run,
        "subscriptions.list",
        json!({"part":"snippet","mine":true,"max":1}),
    );
    scoped_or_rejected(
        run,
        "youtube",
        "subscriptions.insert",
        json!({"part":"snippet","snippet":{"resourceId":{"kind":"youtube#channel","channelId":channel}}}),
        &[400, 403],
    )?;
    youtube_capabilities(run)?;
    comment_operations(run, &channel, &video_id)?;
    if std::env::var("JOOG_LIVE_PHOTOS_CLEANUP").as_deref() == Ok("manual") {
        youtube_banner(run)?;
    } else {
        blocked(
            run,
            "youtube",
            &["channel-banners.insert"],
            "Banner uploads require manual removal of the test image from the Channel Art album. Set JOOG_LIVE_PHOTOS_CLEANUP=manual to include this check.",
        )?;
    }
    if std::env::var("JOOG_LIVE_YOUTUBE_WATERMARK").as_deref() == Ok("disposable") {
        youtube_watermark(run)?;
    } else {
        blocked(
            run,
            "youtube",
            &["watermarks.set", "watermarks.unset"],
            "Watermark checks require a disposable watermark. Set JOOG_LIVE_YOUTUBE_WATERMARK=disposable to include them.",
        )?;
    }
    channel_round_trip(run)?;
    if let Some(image_id) = image_to_update {
        // Keep the provider's known rejection separate from successful coverage.
        reason_or_rejected(
            run,
            "youtube",
            "playlist-images.update",
            json!({"id":image_id,"snippet":{"playlistId":playlist_id,"type":"hero","width":256,"height":256},"upload":image,"contentType":"image/png"}),
            &["unexpectedPart"],
        )?;
    }

    Ok(())
}

fn channel_round_trip(run: &mut Live) -> Result<()> {
    let query = json!({"mine":true,"part":"brandingSettings","fields":"items(id,brandingSettings)","max":1});
    let before = run.api("youtube", "channels.list", query.clone())?;
    let channel = before
        .pointer("/items/0")
        .context("owned channel missing")?;
    let channel_id = id(channel, "id")?;
    let branding = channel
        .get("brandingSettings")
        .context("branding settings missing")?
        .clone();
    let check=run.api("youtube","channels.list",json!({"id":[channel_id],"part":"brandingSettings","fields":"items(id,brandingSettings)","max":1}))?;
    ensure!(
        check.pointer("/items/0/brandingSettings") == Some(&branding),
        "owner and ID lookups returned different settings; no write was attempted"
    );
    let request = json!({"part":"brandingSettings","id":channel_id,"brandingSettings":branding});
    let verification = json!({"command":"channels.list","parameters":query,"pointer":"/items/0/brandingSettings","expected":branding});
    run.register("youtube", "channels.update", request.clone(), &channel_id)?;
    run.pending.last_mut().unwrap().verification = Some(verification);
    run.save()?;
    let path = run.report.with_extension("channel-snapshot.json");
    let mut snapshot =
        tempfile::NamedTempFile::new_in(path.parent().context("snapshot parent missing")?)?;
    serde_json::to_writer_pretty(&mut snapshot, &json!({"before":before,"request":request}))?;
    snapshot.as_file().sync_all()?;
    snapshot.persist(&path)?;
    run.events.push(json!({"kind":"state-backup","service":"youtube","command":"channels.update","path":path,"status":"saved"}));
    run.save()?;
    run.api("youtube", "channels.update", request)?;
    let after = run.api("youtube", "channels.list", query)?;
    let after_channel = after
        .pointer("/items/0")
        .context("owned channel missing after update")?;
    ensure!(
        after_channel["id"] == channel_id && after_channel["brandingSettings"] == branding,
        "channel settings postcondition failed; snapshot and verified restoration remain recorded"
    );
    removed(run, &channel_id)?;
    run.events.push(json!({"kind":"postcondition","service":"youtube","command":"channels.update","status":"passed","preservedSnapshot":path}));
    run.save()
}

fn youtube_capabilities(run: &mut Live) -> Result<()> {
    reason_or_rejected(
        run,
        "youtube",
        "channel-sections.list",
        json!({"mine":true,"part":"snippet,contentDetails","max":1}),
        &["channelNotActive"],
    )?;
    // Preserve pre-existing subscriptions. Only a newly returned resource ID
    // can be registered and deleted by this scenario.
    let target = "UC_x5XG1OV2P6uZZ5FSM9Ttw";
    let existing = run.api(
        "youtube",
        "subscriptions.list",
        json!({"mine":true,"forChannelId":target,"part":"snippet","max":1}),
    )?;
    if existing["items"].as_array().is_none_or(Vec::is_empty) {
        if let Some(subscription) = reason_or_rejected(
            run,
            "youtube",
            "subscriptions.insert",
            json!({"part":"snippet","snippet":{"resourceId":{"kind":"youtube#channel","channelId":target}}}),
            &["subscriptionForbidden"],
        )? {
            let subscription_id = id(&subscription, "id")?;
            run.register(
                "youtube",
                "subscriptions.delete",
                json!({"id":subscription_id}),
                &subscription_id,
            )?;
            ensure!(
                subscription
                    .pointer("/snippet/resourceId/channelId")
                    .and_then(Value::as_str)
                    == Some(target),
                "subscription target changed"
            );
            run.api(
                "youtube",
                "subscriptions.delete",
                json!({"id":subscription_id}),
            )?;
            removed(run, &subscription_id)?;
        }
    } else {
        blocked(
            run,
            "youtube",
            &["subscriptions.insert", "subscriptions.delete"],
            "The target subscription already exists. It was not modified or deleted.",
        )?;
    }
    scoped_or_rejected(
        run,
        "youtube",
        "super-chat-events.list",
        json!({"part":"snippet","max":1,"maxPages":1}),
        &[403],
    )?;
    Ok(())
}

fn youtube_watermark(run: &mut Live) -> Result<()> {
    ensure!(
        std::env::var("JOOG_LIVE_YOUTUBE_WATERMARK").as_deref() == Ok("disposable"),
        "watermark tests require JOOG_LIVE_YOUTUBE_WATERMARK=disposable and a channel whose watermark may be replaced and removed"
    );
    let channels = run.api(
        "youtube",
        "channels.list",
        json!({"mine":true,"part":"id","fields":"items(id)","max":1}),
    )?;
    let channel = channels
        .pointer("/items/0/id")
        .and_then(Value::as_str)
        .context("no owned channel")?
        .to_owned();
    let path = run.directory.path().join("watermark.png");
    fs::write(&path, include_bytes!("../data/live-fixture.png"))?;
    run.register(
        "youtube",
        "watermarks.unset",
        json!({"channelId":channel}),
        &channel,
    )?;
    run.api("youtube","watermarks.set",json!({"channelId":channel,"timing":{"type":"offsetFromStart","offsetMs":"0","durationMs":"1000"},"upload":path,"contentType":"image/png"}))?;
    run.api("youtube", "watermarks.unset", json!({"channelId":channel}))?;
    removed(run, &channel)
}

fn youtube_banner(run: &mut Live) -> Result<()> {
    ensure!(
        std::env::var("JOOG_LIVE_PHOTOS_CLEANUP").as_deref() == Ok("manual"),
        "banner uploads require JOOG_LIVE_PHOTOS_CLEANUP=manual because YouTube retains the image in the Channel Art album"
    );
    let query = json!({"mine":true,"part":"brandingSettings","fields":"items(id,brandingSettings)","max":1});
    let before = run.api("youtube", "channels.list", query.clone())?;
    let original = before
        .pointer("/items/0/brandingSettings/image/bannerExternalUrl")
        .and_then(Value::as_str)
        .context("banner test needs an existing banner to restore")?;
    let bytes = banner_image(original)?;
    let backup = run.report.with_extension("banner-original.png");
    let mut file =
        tempfile::NamedTempFile::new_in(backup.parent().context("backup parent missing")?)?;
    file.write_all(&bytes)?;
    file.as_file().sync_all()?;
    file.persist(&backup)?;
    let restore = json!({"upload":backup,"contentType":if bytes.starts_with(b"\x89PNG") {"image/png"}else{"image/jpeg"}});
    let backup_id = backup.to_str().context("backup path is not UTF-8")?;
    run.register(
        "youtube",
        "channel-banners.insert",
        restore.clone(),
        backup_id,
    )?;
    let verification = json!({"kind":"banner-content","command":"channels.list","parameters":query,"channelId":before["items"][0]["id"],"channel":before["items"][0]["brandingSettings"]["channel"],"imageSha256":format!("{:x}",sha2::Sha256::digest(&bytes))});
    run.pending
        .last_mut()
        .context("banner restoration ledger missing")?
        .verification = Some(verification.clone());
    run.save()?;
    let path = run
        .directory
        .path()
        .join(format!("{}-banner.png", run.marker));
    fs::write(&path, include_bytes!("../data/live-banner.png"))?;
    let banner = run.api(
        "youtube",
        "channel-banners.insert",
        json!({"upload":path,"contentType":"image/png"}),
    )?;
    let image = id(&banner, "url")?;
    run.manual_cleanup.push(json!({"service":"youtube","kind":"channel-banner","id":image,"url":"https://photos.google.com/search/Channel%20Art","imageUrl":image,"marker":run.marker,"status":"pending-user-cleanup"}));
    run.save()?;
    ensure!(
        url::Url::parse(&image)?.scheme() == "https",
        "banner did not return an HTTPS image URL"
    );
    run.events.push(json!({"kind":"postcondition","service":"youtube","command":"channel-banners.insert","status":"passed","imageUrl":image}));
    run.save()?;
    let restored = run.api("youtube", "channel-banners.insert", restore)?;
    run.verify_cleanup("youtube", &verification)?;
    removed(run, backup_id)?;
    run.events.push(json!({"kind":"restoration","service":"youtube","command":"channel-banners.insert","status":"passed","restoredImageUrl":restored["url"],"imageSha256":verification["imageSha256"],"snapshot":backup}));
    run.save()?;
    Ok(())
}

fn youtube_sections(run: &mut Live) -> Result<()> {
    let query = json!({"mine":true,"part":"snippet,contentDetails","max":50});
    let before = run.api("youtube", "channel-sections.list", query.clone())?;
    let items = before["items"].as_array().cloned().unwrap_or_default();
    let position = items
        .iter()
        .filter_map(|item| item.pointer("/snippet/position").and_then(Value::as_u64))
        .max()
        .map_or(0, |position| position + 1);
    let section=run.api("youtube", "channel-sections.insert", json!({"part":"snippet,contentDetails","snippet":{"type":"multipleChannels","title":run.marker,"position":position},"contentDetails":{"channels":["UC_x5XG1OV2P6uZZ5FSM9Ttw"]}}))?;
    let target = id(&section, "id")?;
    run.register(
        "youtube",
        "channel-sections.delete",
        json!({"id":target}),
        &target,
    )?;
    let title = format!("{}-updated", run.marker);
    run.api("youtube","channel-sections.update",json!({"part":"snippet,contentDetails","id":target,"snippet":{"type":"multipleChannels","title":title,"position":position},"contentDetails":{"channels":["UC_x5XG1OV2P6uZZ5FSM9Ttw"]}}))?;
    let updated = run.api(
        "youtube",
        "channel-sections.list",
        json!({"id":target,"part":"snippet,contentDetails"}),
    )?;
    ensure!(
        updated.pointer("/items/0/snippet/title") == Some(&json!(title)),
        "section title was not updated: expected {title:?}; received {updated}"
    );
    run.api("youtube", "channel-sections.delete", json!({"id":target}))?;
    removed(run, &target)?;
    let after = run.api("youtube", "channel-sections.list", query)?;
    ensure!(
        after["items"].as_array().cloned().unwrap_or_default() == items,
        "existing channel sections changed"
    );
    Ok(())
}

fn youtube_image_update(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let playlist = run.json("jyt", &["new-playlist", &marker, "privacy:private"])?;
    let playlist_id = id(&playlist, "id")?;
    run.register(
        "youtube",
        "playlists.delete",
        json!({"id":playlist_id}),
        &playlist_id,
    )?;
    let path = run.directory.path().join("playlist.png");
    fs::write(&path, include_bytes!("../data/live-fixture.png"))?;
    let image=run.api("youtube","playlist-images.insert",json!({"part":"snippet","snippet":{"playlistId":playlist_id,"type":"hero"},"upload":path,"contentType":"image/png"}))?;
    let image_id = id(&image, "id")?;
    run.register(
        "youtube",
        "playlist-images.delete",
        json!({"id":image_id}),
        &image_id,
    )?;
    let result=run.api("youtube","playlist-images.update",json!({"id":image_id,"snippet":{"playlistId":playlist_id,"type":"hero","width":256,"height":256},"upload":path,"contentType":"image/png"}))?;
    ensure!(
        result["id"] == image_id,
        "image update returned another resource"
    );
    contains(
        &run.api(
            "youtube",
            "playlist-images.list",
            json!({"parent":playlist_id,"part":"snippet","max":1}),
        )?,
        &image_id,
    )?;
    Ok(())
}

fn comment_operations(run: &mut Live, channel: &str, video: &str) -> Result<()> {
    // Comments belong only to this run's own synthetic video. Unlisted videos
    // are not added to public search or the channel's public video feed.
    run.api(
        "youtube",
        "videos.update",
        json!({"part":"status","id":video,"status":{"privacyStatus":"unlisted"}}),
    )?;
    if let Some(thread) = scoped_or_rejected(
        run,
        "youtube",
        "comment-threads.insert",
        json!({"part":"snippet","snippet":{"channelId":channel,"videoId":video,"topLevelComment":{"snippet":{"textOriginal":"Temporary synthetic CLI comment"}}}}),
        &[403],
    )? {
        let top = thread
            .pointer("/snippet/topLevelComment/id")
            .and_then(Value::as_str)
            .context("created comment ID missing")?;
        run.register("youtube", "comments.delete", json!({"id":top}), top)?;
        run.api(
            "youtube",
            "comments.list",
            json!({"part":"snippet","id":[top],"max":1}),
        )?;
        run.api("youtube","comments.update",json!({"part":"snippet","id":top,"snippet":{"textOriginal":"Updated synthetic CLI comment"}}))?;
        let reply = run.api("youtube","comments.insert",json!({"part":"snippet","snippet":{"parentId":top,"textOriginal":"Temporary synthetic CLI reply"}}))?;
        let reply_id = id(&reply, "id")?;
        run.register(
            "youtube",
            "comments.delete",
            json!({"id":reply_id}),
            &reply_id,
        )?;
        scoped_or_rejected(
            run,
            "youtube",
            "comments.set-moderation-status",
            json!({"id":[reply_id],"moderationStatus":"heldForReview","banAuthor":false}),
            &[403],
        )?;
        scoped_or_rejected(
            run,
            "youtube",
            "comments.set-moderation-status",
            json!({"id":[reply_id],"moderationStatus":"published","banAuthor":false}),
            &[403],
        )?;
        run.api("youtube", "comments.delete", json!({"id":reply_id}))?;
        removed(run, &reply_id)?;
        run.api("youtube", "comments.delete", json!({"id":top}))?;
        removed(run, top)?;
    }
    run.api(
        "youtube",
        "videos.update",
        json!({"part":"status","id":video,"status":{"privacyStatus":"private"}}),
    )?;
    Ok(())
}

fn youtube_remaining(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let channels = run.api(
        "youtube",
        "channels.list",
        json!({"mine":true,"part":"snippet","max":1}),
    )?;
    let channel = channels
        .pointer("/items/0/id")
        .and_then(Value::as_str)
        .context("no owned channel returned")?;
    let playlist = run.json("jyt", &["new-playlist", &marker, "privacy:private"])?;
    let playlist_id = id(&playlist, "id")?;
    run.register(
        "youtube",
        "playlists.delete",
        json!({"id":playlist_id}),
        &playlist_id,
    )?;
    youtube_sections(run)?;
    let path = run.directory.path().join("comments.mp4");
    fs::write(&path, include_bytes!("../data/live-fixture.mp4"))?;
    let video = run.json(
        "jyt",
        &[
            "upload",
            path.to_str().unwrap(),
            &format!("title:{marker}"),
            "category:27",
            "privacy:private",
        ],
    )?;
    let video_id = id(&video, "id")?;
    run.register(
        "youtube",
        "videos.delete",
        json!({"id":video_id}),
        &video_id,
    )?;
    let thumbnail = run.directory.path().join("thumbnail.png");
    fs::write(&thumbnail, include_bytes!("../data/live-thumbnail.png"))?;
    scoped_or_rejected(
        run,
        "youtube",
        "thumbnails.set",
        json!({"videoId":video_id,"upload":thumbnail,"contentType":"image/png"}),
        &[403],
    )?;
    comment_operations(run, channel, &video_id)?;
    run.api(
        "youtube",
        "subscriptions.list",
        json!({"part":"snippet","mine":true,"max":1}),
    )?;
    scoped_or_rejected(
        run,
        "youtube",
        "subscriptions.insert",
        json!({"part":"snippet","snippet":{"resourceId":{"kind":"youtube#channel","channelId":channel}}}),
        &[400, 403],
    )?;
    scoped_or_rejected(
        run,
        "youtube",
        "super-chat-events.list",
        json!({"part":"snippet","max":1}),
        &[403],
    )?;
    Ok(())
}

pub(super) fn photos(run: &mut Live) -> Result<()> {
    if std::env::var("JOOG_LIVE_PHOTOS_CLEANUP").as_deref() == Ok("manual") {
        return photos_round_trip(run);
    }
    let media = run.api("photos", "media-items.list", json!({"max":1}))?;
    run.json("jphotos", &["ls", "limit:1"])?;
    run.json("jphotos", &["find", "type:photo", "limit:1"])?;
    run.api(
        "photos",
        "media-items.search",
        json!({"filters":{"mediaTypeFilter":{"mediaTypes":["PHOTO"]}},"max":1}),
    )?;
    let albums = run.api("photos", "albums.list", json!({"max":1}))?;
    run.json("jphotos", &["albums", "limit:1"])?;
    if let Some(item) = media.pointer("/mediaItems/0/id").and_then(Value::as_str) {
        run.api("photos", "media-items.get", json!({"mediaItemId":item}))?;
        run.api(
            "photos",
            "media-items.batch-get",
            json!({"mediaItemIds":[item]}),
        )?;
        run.json("jphotos", &["show", item])?;
        let out = run.directory.path().join("photo.bin");
        run.json("jphotos", &["get", item, &format!("out:{}", out.display())])?;
        ensure!(fs::metadata(out)?.len() > 0, "photo download is empty");
    } else {
        blocked(
            run,
            "photos",
            &[
                "media-items.get",
                "media-items.batch-get",
                "jphotos.show",
                "jphotos.get",
            ],
            "The live media-items.list response contained no app-created media items.",
        )?;
    }
    if let Some(album) = albums.pointer("/albums/0/id").and_then(Value::as_str) {
        run.api("photos", "albums.get", json!({"albumId":album}))?;
        run.json("jphotos", &["ls", &format!("album:{album}"), "limit:1"])?;
    }
    blocked(
        run,
        "photos",
        &[
            "albums.add-enrichment",
            "albums.batch-add-media-items",
            "albums.batch-remove-media-items",
            "albums.create",
            "albums.patch",
            "media-items.batch-create",
            "media-items.patch",
            "jphotos.put",
            "jphotos.new-album",
            "jphotos.add",
            "jphotos.remove",
        ],
        "The Library API has no delete-media or delete-album method. Creating these fixtures would leave resources that the CLI cannot clean up. Existing photos/albums were not mutated.",
    )?;
    Ok(())
}

fn photos_round_trip(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let album = run.json("jphotos", &["new-album", &marker])?;
    let album_id = id(&album, "id")?;
    run.manual_cleanup.push(json!({"service":"photos","kind":"album","id":album_id,"url":album["productUrl"],"marker":marker,"status":"pending-user-cleanup"}));
    run.save()?;
    let path = run.directory.path().join(format!("{marker}.png"));
    fs::write(&path, include_bytes!("../data/live-fixture.png"))?;
    let media = run.json(
        "jphotos",
        &["put", path.to_str().unwrap(), &format!("caption:{marker}")],
    )?;
    let media_id = id(&media, "id")?;
    run.manual_cleanup.push(json!({"service":"photos","kind":"media","id":media_id,"url":media["productUrl"],"marker":marker,"status":"pending-user-cleanup"}));
    run.save()?;
    run.json("jphotos", &["add", &album_id, &format!("photo:{media_id}")])?;
    contains(
        &run.api(
            "photos",
            "media-items.search",
            json!({"albumId":album_id,"max":2}),
        )?,
        &media_id,
    )?;
    run.json(
        "jphotos",
        &["remove", &album_id, &format!("photo:{media_id}")],
    )?;
    let empty = run.api(
        "photos",
        "media-items.search",
        json!({"albumId":album_id,"max":2}),
    )?;
    ensure!(
        !empty["mediaItems"]
            .as_array()
            .is_some_and(|items| items.iter().any(|item| item["id"] == media_id)),
        "album removal did not remove the fixture"
    );
    run.api(
        "photos",
        "albums.batch-add-media-items",
        json!({"albumId":album_id,"mediaItemIds":[media_id]}),
    )?;
    run.api(
        "photos",
        "albums.batch-remove-media-items",
        json!({"albumId":album_id,"mediaItemIds":[media_id]}),
    )?;
    run.json("jphotos", &["add", &album_id, &format!("photo:{media_id}")])?;
    run.api("photos","media-items.patch",json!({"id":media_id,"updateMask":"description","description":format!("{marker} updated caption")}))?;
    contains(
        &run.api("photos", "media-items.get", json!({"mediaItemId":media_id}))?,
        "updated caption",
    )?;
    contains(
        &run.api(
            "photos",
            "media-items.batch-get",
            json!({"mediaItemIds":[media_id]}),
        )?,
        &media_id,
    )?;
    run.api("photos","albums.patch",json!({"id":album_id,"updateMask":"title,coverPhotoMediaItemId","title":format!("{marker}-verified"),"coverPhotoMediaItemId":media_id}))?;
    contains(
        &run.api("photos", "albums.get", json!({"albumId":album_id}))?,
        "verified",
    )?;
    run.api("photos","albums.add-enrichment",json!({"albumId":album_id,"newEnrichmentItem":{"textEnrichment":{"text":"Synthetic CLI verification"}},"albumPosition":{"position":"LAST_IN_ALBUM"}}))?;
    run.json("jphotos", &["ls", "limit:1"])?;
    run.json("jphotos", &["ls", &format!("album:{album_id}"), "limit:1"])?;
    run.json("jphotos", &["find", "type:photo", "limit:1"])?;
    run.json("jphotos", &["albums", "limit:1"])?;
    run.api("photos", "albums.list", json!({"max":1}))?;
    run.api("photos", "media-items.list", json!({"max":1}))?;
    run.json("jphotos", &["show", &media_id])?;
    let out = run.directory.path().join("photo.png");
    run.json(
        "jphotos",
        &["get", &media_id, &format!("out:{}", out.display())],
    )?;
    ensure!(
        fs::read(out)?.starts_with(b"\x89PNG\r\n\x1a\n"),
        "download did not contain the uploaded PNG"
    );
    Ok(())
}
