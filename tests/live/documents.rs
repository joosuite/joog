use super::*;

const IMAGE: &str = "https://www.gstatic.com/images/branding/product/1x/docs_2020q4_48dp.png";

fn find<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    if let Some(found) = value.get(key) {
        return Some(found);
    }
    match value {
        Value::Object(fields) => fields.values().find_map(|value| find(value, key)),
        Value::Array(items) => items.iter().find_map(|value| find(value, key)),
        _ => None,
    }
}
fn span(value: &Value, key: &str) -> Option<(i64, i64)> {
    if value.get(key).is_some()
        && let (Some(start), Some(end)) = (value["startIndex"].as_i64(), value["endIndex"].as_i64())
    {
        return Some((start, end));
    }
    match value {
        Value::Object(fields) => fields.values().find_map(|value| span(value, key)),
        Value::Array(items) => items.iter().find_map(|value| span(value, key)),
        _ => None,
    }
}
fn attempt(
    run: &mut Live,
    service: &str,
    target: &str,
    command: &str,
    body: Value,
) -> Option<Value> {
    if run.stop || run.scenario_failed {
        return None;
    }
    let key = if service == "docs" {
        "documentId"
    } else {
        "presentationId"
    };
    run.api(service, command, json!({key:target,"body":body}))
        .ok()
}
pub(super) fn docs(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let created = run.json("jdocs", &["new", &marker])?;
    let target = id(&created, "documentId")?;
    own_file(run, &target)?;
    run.json(
        "jdocs",
        &[
            "share",
            &target,
            &format!("to:{}", run.account.clone()),
            "role:reader",
            "notify:false",
        ],
    )?;
    run.json(
        "jdocs",
        &[
            "append",
            &target,
            "text:Heading\\nHello Unicode café\\nTable follows\\n",
        ],
    )?;
    contains(&run.json("jdocs", &["read", &target])?, "Unicode café")?;
    run.json("jdocs", &["ls", "limit:1"])?;
    run.json("jdocs", &["replace", &target, "find:Hello", "with:Welcome"])?;
    for format in ["txt", "pdf", "docx", "html", "odt"] {
        let out = run.directory.path().join(format!("document.{format}"));
        run.json(
            "jdocs",
            &[
                "export",
                &target,
                &format!("format:{format}"),
                &format!("out:{}", out.display()),
            ],
        )?;
        ensure!(fs::metadata(out)?.len() > 0, "document export is empty");
    }
    let range = json!({"startIndex":1,"endIndex":7});
    for (command, body) in [
        (
            "update-text-style",
            json!({"range":range,"textStyle":{"bold":true},"fields":"bold"}),
        ),
        (
            "update-paragraph-style",
            json!({"range":range,"paragraphStyle":{"namedStyleType":"HEADING_1"},"fields":"namedStyleType"}),
        ),
        (
            "create-paragraph-bullets",
            json!({"range":range,"bulletPreset":"BULLET_DISC_CIRCLE_SQUARE"}),
        ),
        ("delete-paragraph-bullets", json!({"range":range})),
        ("create-named-range", json!({"name":marker,"range":range})),
        (
            "replace-named-range-content",
            json!({"namedRangeName":marker,"text":"Title!"}),
        ),
        ("delete-named-range", json!({"name":marker})),
        (
            "replace-all-text",
            json!({"containsText":{"text":"Welcome","matchCase":true},"replaceText":"Hello"}),
        ),
        (
            "update-document-style",
            json!({"documentStyle":{"marginTop":{"magnitude":36,"unit":"PT"}},"fields":"marginTop"}),
        ),
        (
            "update-section-style",
            json!({"range":range,"sectionStyle":{"marginLeft":{"magnitude":36,"unit":"PT"}},"fields":"marginLeft"}),
        ),
        (
            "update-named-style",
            json!({"namedStyle":{"namedStyleType":"NORMAL_TEXT","textStyle":{"bold":false}},"fields":"namedStyleType,textStyle,textStyle.bold"}),
        ),
        (
            "insert-date",
            json!({"location":{"index":1},"dateElementProperties":{"timestamp":"2026-11-25T12:00:00Z","dateFormat":"DATE_FORMAT_ISO8601"}}),
        ),
        (
            "insert-person",
            json!({"location":{"index":1},"personProperties":{"email":run.account.clone()}}),
        ),
        (
            "insert-rich-link",
            json!({"location":{"index":1},"richLinkProperties":{"uri":format!("https://docs.google.com/document/d/{target}/edit")}}),
        ),
        ("insert-page-break", json!({"location":{"index":1}})),
        (
            "insert-section-break",
            json!({"endOfSegmentLocation":{},"sectionType":"CONTINUOUS"}),
        ),
        ("create-footnote", json!({"location":{"index":1}})),
    ] {
        attempt(run, "docs", &target, command, body);
    }
    for (create, delete, key) in [
        ("create-header", "delete-header", "headerId"),
        ("create-footer", "delete-footer", "footerId"),
    ] {
        if let Some(value) = attempt(run, "docs", &target, create, json!({"type":"DEFAULT"}))
            && let Some(id) = find(&value, key).and_then(Value::as_str)
        {
            attempt(run, "docs", &target, delete, json!({key:id}));
        }
    }
    if let Some(value) = attempt(
        run,
        "docs",
        &target,
        "add-document-tab",
        json!({"tabProperties":{"title":"Fixture tab"}}),
    ) && let Some(tab_id) = find(&value, "tabId").and_then(Value::as_str)
    {
        attempt(
            run,
            "docs",
            &target,
            "update-document-tab-properties",
            json!({"tabProperties":{"tabId":tab_id,"title":"Renamed fixture"},"fields":"title"}),
        );
        attempt(run, "docs", &target, "delete-tab", json!({"tabId":tab_id}));
    }
    if let Some(value) = attempt(
        run,
        "docs",
        &target,
        "insert-inline-image",
        json!({"location":{"index":1},"uri":IMAGE}),
    ) && let Some(image_id) = find(&value, "objectId").and_then(Value::as_str)
    {
        attempt(
            run,
            "docs",
            &target,
            "replace-image",
            json!({"imageObjectId":image_id,"uri":IMAGE,"imageReplaceMethod":"CENTER_CROP"}),
        );
    }
    let dropdown = json!({"dropdownDefinitionId":"kix.fixture01","dropdownDefinitionProperties":{"title":"Fixture status","options":[{"optionId":"dropdownItem.one","displayValue":"One"},{"optionId":"dropdownItem.two","displayValue":"Two"}]}});
    if attempt(
        run,
        "docs",
        &target,
        "create-dropdown-definition",
        json!({"dropdownDefinition":dropdown}),
    )
    .is_some()
    {
        let inserted = attempt(
            run,
            "docs",
            &target,
            "insert-dropdown",
            json!({"dropdownDefinitionId":"kix.fixture01","selectedOptionId":"dropdownItem.one","location":{"index":1}}),
        );
        if let Some(value) = inserted
            && let Some(dropdown_id) = find(&value, "dropdownId").and_then(Value::as_str)
        {
            attempt(
                run,
                "docs",
                &target,
                "update-dropdown-properties",
                json!({"dropdownId":dropdown_id,"dropdownProperties":{"selectedOptionId":"dropdownItem.two"},"fields":"selectedOptionId"}),
            );
        }
        attempt(
            run,
            "docs",
            &target,
            "update-dropdown-definition-properties",
            json!({"dropdownDefinitionId":"kix.fixture01","dropdownDefinitionProperties":{"title":"Updated fixture status"},"fields":"title"}),
        );
        let current = run.api("docs", "get", json!({"documentId":target}))?;
        if let Some((start, end)) = span(&current, "dropdown") {
            attempt(
                run,
                "docs",
                &target,
                "delete-content-range",
                json!({"range":{"startIndex":start,"endIndex":end}}),
            );
        }
        attempt(
            run,
            "docs",
            &target,
            "delete-dropdown-definition",
            json!({"dropdownDefinitionId":"kix.fixture01"}),
        );
    }
    if attempt(
        run,
        "docs",
        &target,
        "insert-table",
        json!({"rows":3,"columns":3,"endOfSegmentLocation":{}}),
    )
    .is_some()
    {
        let document = run.api("docs", "get", json!({"documentId":target}))?;
        let start = document
            .pointer("/body/content")
            .and_then(Value::as_array)
            .and_then(|items| items.iter().find(|item| item.get("table").is_some()))
            .and_then(|item| item["startIndex"].as_i64())
            .context("created table index missing")?;
        let location = json!({"tableStartLocation":{"index":start},"rowIndex":0,"columnIndex":0});
        let table_range = json!({"tableCellLocation":location,"rowSpan":2,"columnSpan":2});
        for (command, body) in [
            (
                "insert-table-column",
                json!({"tableCellLocation":location,"insertRight":true}),
            ),
            (
                "insert-table-row",
                json!({"tableCellLocation":location,"insertBelow":true}),
            ),
            (
                "update-table-cell-style",
                json!({"tableRange":table_range,"tableCellStyle":{"paddingTop":{"magnitude":3,"unit":"PT"}},"fields":"paddingTop"}),
            ),
            (
                "update-table-column-properties",
                json!({"tableStartLocation":{"index":start},"columnIndices":[0],"tableColumnProperties":{"widthType":"FIXED_WIDTH","width":{"magnitude":72,"unit":"PT"}},"fields":"widthType,width"}),
            ),
            (
                "update-table-row-style",
                json!({"tableStartLocation":{"index":start},"rowIndices":[0],"tableRowStyle":{"minRowHeight":{"magnitude":18,"unit":"PT"}},"fields":"minRowHeight"}),
            ),
            ("merge-table-cells", json!({"tableRange":table_range})),
            ("unmerge-table-cells", json!({"tableRange":table_range})),
            (
                "pin-table-header-rows",
                json!({"tableStartLocation":{"index":start},"pinnedHeaderRowsCount":1}),
            ),
            (
                "delete-table-column",
                json!({"tableCellLocation":{"tableStartLocation":{"index":start},"rowIndex":0,"columnIndex":2}}),
            ),
            (
                "delete-table-row",
                json!({"tableCellLocation":{"tableStartLocation":{"index":start},"rowIndex":2,"columnIndex":0}}),
            ),
        ] {
            attempt(run, "docs", &target, command, body);
        }
    }
    if let Some(comment) = attempt(
        run,
        "docs",
        &target,
        "insert-comment",
        json!({"content":"Fixture comment","range":{"startIndex":1,"endIndex":2}}),
    ) && let Some(comment_id) = find(&comment, "commentId").and_then(Value::as_str)
    {
        if let Some(reply) = attempt(
            run,
            "docs",
            &target,
            "add-comment-reply",
            json!({"commentId":comment_id,"post":{"content":"Fixture reply"}}),
        ) && let Some(post_id) = find(&reply, "postId").and_then(Value::as_str)
        {
            attempt(
                run,
                "docs",
                &target,
                "update-comment-post",
                json!({"commentId":comment_id,"postId":post_id,"content":"Updated fixture reply"}),
            );
            attempt(
                run,
                "docs",
                &target,
                "delete-comment-reply",
                json!({"commentId":comment_id,"postId":post_id}),
            );
        }
        attempt(
            run,
            "docs",
            &target,
            "delete-comment",
            json!({"commentId":comment_id}),
        );
    }
    attempt(
        run,
        "docs",
        &target,
        "insert-text",
        json!({"location":{"index":1},"text":"XX"}),
    );
    attempt(
        run,
        "docs",
        &target,
        "delete-content-range",
        json!({"range":{"startIndex":1,"endIndex":3}}),
    );
    run.api("docs","batch-update",json!({"documentId":target,"requests":[{"insertText":{"endOfSegmentLocation":{},"text":"Final fixture text\n"}}]}))?;
    Ok(())
}

pub(super) fn slides(run: &mut Live) -> Result<()> {
    let marker = run.marker.clone();
    let created = run.json("jslides", &["new", &marker])?;
    let target = id(&created, "presentationId")?;
    own_file(run, &target)?;
    run.json(
        "jslides",
        &["add", &target, "title:Fixture title", "body:Hello café"],
    )?;
    run.json("jslides", &["slides", &target])?;
    run.json(
        "jslides",
        &[
            "set",
            &target,
            "slide:1",
            "title:Updated title",
            "body:Updated body",
        ],
    )?;
    run.json(
        "jslides",
        &["replace", &target, "find:Updated", "with:Fixture"],
    )?;
    run.json("jslides", &["ls", "limit:1"])?;
    for format in ["pdf", "pptx", "odp"] {
        let out = run.directory.path().join(format!("presentation.{format}"));
        run.json(
            "jslides",
            &[
                "export",
                &target,
                &format!("format:{format}"),
                &format!("out:{}", out.display()),
            ],
        )?;
        ensure!(fs::metadata(out)?.len() > 0, "presentation export is empty");
    }
    let deck = run.api("slides", "get", json!({"presentationId":target}))?;
    let page = deck
        .pointer("/slides/0/objectId")
        .and_then(Value::as_str)
        .context("slide ID missing")?;
    let element = json!({"pageObjectId":page,"size":{"width":{"magnitude":120,"unit":"PT"},"height":{"magnitude":80,"unit":"PT"}},"transform":{"scaleX":1,"scaleY":1,"translateX":10,"translateY":10,"unit":"PT"}});
    let text = json!({"type":"ALL"});
    for (command, body) in [
        (
            "create-slide",
            json!({"objectId":"slide_fixture","slideLayoutReference":{"predefinedLayout":"BLANK"}}),
        ),
        (
            "create-shape",
            json!({"objectId":"shape_fixture","shapeType":"RECTANGLE","elementProperties":element}),
        ),
        (
            "insert-text",
            json!({"objectId":"shape_fixture","insertionIndex":0,"text":"Hello fixture\nSecond line\n"}),
        ),
        (
            "update-text-style",
            json!({"objectId":"shape_fixture","textRange":text,"style":{"bold":true},"fields":"bold"}),
        ),
        (
            "update-paragraph-style",
            json!({"objectId":"shape_fixture","textRange":text,"style":{"alignment":"CENTER"},"fields":"alignment"}),
        ),
        (
            "create-paragraph-bullets",
            json!({"objectId":"shape_fixture","textRange":text,"bulletPreset":"BULLET_DISC_CIRCLE_SQUARE"}),
        ),
        (
            "delete-paragraph-bullets",
            json!({"objectId":"shape_fixture","textRange":text}),
        ),
        (
            "update-shape-properties",
            json!({"objectId":"shape_fixture","shapeProperties":{"shapeBackgroundFill":{"solidFill":{"color":{"rgbColor":{"red":0.8,"green":0.9,"blue":1.0}}}}},"fields":"shapeBackgroundFill.solidFill.color"}),
        ),
        (
            "update-page-element-alt-text",
            json!({"objectId":"shape_fixture","title":"Fixture shape","description":"A temporary test shape"}),
        ),
        (
            "update-page-element-transform",
            json!({"objectId":"shape_fixture","applyMode":"RELATIVE","transform":{"scaleX":1,"scaleY":1,"translateX":10,"translateY":10,"unit":"PT"}}),
        ),
        (
            "update-page-elements-z-order",
            json!({"pageElementObjectIds":["shape_fixture"],"operation":"BRING_TO_FRONT"}),
        ),
        (
            "duplicate-object",
            json!({"objectId":"shape_fixture","objectIds":{"shape_fixture":"shape_copy"}}),
        ),
        (
            "group-objects",
            json!({"childrenObjectIds":["shape_fixture","shape_copy"],"groupObjectId":"group_fixture"}),
        ),
        ("ungroup-objects", json!({"objectIds":["group_fixture"]})),
        (
            "update-page-properties",
            json!({"objectId":page,"pageProperties":{"pageBackgroundFill":{"solidFill":{"color":{"rgbColor":{"red":1,"green":1,"blue":1}}}}},"fields":"pageBackgroundFill"}),
        ),
        (
            "update-slides-position",
            json!({"slideObjectIds":["slide_fixture"],"insertionIndex":0}),
        ),
        (
            "create-line",
            json!({"objectId":"line_fixture","lineCategory":"STRAIGHT","elementProperties":element}),
        ),
        (
            "update-line-category",
            json!({"objectId":"line_fixture","lineCategory":"BENT"}),
        ),
        (
            "update-line-properties",
            json!({"objectId":"line_fixture","lineProperties":{"weight":{"magnitude":2,"unit":"PT"},"startConnection":{"connectedObjectId":"shape_fixture","connectionSiteIndex":0},"endConnection":{"connectedObjectId":"shape_copy","connectionSiteIndex":2}},"fields":"weight,startConnection,endConnection"}),
        ),
        ("reroute-line", json!({"objectId":"line_fixture"})),
        (
            "create-image",
            json!({"objectId":"image_fixture","url":IMAGE,"elementProperties":element}),
        ),
        (
            "update-image-properties",
            json!({"objectId":"image_fixture","imageProperties":{"link":{"url":"https://developers.google.com/slides"}},"fields":"link"}),
        ),
        (
            "replace-image",
            json!({"imageObjectId":"image_fixture","url":IMAGE,"imageReplaceMethod":"CENTER_CROP"}),
        ),
        (
            "create-table",
            json!({"objectId":"table_fixture","rows":3,"columns":3,"elementProperties":element}),
        ),
        (
            "insert-table-columns",
            json!({"tableObjectId":"table_fixture","cellLocation":{"rowIndex":0,"columnIndex":0},"insertRight":true,"number":1}),
        ),
        (
            "insert-table-rows",
            json!({"tableObjectId":"table_fixture","cellLocation":{"rowIndex":0,"columnIndex":0},"insertBelow":true,"number":1}),
        ),
        (
            "update-table-cell-properties",
            json!({"objectId":"table_fixture","tableRange":{"location":{"rowIndex":0,"columnIndex":0},"rowSpan":1,"columnSpan":1},"tableCellProperties":{"contentAlignment":"MIDDLE"},"fields":"contentAlignment"}),
        ),
        (
            "update-table-border-properties",
            json!({"objectId":"table_fixture","tableRange":{"location":{"rowIndex":0,"columnIndex":0},"rowSpan":1,"columnSpan":1},"borderPosition":"ALL","tableBorderProperties":{"dashStyle":"SOLID","weight":{"magnitude":1,"unit":"PT"}},"fields":"dashStyle,weight"}),
        ),
        (
            "update-table-column-properties",
            json!({"objectId":"table_fixture","columnIndices":[0],"tableColumnProperties":{"columnWidth":{"magnitude":40,"unit":"PT"}},"fields":"columnWidth"}),
        ),
        (
            "update-table-row-properties",
            json!({"objectId":"table_fixture","rowIndices":[0],"tableRowProperties":{"minRowHeight":{"magnitude":20,"unit":"PT"}},"fields":"minRowHeight"}),
        ),
        (
            "merge-table-cells",
            json!({"objectId":"table_fixture","tableRange":{"location":{"rowIndex":0,"columnIndex":0},"rowSpan":2,"columnSpan":2}}),
        ),
        (
            "unmerge-table-cells",
            json!({"objectId":"table_fixture","tableRange":{"location":{"rowIndex":0,"columnIndex":0},"rowSpan":2,"columnSpan":2}}),
        ),
        (
            "delete-table-column",
            json!({"tableObjectId":"table_fixture","cellLocation":{"rowIndex":0,"columnIndex":2}}),
        ),
        (
            "delete-table-row",
            json!({"tableObjectId":"table_fixture","cellLocation":{"rowIndex":2,"columnIndex":0}}),
        ),
        (
            "replace-all-text",
            json!({"containsText":{"text":"Hello","matchCase":true},"replaceText":"Welcome"}),
        ),
    ] {
        attempt(run, "slides", &target, command, body);
    }
    run.api(
        "slides",
        "pages.get",
        json!({"presentationId":target,"pageObjectId":page}),
    )?;
    let thumbnail = run.api(
        "slides",
        "pages.get-thumbnail",
        json!({"presentationId":target,"pageObjectId":page}),
    )?;
    ensure!(thumbnail["contentUrl"].is_string(), "thumbnail URL missing");
    attempt(
        run,
        "slides",
        &target,
        "update-slide-properties",
        json!({"objectId":page,"slideProperties":{"isSkipped":true},"fields":"isSkipped"}),
    );
    if let Some(comment) = attempt(
        run,
        "slides",
        &target,
        "insert-comment",
        json!({"content":"Fixture comment","objectId":"shape_fixture"}),
    ) && let Some(comment_id) = find(&comment, "commentId").and_then(Value::as_str)
    {
        if let Some(reply) = attempt(
            run,
            "slides",
            &target,
            "add-comment-reply",
            json!({"commentId":comment_id,"post":{"content":"Fixture reply"}}),
        ) && let Some(post_id) = find(&reply, "postId").and_then(Value::as_str)
        {
            attempt(
                run,
                "slides",
                &target,
                "update-comment-post",
                json!({"commentId":comment_id,"postId":post_id,"content":"Updated fixture reply"}),
            );
            attempt(
                run,
                "slides",
                &target,
                "delete-comment-reply",
                json!({"commentId":comment_id,"postId":post_id}),
            );
        }
        attempt(
            run,
            "slides",
            &target,
            "delete-comment",
            json!({"commentId":comment_id}),
        );
    }
    attempt(
        run,
        "slides",
        &target,
        "delete-text",
        json!({"objectId":"shape_copy","textRange":{"type":"ALL"}}),
    );
    attempt(
        run,
        "slides",
        &target,
        "delete-object",
        json!({"objectId":"shape_copy"}),
    );
    run.api("slides","batch-update",json!({"presentationId":target,"requests":[{"insertText":{"objectId":"shape_fixture","text":"Final fixture text","insertionIndex":0}}]}))?;
    Ok(())
}
