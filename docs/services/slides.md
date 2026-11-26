# Google Slides methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `add-comment-reply` | Adds a reply to a CommentThread. |
| `batch-update` | Applies one or more updates to the presentation |
| `create` | Creates a blank presentation using the title given in the request |
| `create-image` | Creates an image. |
| `create-line` | Creates a line. |
| `create-paragraph-bullets` | Creates bullets for paragraphs. |
| `create-shape` | Creates a new shape. |
| `create-sheets-chart` | Creates an embedded Google Sheets chart. |
| `create-slide` | Creates a new slide. |
| `create-table` | Creates a new table. |
| `create-video` | Creates a video. |
| `delete-comment` | Deletes a CommentThread. |
| `delete-comment-reply` | Deletes a reply Post from a CommentThread. |
| `delete-object` | Deletes a page or page element from the presentation. |
| `delete-paragraph-bullets` | Deletes bullets from paragraphs. |
| `delete-table-column` | Deletes a column from a table. |
| `delete-table-row` | Deletes a row from a table. |
| `delete-text` | Deletes text from a shape or a table cell. |
| `duplicate-object` | Duplicates a slide or page element. |
| `get` | Gets the latest version of the specified presentation. |
| `group-objects` | Groups objects, such as page elements. |
| `insert-comment` | Inserts a CommentThread into the presentation. |
| `insert-table-columns` | Inserts columns into a table. |
| `insert-table-rows` | Inserts rows into a table. |
| `insert-text` | Inserts text into a shape or table cell. |
| `merge-table-cells` | Merges cells in a Table. |
| `pages.get` | Gets the latest version of the specified page in the presentation. |
| `pages.get-thumbnail` | Generates a thumbnail of the latest version of the specified page in the presentation and returns a URL to the thumbnail image |
| `refresh-sheets-chart` | Refreshes a Google Sheets chart. |
| `replace-all-shapes-with-image` | Replaces all shapes matching some criteria with an image. |
| `replace-all-shapes-with-sheets-chart` | Replaces all shapes matching some criteria with a Google Sheets chart. |
| `replace-all-text` | Replaces all instances of specified text. |
| `replace-image` | Replaces an existing image with a new image. |
| `reroute-line` | Reroutes a line such that it's connected at the two closest connection sites on the connected page elements. |
| `ungroup-objects` | Ungroups objects, such as groups. |
| `unmerge-table-cells` | Unmerges cells in a Table. |
| `update-comment-post` | Updates an existing post (head post or reply) of a CommentThread. |
| `update-image-properties` | Updates the properties of an Image. |
| `update-line-category` | Updates the category of a line. |
| `update-line-properties` | Updates the properties of a Line. |
| `update-page-element-alt-text` | Updates the alt text title and/or description of a page element. |
| `update-page-element-transform` | Updates the transform of a page element. |
| `update-page-elements-z-order` | Updates the Z-order of page elements. |
| `update-page-properties` | Updates the properties of a Page. |
| `update-paragraph-style` | Updates the styling of paragraphs within a Shape or Table. |
| `update-shape-properties` | Updates the properties of a Shape. |
| `update-slide-properties` | Updates the properties of a Slide |
| `update-slides-position` | Updates the position of a set of slides in the presentation. |
| `update-table-border-properties` | Updates the properties of the table borders in a Table. |
| `update-table-cell-properties` | Updates the properties of a TableCell. |
| `update-table-column-properties` | Updates the properties of a Table column. |
| `update-table-row-properties` | Updates the properties of a Table row. |
| `update-text-style` | Updates the styling of text within a Shape or Table. |
| `update-video-properties` | Updates the properties of a Video. |

## add-comment-reply

```text
Adds a reply to a CommentThread.

Usage: joog [OPTIONS] slides <ACCOUNT> add-comment-reply [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --comment-id <COMMENT_ID>
          The ID of the CommentThread to add the reply to.

      --post <JSON>
          Represents a single post in a comment thread.

      --post-assignee-email <POST_ASSIGNEE_EMAIL>
          Optional. The email of the user who is being newly assigned to the thread as part of this post. Returns a 400 bad request error if: - The parent thread is a CommentThread whose headPost does not have an assignee. - commentAction is specified as `RESOLVE` or `REOPEN`. - `assignee_email` exceeds 2048 UTF-8 code units.

      --post-author-anonymous[=<BOOL>]
          Whether the user is anonymous.

      --post-author-display-name <POST_AUTHOR_DISPLAY_NAME>
          The display name of the user. May be absent if the author is anonymous.

      --post-author-me[=<BOOL>]
          Whether the user is the authenticated user making the request.

      --post-author-user <POST_AUTHOR_USER>
          The resource name of the post author user, which can also be used to identify the user in the [Google People API](https://developers.google.com/people/api/rest/v1/people). Format: `users/{user}`. Will not be populated if the anonymous field is `true` or if the post is from an imported presentation.

      --post-comment-action <POST_COMMENT_ACTION>
          Action taken as part of creating the post. Choices: COMMENT_ACTION_TYPE_UNSPECIFIED, NO_COMMENT_ACTION_CHANGE, RESOLVE, REOPEN.

      --post-content <POST_CONTENT>
          The content of the post. Required to be non-empty if comment_action is not `RESOLVE` or `REOPEN`. This text content will be handled similarly to comments created in the Slides editor. It will have similar behaviors for formatting, notifications, etc. May not exceed 2048 UTF-8 code units.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## batch-update

```text
Applies one or more updates to the presentation. Each request is validated before being applied. If any request is not valid, then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. Other requests do not need to return information; these each return an empty reply. The order of replies matches that of the requests. For example, suppose you call batchUpdate with four updates, and only the third one returns information. The response would have two empty replies: the reply to the third request, and another empty reply, in that order. Because other users may be editing the presentation, the presentation might not exactly reflect your changes: your changes may be altered with respect to collaborator changes. If there are no collaborators, the presentation should reflect your changes. In any case, the updates in your request are guaranteed to be applied together atomically.

Usage: joog [OPTIONS] slides <ACCOUNT> batch-update [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --requests <JSON>
          A list of updates to apply to the presentation.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --write-control <JSON>
          Provides control over how write requests are executed.

      --write-control-required-revision-id <WRITE_CONTROL_REQUIRED_REVISION_ID>
          The revision ID of the presentation required for the write request. If specified and the required revision ID doesn't match the presentation's current revision ID, the request is not processed and returns a 400 bad request error. When a required revision ID is returned in a response, it indicates the revision ID of the document after the request was applied.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create

```text
Creates a blank presentation using the title given in the request. If a `presentationId` is provided, it is used as the ID of the new presentation. Otherwise, a new ID is generated. Other fields in the request, including any provided content, are ignored. Returns the created presentation.

Usage: joog [OPTIONS] slides <ACCOUNT> create [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --title <TITLE>
          The title of the presentation.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --layouts <JSON>
          The layouts in the presentation. A layout is a template that determines how content is arranged and styled on the slides that inherit from that layout.

      --locale <LOCALE>
          The locale of the presentation, as an IETF BCP 47 language tag.

      --masters <JSON>
          The slide masters in the presentation. A slide master contains all common page elements and the common properties for a set of layouts. They serve three purposes: - Placeholder shapes on a master contain the default text styles and shape properties of all placeholder shapes on pages that use that master. - The master page properties define the common page properties inherited by its layouts. - Any other shapes on the master slide appear on all slides using that master, regardless of their layout.

      --notes-master <JSON>
          A page in a presentation.

      --notes-master-layout-properties-display-name <NOTES_MASTER_LAYOUT_PROPERTIES_DISPLAY_NAME>
          The human-readable name of the layout.

      --notes-master-layout-properties-master-object-id <NOTES_MASTER_LAYOUT_PROPERTIES_MASTER_OBJECT_ID>
          The object ID of the master that this layout is based on.

      --notes-master-layout-properties-name <NOTES_MASTER_LAYOUT_PROPERTIES_NAME>
          The name of the layout.

      --notes-master-master-properties-display-name <NOTES_MASTER_MASTER_PROPERTIES_DISPLAY_NAME>
          The human-readable name of the master.

      --notes-master-notes-properties-speaker-notes-object-id <NOTES_MASTER_NOTES_PROPERTIES_SPEAKER_NOTES_OBJECT_ID>
          The object ID of the shape on this notes page that contains the speaker notes for the corresponding slide. The actual shape may not always exist on the notes page. Inserting text using this object ID will automatically create the shape. In this case, the actual shape may have different object ID. The `GetPresentation` or `GetPage` action will always return the latest object ID.

      --notes-master-object-id <NOTES_MASTER_OBJECT_ID>
          The object ID for this page. Object IDs used by Page and PageElement share the same namespace.

      --notes-master-page-elements <JSON>
          The page elements rendered on the page.

      --notes-master-page-properties-color-scheme-colors <JSON>
          The ThemeColorType and corresponding concrete color pairs.

      --notes-master-page-properties-page-background-fill-property-state <NOTES_MASTER_PAGE_PROPERTIES_PAGE_BACKGROUND_FILL_PROPERTY_STATE>
          The background fill property state. Updating the fill on a page will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a page, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --notes-master-page-type <NOTES_MASTER_PAGE_TYPE>
          The type of the page. Choices: SLIDE, MASTER, LAYOUT, NOTES, NOTES_MASTER.

      --notes-master-slide-properties-is-skipped[=<BOOL>]
          Whether the slide is skipped in the presentation mode. Defaults to false.

      --notes-master-slide-properties-layout-object-id <NOTES_MASTER_SLIDE_PROPERTIES_LAYOUT_OBJECT_ID>
          The object ID of the layout that this slide is based on. This property is read-only.

      --notes-master-slide-properties-master-object-id <NOTES_MASTER_SLIDE_PROPERTIES_MASTER_OBJECT_ID>
          The object ID of the master that this slide is based on. This property is read-only.

      --notes-master-slide-properties-notes-page-object-id <NOTES_MASTER_SLIDE_PROPERTIES_NOTES_PAGE_OBJECT_ID>
          The object ID for this page. Object IDs used by Page and PageElement share the same namespace.

      --notes-master-slide-properties-notes-page-page-elements <JSON>
          The page elements rendered on the page.

      --notes-master-slide-properties-notes-page-page-type <NOTES_MASTER_SLIDE_PROPERTIES_NOTES_PAGE_PAGE_TYPE>
          The type of the page. Choices: SLIDE, MASTER, LAYOUT, NOTES, NOTES_MASTER.

      --page-size <JSON>
          A width and height.

      --page-size-height-magnitude <PAGE_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --page-size-height-unit <PAGE_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --page-size-width-magnitude <PAGE_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --page-size-width-unit <PAGE_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --presentation-id <PRESENTATION_ID>
          The ID of the presentation.

      --slides <JSON>
          The slides in the presentation. A slide inherits properties from a slide layout.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-image

```text
Creates an image.

Usage: joog [OPTIONS] slides <ACCOUNT> create-image [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --element-properties <JSON>
          Common properties for a page element. Note: When you initially create a PageElement, the API may modify the values of both `size` and `transform`, but the visual size will be unchanged.

      --element-properties-page-object-id <ELEMENT_PROPERTIES_PAGE_OBJECT_ID>
          The object ID of the page where the element is located.

      --element-properties-size-height-magnitude <ELEMENT_PROPERTIES_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --element-properties-size-height-unit <ELEMENT_PROPERTIES_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-size-width-magnitude <ELEMENT_PROPERTIES_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --element-properties-size-width-unit <ELEMENT_PROPERTIES_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-transform-scale-x <ELEMENT_PROPERTIES_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --element-properties-transform-scale-y <ELEMENT_PROPERTIES_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --element-properties-transform-shear-x <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --element-properties-transform-shear-y <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --element-properties-transform-translate-x <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --element-properties-transform-translate-y <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --element-properties-transform-unit <ELEMENT_PROPERTIES_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --object-id <OBJECT_ID>
          A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated.

      --url <URL>
          The image URL. The image is fetched once at insertion time and a copy is stored for display inside the presentation. Images must be less than 50 MB in size, can't exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF formats. The provided URL must be publicly accessible and up to 2 KB in length. The URL is saved with the image, and exposed through the Image.source_url field.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-line

```text
Creates a line.

Usage: joog [OPTIONS] slides <ACCOUNT> create-line [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --category <CATEGORY>
          The category of the line to be created. The exact line type created is determined based on the category and how it's routed to connect to other page elements. If you specify both a `category` and a `line_category`, the `category` takes precedence. If you do not specify a value for `category`, but specify a value for `line_category`, then the specified `line_category` value is used. If you do not specify either, then STRAIGHT is used. Choices: LINE_CATEGORY_UNSPECIFIED, STRAIGHT, BENT, CURVED.

      --element-properties <JSON>
          Common properties for a page element. Note: When you initially create a PageElement, the API may modify the values of both `size` and `transform`, but the visual size will be unchanged.

      --element-properties-page-object-id <ELEMENT_PROPERTIES_PAGE_OBJECT_ID>
          The object ID of the page where the element is located.

      --element-properties-size-height-magnitude <ELEMENT_PROPERTIES_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --element-properties-size-height-unit <ELEMENT_PROPERTIES_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-size-width-magnitude <ELEMENT_PROPERTIES_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --element-properties-size-width-unit <ELEMENT_PROPERTIES_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-transform-scale-x <ELEMENT_PROPERTIES_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --element-properties-transform-scale-y <ELEMENT_PROPERTIES_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --element-properties-transform-shear-x <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --element-properties-transform-shear-y <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --element-properties-transform-translate-x <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --element-properties-transform-translate-y <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --element-properties-transform-unit <ELEMENT_PROPERTIES_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --line-category <LINE_CATEGORY>
          The category of the line to be created. *Deprecated*: use `category` instead. The exact line type created is determined based on the category and how it's routed to connect to other page elements. If you specify both a `category` and a `line_category`, the `category` takes precedence. Choices: STRAIGHT, BENT, CURVED.

      --object-id <OBJECT_ID>
          A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-paragraph-bullets

```text
Creates bullets for paragraphs.

Usage: joog [OPTIONS] slides <ACCOUNT> create-paragraph-bullets [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --text-range <JSON>
          Specifies a contiguous range of an indexed collection, such as characters in text.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --bullet-preset <BULLET_PRESET>
          The kinds of bullet glyphs to be used. Defaults to the `BULLET_DISC_CIRCLE_SQUARE` preset. Choices: BULLET_DISC_CIRCLE_SQUARE, BULLET_DIAMONDX_ARROW3D_SQUARE, BULLET_CHECKBOX, BULLET_ARROW_DIAMOND_DISC, BULLET_STAR_CIRCLE_SQUARE, BULLET_ARROW3D_CIRCLE_SQUARE, BULLET_LEFTTRIANGLE_DIAMOND_DISC, BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE, BULLET_DIAMOND_CIRCLE_SQUARE, NUMBERED_DIGIT_ALPHA_ROMAN, NUMBERED_DIGIT_ALPHA_ROMAN_PARENS, NUMBERED_DIGIT_NESTED, NUMBERED_UPPERALPHA_ALPHA_ROMAN, NUMBERED_UPPERROMAN_UPPERALPHA_DIGIT, NUMBERED_ZERODIGIT_ALPHA_ROMAN.

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --object-id <OBJECT_ID>
          The object ID of the shape or table containing the text to add bullets to.

      --text-range-end-index <TEXT_RANGE_END_INDEX>
          The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges.

      --text-range-start-index <TEXT_RANGE_START_INDEX>
          The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges.

      --text-range-type <TEXT_RANGE_TYPE>
          The type of range. Choices: RANGE_TYPE_UNSPECIFIED, FIXED_RANGE, FROM_START_INDEX, ALL.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-shape

```text
Creates a new shape.

Usage: joog [OPTIONS] slides <ACCOUNT> create-shape [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --element-properties <JSON>
          Common properties for a page element. Note: When you initially create a PageElement, the API may modify the values of both `size` and `transform`, but the visual size will be unchanged.

      --element-properties-page-object-id <ELEMENT_PROPERTIES_PAGE_OBJECT_ID>
          The object ID of the page where the element is located.

      --element-properties-size-height-magnitude <ELEMENT_PROPERTIES_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --element-properties-size-height-unit <ELEMENT_PROPERTIES_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-size-width-magnitude <ELEMENT_PROPERTIES_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --element-properties-size-width-unit <ELEMENT_PROPERTIES_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-transform-scale-x <ELEMENT_PROPERTIES_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --element-properties-transform-scale-y <ELEMENT_PROPERTIES_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --element-properties-transform-shear-x <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --element-properties-transform-shear-y <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --element-properties-transform-translate-x <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --element-properties-transform-translate-y <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --element-properties-transform-unit <ELEMENT_PROPERTIES_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --object-id <OBJECT_ID>
          A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If empty, a unique identifier will be generated.

      --shape-type <SHAPE_TYPE>
          The shape type. Choices: TYPE_UNSPECIFIED, TEXT_BOX, RECTANGLE, ROUND_RECTANGLE, ELLIPSE, ARC, BENT_ARROW, BENT_UP_ARROW, BEVEL, BLOCK_ARC, BRACE_PAIR, BRACKET_PAIR, CAN, CHEVRON, CHORD, CLOUD, CORNER, CUBE, CURVED_DOWN_ARROW, CURVED_LEFT_ARROW, CURVED_RIGHT_ARROW, CURVED_UP_ARROW, DECAGON, DIAGONAL_STRIPE, DIAMOND, DODECAGON, DONUT, DOUBLE_WAVE, DOWN_ARROW, DOWN_ARROW_CALLOUT, FOLDED_CORNER, FRAME, HALF_FRAME, HEART, HEPTAGON, HEXAGON, HOME_PLATE, HORIZONTAL_SCROLL, IRREGULAR_SEAL_1, IRREGULAR_SEAL_2, LEFT_ARROW, LEFT_ARROW_CALLOUT, LEFT_BRACE, LEFT_BRACKET, LEFT_RIGHT_ARROW, LEFT_RIGHT_ARROW_CALLOUT, LEFT_RIGHT_UP_ARROW, LEFT_UP_ARROW, LIGHTNING_BOLT, MATH_DIVIDE, MATH_EQUAL, MATH_MINUS, MATH_MULTIPLY, MATH_NOT_EQUAL, MATH_PLUS, MOON, NO_SMOKING, NOTCHED_RIGHT_ARROW, OCTAGON, PARALLELOGRAM, PENTAGON, PIE, PLAQUE, PLUS, QUAD_ARROW, QUAD_ARROW_CALLOUT, RIBBON, RIBBON_2, RIGHT_ARROW, RIGHT_ARROW_CALLOUT, RIGHT_BRACE, RIGHT_BRACKET, ROUND_1_RECTANGLE, ROUND_2_DIAGONAL_RECTANGLE, ROUND_2_SAME_RECTANGLE, RIGHT_TRIANGLE, SMILEY_FACE, SNIP_1_RECTANGLE, SNIP_2_DIAGONAL_RECTANGLE, SNIP_2_SAME_RECTANGLE, SNIP_ROUND_RECTANGLE, STAR_10, STAR_12, STAR_16, STAR_24, STAR_32, STAR_4, STAR_5, STAR_6, STAR_7, STAR_8, STRIPED_RIGHT_ARROW, SUN, TRAPEZOID, TRIANGLE, UP_ARROW, UP_ARROW_CALLOUT, UP_DOWN_ARROW, UTURN_ARROW, VERTICAL_SCROLL, WAVE, WEDGE_ELLIPSE_CALLOUT, WEDGE_RECTANGLE_CALLOUT, WEDGE_ROUND_RECTANGLE_CALLOUT, FLOW_CHART_ALTERNATE_PROCESS, FLOW_CHART_COLLATE, FLOW_CHART_CONNECTOR, FLOW_CHART_DECISION, FLOW_CHART_DELAY, FLOW_CHART_DISPLAY, FLOW_CHART_DOCUMENT, FLOW_CHART_EXTRACT, FLOW_CHART_INPUT_OUTPUT, FLOW_CHART_INTERNAL_STORAGE, FLOW_CHART_MAGNETIC_DISK, FLOW_CHART_MAGNETIC_DRUM, FLOW_CHART_MAGNETIC_TAPE, FLOW_CHART_MANUAL_INPUT, FLOW_CHART_MANUAL_OPERATION, FLOW_CHART_MERGE, FLOW_CHART_MULTIDOCUMENT, FLOW_CHART_OFFLINE_STORAGE, FLOW_CHART_OFFPAGE_CONNECTOR, FLOW_CHART_ONLINE_STORAGE, FLOW_CHART_OR, FLOW_CHART_PREDEFINED_PROCESS, FLOW_CHART_PREPARATION, FLOW_CHART_PROCESS, FLOW_CHART_PUNCHED_CARD, FLOW_CHART_PUNCHED_TAPE, FLOW_CHART_SORT, FLOW_CHART_SUMMING_JUNCTION, FLOW_CHART_TERMINATOR, ARROW_EAST, ARROW_NORTH_EAST, ARROW_NORTH, SPEECH, STARBURST, TEARDROP, ELLIPSE_RIBBON, ELLIPSE_RIBBON_2, CLOUD_CALLOUT, CUSTOM.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-sheets-chart

```text
Creates an embedded Google Sheets chart.

Usage: joog [OPTIONS] slides <ACCOUNT> create-sheets-chart [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --chart-id <CHART_ID>
          The ID of the specific chart in the Google Sheets spreadsheet.

      --element-properties <JSON>
          Common properties for a page element. Note: When you initially create a PageElement, the API may modify the values of both `size` and `transform`, but the visual size will be unchanged.

      --element-properties-page-object-id <ELEMENT_PROPERTIES_PAGE_OBJECT_ID>
          The object ID of the page where the element is located.

      --element-properties-size-height-magnitude <ELEMENT_PROPERTIES_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --element-properties-size-height-unit <ELEMENT_PROPERTIES_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-size-width-magnitude <ELEMENT_PROPERTIES_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --element-properties-size-width-unit <ELEMENT_PROPERTIES_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-transform-scale-x <ELEMENT_PROPERTIES_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --element-properties-transform-scale-y <ELEMENT_PROPERTIES_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --element-properties-transform-shear-x <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --element-properties-transform-shear-y <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --element-properties-transform-translate-x <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --element-properties-transform-translate-y <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --element-properties-transform-unit <ELEMENT_PROPERTIES_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --linking-mode <LINKING_MODE>
          The mode with which the chart is linked to the source spreadsheet. When not specified, the chart will be an image that is not linked. Choices: NOT_LINKED_IMAGE, LINKED.

      --object-id <OBJECT_ID>
          A user-supplied object ID. If specified, the ID must be unique among all pages and page elements in the presentation. The ID should start with a word character [a-zA-Z0-9_] and then followed by any number of the following characters [a-zA-Z0-9_-:]. The length of the ID should not be less than 5 or greater than 50. If empty, a unique identifier will be generated.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the Google Sheets spreadsheet that contains the chart. You might need to add a resource key to the HTTP header for a subset of old files. For more information, see [Access link-shared files using resource keys](https://developers.google.com/drive/api/v3/resource-keys).

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-slide

```text
Creates a new slide.

Usage: joog [OPTIONS] slides <ACCOUNT> create-slide [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --layout <LAYOUT>
          Predefined layout. Choices: PREDEFINED_LAYOUT_UNSPECIFIED, BLANK, CAPTION_ONLY, TITLE, TITLE_AND_BODY, TITLE_AND_TWO_COLUMNS, TITLE_ONLY, SECTION_HEADER, SECTION_TITLE_AND_DESCRIPTION, ONE_COLUMN_TEXT, MAIN_POINT, BIG_NUMBER.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --insertion-index <INSERTION_INDEX>
          The optional zero-based index indicating where to insert the slides. If you don't specify an index, the slide is created at the end.

      --object-id <OBJECT_ID>
          A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The ID length must be between 5 and 50 characters, inclusive. If you don't specify an ID, a unique one is generated.

      --placeholder-id-mappings <JSON>
          An optional list of object ID mappings from the placeholder(s) on the layout to the placeholders that are created on the slide from the specified layout. Can only be used when `slide_layout_reference` is specified.

      --slide-layout-reference <JSON>
          Slide layout reference. This may reference either: - A predefined layout - One of the layouts in the presentation.

      --slide-layout-reference-layout-id <SLIDE_LAYOUT_REFERENCE_LAYOUT_ID>
          Layout ID: the object ID of one of the layouts in the presentation.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-table

```text
Creates a new table.

Usage: joog [OPTIONS] slides <ACCOUNT> create-table [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --columns <COLUMNS>
          Number of columns in the table.

      --element-properties <JSON>
          Common properties for a page element. Note: When you initially create a PageElement, the API may modify the values of both `size` and `transform`, but the visual size will be unchanged.

      --element-properties-page-object-id <ELEMENT_PROPERTIES_PAGE_OBJECT_ID>
          The object ID of the page where the element is located.

      --element-properties-size-height-magnitude <ELEMENT_PROPERTIES_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --element-properties-size-height-unit <ELEMENT_PROPERTIES_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-size-width-magnitude <ELEMENT_PROPERTIES_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --element-properties-size-width-unit <ELEMENT_PROPERTIES_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-transform-scale-x <ELEMENT_PROPERTIES_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --element-properties-transform-scale-y <ELEMENT_PROPERTIES_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --element-properties-transform-shear-x <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --element-properties-transform-shear-y <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --element-properties-transform-translate-x <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --element-properties-transform-translate-y <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --element-properties-transform-unit <ELEMENT_PROPERTIES_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --object-id <OBJECT_ID>
          A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated.

      --rows <ROWS>
          Number of rows in the table.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-video

```text
Creates a video.

Usage: joog [OPTIONS] slides <ACCOUNT> create-video [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --element-properties <JSON>
          Common properties for a page element. Note: When you initially create a PageElement, the API may modify the values of both `size` and `transform`, but the visual size will be unchanged.

      --element-properties-page-object-id <ELEMENT_PROPERTIES_PAGE_OBJECT_ID>
          The object ID of the page where the element is located.

      --element-properties-size-height-magnitude <ELEMENT_PROPERTIES_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --element-properties-size-height-unit <ELEMENT_PROPERTIES_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-size-width-magnitude <ELEMENT_PROPERTIES_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --element-properties-size-width-unit <ELEMENT_PROPERTIES_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --element-properties-transform-scale-x <ELEMENT_PROPERTIES_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --element-properties-transform-scale-y <ELEMENT_PROPERTIES_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --element-properties-transform-shear-x <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --element-properties-transform-shear-y <ELEMENT_PROPERTIES_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --element-properties-transform-translate-x <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --element-properties-transform-translate-y <ELEMENT_PROPERTIES_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --element-properties-transform-unit <ELEMENT_PROPERTIES_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --id <ID>
          The video source's unique identifier for this video. e.g. For YouTube video https://www.youtube.com/watch?v=7U3axjORYZ0, the ID is 7U3axjORYZ0. For a Google Drive video https://drive.google.com/file/d/1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q the ID is 1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q. To access a Google Drive video file, you might need to add a resource key to the HTTP header for a subset of old files. For more information, see [Access link-shared files using resource keys](https://developers.google.com/drive/api/v3/resource-keys).

      --object-id <OBJECT_ID>
          A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated.

      --source <SOURCE>
          The video source. Choices: SOURCE_UNSPECIFIED, YOUTUBE, DRIVE.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-comment

```text
Deletes a CommentThread.

Usage: joog [OPTIONS] slides <ACCOUNT> delete-comment [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --comment-id <COMMENT_ID>
          The ID of the CommentThread that is being deleted.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-comment-reply

```text
Deletes a reply Post from a CommentThread.

Usage: joog [OPTIONS] slides <ACCOUNT> delete-comment-reply [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --comment-id <COMMENT_ID>
          The ID of the CommentThread which the post belongs to.

      --post-id <POST_ID>
          The ID of the reply Post being deleted.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-object

```text
Deletes a page or page element from the presentation.

Usage: joog [OPTIONS] slides <ACCOUNT> delete-object [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the page or page element to delete. If after a delete operation a group contains only 1 or no page elements, the group is also deleted. If a placeholder is deleted on a layout, any empty inheriting placeholders are also deleted.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-paragraph-bullets

```text
Deletes bullets from paragraphs.

Usage: joog [OPTIONS] slides <ACCOUNT> delete-paragraph-bullets [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --text-range <JSON>
          Specifies a contiguous range of an indexed collection, such as characters in text.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --object-id <OBJECT_ID>
          The object ID of the shape or table containing the text to delete bullets from.

      --text-range-end-index <TEXT_RANGE_END_INDEX>
          The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges.

      --text-range-start-index <TEXT_RANGE_START_INDEX>
          The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges.

      --text-range-type <TEXT_RANGE_TYPE>
          The type of range. Choices: RANGE_TYPE_UNSPECIFIED, FIXED_RANGE, FROM_START_INDEX, ALL.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-table-column

```text
Deletes a column from a table.

Usage: joog [OPTIONS] slides <ACCOUNT> delete-table-column [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-object-id <TABLE_OBJECT_ID>
          The table to delete columns from.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-table-row

```text
Deletes a row from a table.

Usage: joog [OPTIONS] slides <ACCOUNT> delete-table-row [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-object-id <TABLE_OBJECT_ID>
          The table to delete rows from.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-text

```text
Deletes text from a shape or a table cell.

Usage: joog [OPTIONS] slides <ACCOUNT> delete-text [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --text-range <JSON>
          Specifies a contiguous range of an indexed collection, such as characters in text.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --object-id <OBJECT_ID>
          The object ID of the shape or table from which the text will be deleted.

      --text-range-end-index <TEXT_RANGE_END_INDEX>
          The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges.

      --text-range-start-index <TEXT_RANGE_START_INDEX>
          The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges.

      --text-range-type <TEXT_RANGE_TYPE>
          The type of range. Choices: RANGE_TYPE_UNSPECIFIED, FIXED_RANGE, FROM_START_INDEX, ALL.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## duplicate-object

```text
Duplicates a slide or page element.

Usage: joog [OPTIONS] slides <ACCOUNT> duplicate-object [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The ID of the object to duplicate.

      --object-ids <JSON>
          The object being duplicated may contain other objects, for example when duplicating a slide or a group page element. This map defines how the IDs of duplicated objects are generated: the keys are the IDs of the original objects and its values are the IDs that will be assigned to the corresponding duplicate object. The ID of the source object's duplicate may be specified in this map as well, using the same value of the `object_id` field as a key and the newly desired ID as the value. All keys must correspond to existing IDs in the presentation. All values must be unique in the presentation and must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the new ID must not be less than 5 or greater than 50. If any IDs of source objects are omitted from the map, a new random ID will be assigned. If the map is empty or unset, all duplicate objects will receive a new random ID.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## get

```text
Gets the latest version of the specified presentation.

Usage: joog [OPTIONS] slides <ACCOUNT> get [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The ID of the presentation to retrieve.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

      --comments-view-mode <COMMENTS_VIEW_MODE>
          The comments view mode to apply to the presentation. This allows viewing the presentation with comments omitted or included. If one is not specified, COMMENTS_VIEW_MODE_OMITTED is used. Choices: COMMENTS_VIEW_MODE_UNSPECIFIED, COMMENTS_VIEW_MODE_DEFAULT_FOR_CURRENT_ACCESS, COMMENTS_VIEW_MODE_OMITTED, COMMENTS_VIEW_MODE_INCLUDED.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## group-objects

```text
Groups objects, such as page elements.

Usage: joog [OPTIONS] slides <ACCOUNT> group-objects [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --children-object-ids <CHILDREN_OBJECT_IDS>...
          The object IDs of the objects to group. Only page elements can be grouped. There should be at least two page elements on the same page that are not already in another group. Some page elements, such as videos, tables and placeholders cannot be grouped.

      --group-object-id <GROUP_OBJECT_ID>
          A user-supplied object ID for the group to be created. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-comment

```text
Inserts a CommentThread into the presentation.

Usage: joog [OPTIONS] slides <ACCOUNT> insert-comment [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --content <CONTENT>
          The text of the comment, as plain text. This text content will be handled similarly to comments created in the Slides editor. It will have similar behaviors for formatting, notifications, etc. This field cannot be empty, and must not exceed 2048 UTF-8 code units.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --assignee-email-address <ASSIGNEE_EMAIL_ADDRESS>
          Optional. The email address of the assignee of the comment. Leave empty for a non-assigned comment. May not exceed 2048 UTF-8 code units.

      --object-id <OBJECT_ID>
          The object ID of the Page or PageElement that is tied to this comment. If the specified object_id is a Group, the comment will be anchored to at most 100 of the group's non-group descendants.

      --shape-text-anchor <JSON>
          An anchor to a specific range of text within a Shape's text. To insert comments in speaker notes, use the ShapeTextAnchor with the speaker notes object ID.

      --shape-text-anchor-object-id <SHAPE_TEXT_ANCHOR_OBJECT_ID>
          The object ID of the page element containing the text.

      --shape-text-anchor-text-range-end-index <SHAPE_TEXT_ANCHOR_TEXT_RANGE_END_INDEX>
          The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges.

      --shape-text-anchor-text-range-start-index <SHAPE_TEXT_ANCHOR_TEXT_RANGE_START_INDEX>
          The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges.

      --shape-text-anchor-text-range-type <SHAPE_TEXT_ANCHOR_TEXT_RANGE_TYPE>
          The type of range. Choices: RANGE_TYPE_UNSPECIFIED, FIXED_RANGE, FROM_START_INDEX, ALL.

      --table-anchor <JSON>
          An anchor to a specific range of cells within a Table. Used to anchor a comment to all of the text in each cell in a range within a table.

      --table-anchor-object-id <TABLE_ANCHOR_OBJECT_ID>
          The object ID of the table.

      --table-anchor-table-range-column-span <TABLE_ANCHOR_TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-anchor-table-range-location-column-index <TABLE_ANCHOR_TABLE_RANGE_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --table-anchor-table-range-location-row-index <TABLE_ANCHOR_TABLE_RANGE_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-anchor-table-range-row-span <TABLE_ANCHOR_TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --table-cell-text-anchor <JSON>
          An anchor to a specific range of text within a TableCell's TextElement.

      --table-cell-text-anchor-cell-location-column-index <TABLE_CELL_TEXT_ANCHOR_CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --table-cell-text-anchor-cell-location-row-index <TABLE_CELL_TEXT_ANCHOR_CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-cell-text-anchor-object-id <TABLE_CELL_TEXT_ANCHOR_OBJECT_ID>
          The object ID of the table containing the cell.

      --table-cell-text-anchor-text-range-end-index <TABLE_CELL_TEXT_ANCHOR_TEXT_RANGE_END_INDEX>
          The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges.

      --table-cell-text-anchor-text-range-start-index <TABLE_CELL_TEXT_ANCHOR_TEXT_RANGE_START_INDEX>
          The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges.

      --table-cell-text-anchor-text-range-type <TABLE_CELL_TEXT_ANCHOR_TEXT_RANGE_TYPE>
          The type of range. Choices: RANGE_TYPE_UNSPECIFIED, FIXED_RANGE, FROM_START_INDEX, ALL.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-table-columns

```text
Inserts columns into a table.

Usage: joog [OPTIONS] slides <ACCOUNT> insert-table-columns [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --insert-right[=<BOOL>]
          Whether to insert new columns to the right of the reference cell location. - `True`: insert to the right. - `False`: insert to the left.

      --number <NUMBER>
          The number of columns to be inserted. Maximum 20 per request.

      --table-object-id <TABLE_OBJECT_ID>
          The table to insert columns into.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-table-rows

```text
Inserts rows into a table.

Usage: joog [OPTIONS] slides <ACCOUNT> insert-table-rows [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --insert-below[=<BOOL>]
          Whether to insert new rows below the reference cell location. - `True`: insert below the cell. - `False`: insert above the cell.

      --number <NUMBER>
          The number of rows to be inserted. Maximum 20 per request.

      --table-object-id <TABLE_OBJECT_ID>
          The table to insert rows into.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-text

```text
Inserts text into a shape or table cell.

Usage: joog [OPTIONS] slides <ACCOUNT> insert-text [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --text <TEXT>
          The text to be inserted. Inserting a newline character will implicitly create a new ParagraphMarker at that index. The paragraph style of the new paragraph will be copied from the paragraph at the current insertion index, including lists and bullets. Text styles for inserted text will be determined automatically, generally preserving the styling of neighboring text. In most cases, the text will be added to the TextRun that exists at the insertion index. Some control characters (U+0000-U+0008, U+000C-U+001F) and characters from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF) will be stripped out of the inserted text.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --insertion-index <INSERTION_INDEX>
          The index where the text will be inserted, in Unicode code units, based on TextElement indexes. The index is zero-based and is computed from the start of the string. The index may be adjusted to prevent insertions inside Unicode grapheme clusters. In these cases, the text will be inserted immediately after the grapheme cluster.

      --object-id <OBJECT_ID>
          The object ID of the shape or table where the text will be inserted.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## merge-table-cells

```text
Merges cells in a Table.

Usage: joog [OPTIONS] slides <ACCOUNT> merge-table-cells [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the table.

      --table-range <JSON>
          A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with location = (0, 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]

      --table-range-column-span <TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-range-location-column-index <TABLE_RANGE_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --table-range-location-row-index <TABLE_RANGE_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-range-row-span <TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## pages.get

```text
Gets the latest version of the specified page in the presentation.

Usage: joog [OPTIONS] slides <ACCOUNT> pages.get [COMMAND_OPTIONS]

Mandatory parameters:
      --page-object-id <PAGE_OBJECT_ID>
          The object ID of the page to retrieve.

      --presentation-id <PRESENTATION_ID>
          The ID of the presentation to retrieve.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

      --comments-view-mode <COMMENTS_VIEW_MODE>
          The comments view mode to apply to the page. This allows viewing the page with comments omitted or included. If one is not specified, COMMENTS_VIEW_MODE_OMITTED is used. Choices: COMMENTS_VIEW_MODE_UNSPECIFIED, COMMENTS_VIEW_MODE_DEFAULT_FOR_CURRENT_ACCESS, COMMENTS_VIEW_MODE_OMITTED, COMMENTS_VIEW_MODE_INCLUDED.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## pages.get-thumbnail

```text
Generates a thumbnail of the latest version of the specified page in the presentation and returns a URL to the thumbnail image. This request counts as an [expensive read request](https://developers.google.com/workspace/slides/limits) for quota purposes.

Usage: joog [OPTIONS] slides <ACCOUNT> pages.get-thumbnail [COMMAND_OPTIONS]

Mandatory parameters:
      --page-object-id <PAGE_OBJECT_ID>
          The object ID of the page whose thumbnail to retrieve.

      --presentation-id <PRESENTATION_ID>
          The ID of the presentation to retrieve.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

      --thumbnail-properties-mime-type <THUMBNAIL_PROPERTIES_MIME_TYPE>
          The optional mime type of the thumbnail image. If you don't specify the mime type, the mime type defaults to PNG. Choices: PNG.

      --thumbnail-properties-thumbnail-size <THUMBNAIL_PROPERTIES_THUMBNAIL_SIZE>
          The optional thumbnail image size. If you don't specify the size, the server chooses a default size of the image. Choices: THUMBNAIL_SIZE_UNSPECIFIED, LARGE, MEDIUM, SMALL, WIDTH2000_PX.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## refresh-sheets-chart

```text
Refreshes a Google Sheets chart.

Usage: joog [OPTIONS] slides <ACCOUNT> refresh-sheets-chart [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the chart to refresh.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replace-all-shapes-with-image

```text
Replaces all shapes matching some criteria with an image.

Usage: joog [OPTIONS] slides <ACCOUNT> replace-all-shapes-with-image [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --find <FIND>
          The text to search for in the shape or table.

      --match-case[=<BOOL>]
          Indicates whether the search should respect case: - `True`: the search is case sensitive. - `False`: the search is case insensitive.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --contains-text <JSON>
          A criteria that matches a specific string of text in a shape or table.

      --contains-text-search-by-regex[=<BOOL>]
          Optional. True if the find value should be treated as a regular expression. Any backslashes in the pattern should be escaped. - `True`: the search text is treated as a regular expressions. - `False`: the search text is treated as a substring for matching.

      --image-replace-method <IMAGE_REPLACE_METHOD>
          The image replace method. If you specify both a `replace_method` and an `image_replace_method`, the `image_replace_method` takes precedence. If you do not specify a value for `image_replace_method`, but specify a value for `replace_method`, then the specified `replace_method` value is used. If you do not specify either, then CENTER_INSIDE is used. Choices: IMAGE_REPLACE_METHOD_UNSPECIFIED, CENTER_INSIDE, CENTER_CROP.

      --image-url <IMAGE_URL>
          The image URL. The image is fetched once at insertion time and a copy is stored for display inside the presentation. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URL can be at most 2 kB in length. The URL itself is saved with the image, and exposed via the Image.source_url field.

      --page-object-ids <PAGE_OBJECT_IDS>...
          If non-empty, limits the matches to page elements only on the given pages. Returns a 400 bad request error if given the page object ID of a notes page or a notes master, or if a page with that object ID doesn't exist in the presentation.

      --replace-method <REPLACE_METHOD>
          The replace method. *Deprecated*: use `image_replace_method` instead. If you specify both a `replace_method` and an `image_replace_method`, the `image_replace_method` takes precedence. Choices: CENTER_INSIDE, CENTER_CROP.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replace-all-shapes-with-sheets-chart

```text
Replaces all shapes matching some criteria with a Google Sheets chart.

Usage: joog [OPTIONS] slides <ACCOUNT> replace-all-shapes-with-sheets-chart [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --find <FIND>
          The text to search for in the shape or table.

      --match-case[=<BOOL>]
          Indicates whether the search should respect case: - `True`: the search is case sensitive. - `False`: the search is case insensitive.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --chart-id <CHART_ID>
          The ID of the specific chart in the Google Sheets spreadsheet.

      --contains-text <JSON>
          A criteria that matches a specific string of text in a shape or table.

      --contains-text-search-by-regex[=<BOOL>]
          Optional. True if the find value should be treated as a regular expression. Any backslashes in the pattern should be escaped. - `True`: the search text is treated as a regular expressions. - `False`: the search text is treated as a substring for matching.

      --linking-mode <LINKING_MODE>
          The mode with which the chart is linked to the source spreadsheet. When not specified, the chart will be an image that is not linked. Choices: NOT_LINKED_IMAGE, LINKED.

      --page-object-ids <PAGE_OBJECT_IDS>...
          If non-empty, limits the matches to page elements only on the given pages. Returns a 400 bad request error if given the page object ID of a notes page or a notes master, or if a page with that object ID doesn't exist in the presentation.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the Google Sheets spreadsheet that contains the chart.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replace-all-text

```text
Replaces all instances of specified text.

Usage: joog [OPTIONS] slides <ACCOUNT> replace-all-text [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --find <FIND>
          The text to search for in the shape or table.

      --match-case[=<BOOL>]
          Indicates whether the search should respect case: - `True`: the search is case sensitive. - `False`: the search is case insensitive.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --contains-text <JSON>
          A criteria that matches a specific string of text in a shape or table.

      --contains-text-search-by-regex[=<BOOL>]
          Optional. True if the find value should be treated as a regular expression. Any backslashes in the pattern should be escaped. - `True`: the search text is treated as a regular expressions. - `False`: the search text is treated as a substring for matching.

      --page-object-ids <PAGE_OBJECT_IDS>...
          If non-empty, limits the matches to page elements only on the given pages. Returns a 400 bad request error if given the page object ID of a notes master, or if a page with that object ID doesn't exist in the presentation.

      --replace-text <REPLACE_TEXT>
          The text that will replace the matched text.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replace-image

```text
Replaces an existing image with a new image.

Usage: joog [OPTIONS] slides <ACCOUNT> replace-image [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --image-object-id <IMAGE_OBJECT_ID>
          The ID of the existing image that will be replaced. The ID can be retrieved from the response of a get request.

      --image-replace-method <IMAGE_REPLACE_METHOD>
          The replacement method. Choices: IMAGE_REPLACE_METHOD_UNSPECIFIED, CENTER_INSIDE, CENTER_CROP.

      --url <URL>
          The image URL. The image is fetched once at insertion time and a copy is stored for display inside the presentation. Images must be less than 50MB, cannot exceed 25 megapixels, and must be in PNG, JPEG, or GIF format. The provided URL can't surpass 2 KB in length. The URL is saved with the image, and exposed through the Image.source_url field.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## reroute-line

```text
Reroutes a line such that it's connected at the two closest connection sites on the connected page elements.

Usage: joog [OPTIONS] slides <ACCOUNT> reroute-line [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the line to reroute. Only a line with a category indicating it is a "connector" can be rerouted. The start and end connections of the line must be on different page elements.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## ungroup-objects

```text
Ungroups objects, such as groups.

Usage: joog [OPTIONS] slides <ACCOUNT> ungroup-objects [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-ids <OBJECT_IDS>...
          The object IDs of the objects to ungroup. Only groups that are not inside other groups can be ungrouped. All the groups should be on the same page. The group itself is deleted. The visual sizes and positions of all the children are preserved.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## unmerge-table-cells

```text
Unmerges cells in a Table.

Usage: joog [OPTIONS] slides <ACCOUNT> unmerge-table-cells [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the table.

      --table-range <JSON>
          A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with location = (0, 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]

      --table-range-column-span <TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-range-location-column-index <TABLE_RANGE_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --table-range-location-row-index <TABLE_RANGE_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-range-row-span <TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-comment-post

```text
Updates an existing post (head post or reply) of a CommentThread.

Usage: joog [OPTIONS] slides <ACCOUNT> update-comment-post [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --content <CONTENT>
          The new text of the comment, as plain text. This text content will be handled similarly to comments created in the Slides editor. It will have similar behaviors for formatting, notifications, etc. This field cannot be empty, and must not exceed 2048 UTF-8 code units.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --comment-id <COMMENT_ID>
          The ID of the CommentThread which the post belongs to.

      --post-id <POST_ID>
          The ID of the post being updated.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-image-properties

```text
Updates the properties of an Image.

Usage: joog [OPTIONS] slides <ACCOUNT> update-image-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `imageProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the image outline color, set `fields` to `"outline.outlineFill.solidFill.color"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --image-properties <JSON>
          The properties of the Image.

      --image-properties-brightness <IMAGE_PROPERTIES_BRIGHTNESS>
          The brightness effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect. This property is read-only.

      --image-properties-contrast <IMAGE_PROPERTIES_CONTRAST>
          The contrast effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect. This property is read-only.

      --image-properties-crop-properties-angle <IMAGE_PROPERTIES_CROP_PROPERTIES_ANGLE>
          The rotation angle of the crop window around its center, in radians. Rotation angle is applied after the offset.

      --image-properties-crop-properties-bottom-offset <IMAGE_PROPERTIES_CROP_PROPERTIES_BOTTOM_OFFSET>
          The offset specifies the bottom edge of the crop rectangle that is located above the original bounding rectangle bottom edge, relative to the object's original height.

      --image-properties-crop-properties-left-offset <IMAGE_PROPERTIES_CROP_PROPERTIES_LEFT_OFFSET>
          The offset specifies the left edge of the crop rectangle that is located to the right of the original bounding rectangle left edge, relative to the object's original width.

      --image-properties-crop-properties-right-offset <IMAGE_PROPERTIES_CROP_PROPERTIES_RIGHT_OFFSET>
          The offset specifies the right edge of the crop rectangle that is located to the left of the original bounding rectangle right edge, relative to the object's original width.

      --image-properties-crop-properties-top-offset <IMAGE_PROPERTIES_CROP_PROPERTIES_TOP_OFFSET>
          The offset specifies the top edge of the crop rectangle that is located below the original bounding rectangle top edge, relative to the object's original height.

      --image-properties-link-page-object-id <IMAGE_PROPERTIES_LINK_PAGE_OBJECT_ID>
          If set, indicates this is a link to the specific page in this presentation with this ID. A page with this ID may not exist.

      --image-properties-link-relative-link <IMAGE_PROPERTIES_LINK_RELATIVE_LINK>
          If set, indicates this is a link to a slide in this presentation, addressed by its position. Choices: RELATIVE_SLIDE_LINK_UNSPECIFIED, NEXT_SLIDE, PREVIOUS_SLIDE, FIRST_SLIDE, LAST_SLIDE.

      --image-properties-link-slide-index <IMAGE_PROPERTIES_LINK_SLIDE_INDEX>
          If set, indicates this is a link to the slide at this zero-based index in the presentation. There may not be a slide at this index.

      --image-properties-link-url <IMAGE_PROPERTIES_LINK_URL>
          If set, indicates this is a link to the external web page at this URL.

      --image-properties-outline-dash-style <IMAGE_PROPERTIES_OUTLINE_DASH_STYLE>
          The dash style of the outline. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH, DASH_DOT, LONG_DASH, LONG_DASH_DOT.

      --image-properties-outline-property-state <IMAGE_PROPERTIES_OUTLINE_PROPERTY_STATE>
          The outline property state. Updating the outline on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no outline on a page element, set this field to `NOT_RENDERED`. In this case, any other outline fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --image-properties-outline-weight-magnitude <IMAGE_PROPERTIES_OUTLINE_WEIGHT_MAGNITUDE>
          The magnitude.

      --image-properties-outline-weight-unit <IMAGE_PROPERTIES_OUTLINE_WEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --image-properties-recolor-name <IMAGE_PROPERTIES_RECOLOR_NAME>
          The name of the recolor effect. The name is determined from the `recolor_stops` by matching the gradient against the colors in the page's current color scheme. This property is read-only. Choices: NONE, LIGHT1, LIGHT2, LIGHT3, LIGHT4, LIGHT5, LIGHT6, LIGHT7, LIGHT8, LIGHT9, LIGHT10, DARK1, DARK2, DARK3, DARK4, DARK5, DARK6, DARK7, DARK8, DARK9, DARK10, GRAYSCALE, NEGATIVE, SEPIA, CUSTOM.

      --image-properties-recolor-recolor-stops <JSON>
          The recolor effect is represented by a gradient, which is a list of color stops. The colors in the gradient will replace the corresponding colors at the same position in the color palette and apply to the image. This property is read-only.

      --image-properties-shadow-alignment <IMAGE_PROPERTIES_SHADOW_ALIGNMENT>
          The alignment point of the shadow, that sets the origin for translate, scale and skew of the shadow. This property is read-only. Choices: RECTANGLE_POSITION_UNSPECIFIED, TOP_LEFT, TOP_CENTER, TOP_RIGHT, LEFT_CENTER, CENTER, RIGHT_CENTER, BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT.

      --image-properties-shadow-alpha <IMAGE_PROPERTIES_SHADOW_ALPHA>
          The alpha of the shadow's color, from 0.0 to 1.0.

      --image-properties-shadow-blur-radius-magnitude <IMAGE_PROPERTIES_SHADOW_BLUR_RADIUS_MAGNITUDE>
          The magnitude.

      --image-properties-shadow-blur-radius-unit <IMAGE_PROPERTIES_SHADOW_BLUR_RADIUS_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --image-properties-shadow-color-theme-color <IMAGE_PROPERTIES_SHADOW_COLOR_THEME_COLOR>
          An opaque theme color. Choices: THEME_COLOR_TYPE_UNSPECIFIED, DARK1, LIGHT1, DARK2, LIGHT2, ACCENT1, ACCENT2, ACCENT3, ACCENT4, ACCENT5, ACCENT6, HYPERLINK, FOLLOWED_HYPERLINK, TEXT1, BACKGROUND1, TEXT2, BACKGROUND2.

      --image-properties-shadow-property-state <IMAGE_PROPERTIES_SHADOW_PROPERTY_STATE>
          The shadow property state. Updating the shadow on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no shadow on a page element, set this field to `NOT_RENDERED`. In this case, any other shadow fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --image-properties-shadow-rotate-with-shape[=<BOOL>]
          Whether the shadow should rotate with the shape. This property is read-only.

      --image-properties-shadow-transform-scale-x <IMAGE_PROPERTIES_SHADOW_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --image-properties-shadow-transform-scale-y <IMAGE_PROPERTIES_SHADOW_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --image-properties-shadow-transform-shear-x <IMAGE_PROPERTIES_SHADOW_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --image-properties-shadow-transform-shear-y <IMAGE_PROPERTIES_SHADOW_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --image-properties-shadow-transform-translate-x <IMAGE_PROPERTIES_SHADOW_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --image-properties-shadow-transform-translate-y <IMAGE_PROPERTIES_SHADOW_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --image-properties-shadow-transform-unit <IMAGE_PROPERTIES_SHADOW_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --image-properties-shadow-type <IMAGE_PROPERTIES_SHADOW_TYPE>
          The type of the shadow. This property is read-only. Choices: SHADOW_TYPE_UNSPECIFIED, OUTER.

      --image-properties-transparency <IMAGE_PROPERTIES_TRANSPARENCY>
          The transparency effect of the image. The value should be in the interval [0.0, 1.0], where 0 means no effect and 1 means completely transparent. This property is read-only.

      --object-id <OBJECT_ID>
          The object ID of the image the updates are applied to.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-line-category

```text
Updates the category of a line.

Usage: joog [OPTIONS] slides <ACCOUNT> update-line-category [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --line-category <LINE_CATEGORY>
          The line category to update to. The exact line type is determined based on the category to update to and how it's routed to connect to other page elements. Choices: LINE_CATEGORY_UNSPECIFIED, STRAIGHT, BENT, CURVED.

      --object-id <OBJECT_ID>
          The object ID of the line the update is applied to. Only a line with a category indicating it is a "connector" can be updated. The line may be rerouted after updating its category.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-line-properties

```text
Updates the properties of a Line.

Usage: joog [OPTIONS] slides <ACCOUNT> update-line-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `lineProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the line solid fill color, set `fields` to `"lineFill.solidFill.color"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --line-properties <JSON>
          The properties of the Line. When unset, these fields default to values that match the appearance of new lines created in the Slides editor.

      --line-properties-dash-style <LINE_PROPERTIES_DASH_STYLE>
          The dash style of the line. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH, DASH_DOT, LONG_DASH, LONG_DASH_DOT.

      --line-properties-end-arrow <LINE_PROPERTIES_END_ARROW>
          The style of the arrow at the end of the line. Choices: ARROW_STYLE_UNSPECIFIED, NONE, STEALTH_ARROW, FILL_ARROW, FILL_CIRCLE, FILL_SQUARE, FILL_DIAMOND, OPEN_ARROW, OPEN_CIRCLE, OPEN_SQUARE, OPEN_DIAMOND.

      --line-properties-end-connection-connected-object-id <LINE_PROPERTIES_END_CONNECTION_CONNECTED_OBJECT_ID>
          The object ID of the connected page element. Some page elements, such as groups, tables, and lines do not have connection sites and therefore cannot be connected to a connector line.

      --line-properties-end-connection-connection-site-index <LINE_PROPERTIES_END_CONNECTION_CONNECTION_SITE_INDEX>
          The index of the connection site on the connected page element. In most cases, it corresponds to the predefined connection site index from the ECMA-376 standard. More information on those connection sites can be found in both the description of the "cxn" attribute in section 20.1.9.9 and "Annex H. Example Predefined DrawingML Shape and Text Geometries" of "Office Open XML File Formats - Fundamentals and Markup Language Reference", part 1 of [ECMA-376 5th edition](https://ecma-international.org/publications-and-standards/standards/ecma-376/). The position of each connection site can also be viewed from Slides editor.

      --line-properties-line-fill-solid-fill-alpha <LINE_PROPERTIES_LINE_FILL_SOLID_FILL_ALPHA>
          The fraction of this `color` that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color.

      --line-properties-link-page-object-id <LINE_PROPERTIES_LINK_PAGE_OBJECT_ID>
          If set, indicates this is a link to the specific page in this presentation with this ID. A page with this ID may not exist.

      --line-properties-link-relative-link <LINE_PROPERTIES_LINK_RELATIVE_LINK>
          If set, indicates this is a link to a slide in this presentation, addressed by its position. Choices: RELATIVE_SLIDE_LINK_UNSPECIFIED, NEXT_SLIDE, PREVIOUS_SLIDE, FIRST_SLIDE, LAST_SLIDE.

      --line-properties-link-slide-index <LINE_PROPERTIES_LINK_SLIDE_INDEX>
          If set, indicates this is a link to the slide at this zero-based index in the presentation. There may not be a slide at this index.

      --line-properties-link-url <LINE_PROPERTIES_LINK_URL>
          If set, indicates this is a link to the external web page at this URL.

      --line-properties-start-arrow <LINE_PROPERTIES_START_ARROW>
          The style of the arrow at the beginning of the line. Choices: ARROW_STYLE_UNSPECIFIED, NONE, STEALTH_ARROW, FILL_ARROW, FILL_CIRCLE, FILL_SQUARE, FILL_DIAMOND, OPEN_ARROW, OPEN_CIRCLE, OPEN_SQUARE, OPEN_DIAMOND.

      --line-properties-start-connection-connected-object-id <LINE_PROPERTIES_START_CONNECTION_CONNECTED_OBJECT_ID>
          The object ID of the connected page element. Some page elements, such as groups, tables, and lines do not have connection sites and therefore cannot be connected to a connector line.

      --line-properties-start-connection-connection-site-index <LINE_PROPERTIES_START_CONNECTION_CONNECTION_SITE_INDEX>
          The index of the connection site on the connected page element. In most cases, it corresponds to the predefined connection site index from the ECMA-376 standard. More information on those connection sites can be found in both the description of the "cxn" attribute in section 20.1.9.9 and "Annex H. Example Predefined DrawingML Shape and Text Geometries" of "Office Open XML File Formats - Fundamentals and Markup Language Reference", part 1 of [ECMA-376 5th edition](https://ecma-international.org/publications-and-standards/standards/ecma-376/). The position of each connection site can also be viewed from Slides editor.

      --line-properties-weight-magnitude <LINE_PROPERTIES_WEIGHT_MAGNITUDE>
          The magnitude.

      --line-properties-weight-unit <LINE_PROPERTIES_WEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --object-id <OBJECT_ID>
          The object ID of the line the update is applied to.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-page-element-alt-text

```text
Updates the alt text title and/or description of a page element.

Usage: joog [OPTIONS] slides <ACCOUNT> update-page-element-alt-text [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --description <DESCRIPTION>
          The updated alt text description of the page element. If unset the existing value will be maintained. The description is exposed to screen readers and other accessibility interfaces. Only use human readable values related to the content of the page element.

      --verbose
          Include advanced parameters in help

      --title <TITLE>
          The updated alt text title of the page element. If unset the existing value will be maintained. The title is exposed to screen readers and other accessibility interfaces. Only use human readable values related to the content of the page element.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the page element the updates are applied to.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-page-element-transform

```text
Updates the transform of a page element.

Usage: joog [OPTIONS] slides <ACCOUNT> update-page-element-transform [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --apply-mode <APPLY_MODE>
          The apply mode of the transform update. Choices: APPLY_MODE_UNSPECIFIED, RELATIVE, ABSOLUTE.

      --object-id <OBJECT_ID>
          The object ID of the page element to update.

      --transform <JSON>
          AffineTransform uses a 3x3 matrix with an implied last row of [ 0 0 1 ] to transform source coordinates (x,y) into destination coordinates (x', y') according to: x' x = shear_y scale_y translate_y 1 [ 1 ] After transformation, x' = scale_x * x + shear_x * y + translate_x; y' = scale_y * y + shear_y * x + translate_y; This message is therefore composed of these six matrix elements.

      --transform-scale-x <TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --transform-scale-y <TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --transform-shear-x <TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --transform-shear-y <TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --transform-translate-x <TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --transform-translate-y <TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --transform-unit <TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-page-elements-z-order

```text
Updates the Z-order of page elements.

Usage: joog [OPTIONS] slides <ACCOUNT> update-page-elements-z-order [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --operation <OPERATION>
          The Z-order operation to apply on the page elements. When applying the operation on multiple page elements, the relative Z-orders within these page elements before the operation is maintained. Choices: Z_ORDER_OPERATION_UNSPECIFIED, BRING_TO_FRONT, BRING_FORWARD, SEND_BACKWARD, SEND_TO_BACK.

      --page-element-object-ids <PAGE_ELEMENT_OBJECT_IDS>...
          The object IDs of the page elements to update. All the page elements must be on the same page and must not be grouped.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-page-properties

```text
Updates the properties of a Page.

Usage: joog [OPTIONS] slides <ACCOUNT> update-page-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `pageProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the page background solid fill color, set `fields` to `"pageBackgroundFill.solidFill.color"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the page the update is applied to.

      --page-properties <JSON>
          The properties of the Page. The page will inherit properties from the parent page. Depending on the page type the hierarchy is defined in either SlideProperties or LayoutProperties.

      --page-properties-color-scheme-colors <JSON>
          The ThemeColorType and corresponding concrete color pairs.

      --page-properties-page-background-fill-property-state <PAGE_PROPERTIES_PAGE_BACKGROUND_FILL_PROPERTY_STATE>
          The background fill property state. Updating the fill on a page will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a page, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --page-properties-page-background-fill-solid-fill-alpha <PAGE_PROPERTIES_PAGE_BACKGROUND_FILL_SOLID_FILL_ALPHA>
          The fraction of this `color` that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color.

      --page-properties-page-background-fill-stretched-picture-fill-content-url <PAGE_PROPERTIES_PAGE_BACKGROUND_FILL_STRETCHED_PICTURE_FILL_CONTENT_URL>
          Reading the content_url: An URL to a picture with a default lifetime of 30 minutes. This URL is tagged with the account of the requester. Anyone with the URL effectively accesses the picture as the original requester. Access to the picture may be lost if the presentation's sharing settings change. Writing the content_url: The picture is fetched once at insertion time and a copy is stored for display inside the presentation. Pictures must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URL can be at most 2 kB in length.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-paragraph-style

```text
Updates the styling of paragraphs within a Shape or Table.

Usage: joog [OPTIONS] slides <ACCOUNT> update-paragraph-style [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example, to update the paragraph alignment, set `fields` to `"alignment"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

      --text-range <JSON>
          Specifies a contiguous range of an indexed collection, such as characters in text.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --object-id <OBJECT_ID>
          The object ID of the shape or table with the text to be styled.

      --style <JSON>
          Styles that apply to a whole paragraph. If this text is contained in a shape with a parent placeholder, then these paragraph styles may be inherited from the parent. Which paragraph styles are inherited depend on the nesting level of lists: * A paragraph not in a list will inherit its paragraph style from the paragraph at the 0 nesting level of the list inside the parent placeholder. * A paragraph in a list will inherit its paragraph style from the paragraph at its corresponding nesting level of the list inside the parent placeholder. Inherited paragraph styles are represented as unset fields in this message.

      --style-alignment <STYLE_ALIGNMENT>
          The text alignment for this paragraph. Choices: ALIGNMENT_UNSPECIFIED, START, CENTER, END, JUSTIFIED.

      --style-direction <STYLE_DIRECTION>
          The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since text direction is not inherited. Choices: TEXT_DIRECTION_UNSPECIFIED, LEFT_TO_RIGHT, RIGHT_TO_LEFT.

      --style-indent-end-magnitude <STYLE_INDENT_END_MAGNITUDE>
          The magnitude.

      --style-indent-end-unit <STYLE_INDENT_END_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --style-indent-first-line-magnitude <STYLE_INDENT_FIRST_LINE_MAGNITUDE>
          The magnitude.

      --style-indent-first-line-unit <STYLE_INDENT_FIRST_LINE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --style-indent-start-magnitude <STYLE_INDENT_START_MAGNITUDE>
          The magnitude.

      --style-indent-start-unit <STYLE_INDENT_START_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --style-line-spacing <STYLE_LINE_SPACING>
          The amount of space between lines, as a percentage of normal, where normal is represented as 100.0. If unset, the value is inherited from the parent.

      --style-space-above-magnitude <STYLE_SPACE_ABOVE_MAGNITUDE>
          The magnitude.

      --style-space-above-unit <STYLE_SPACE_ABOVE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --style-space-below-magnitude <STYLE_SPACE_BELOW_MAGNITUDE>
          The magnitude.

      --style-space-below-unit <STYLE_SPACE_BELOW_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --style-spacing-mode <STYLE_SPACING_MODE>
          The spacing mode for the paragraph. Choices: SPACING_MODE_UNSPECIFIED, NEVER_COLLAPSE, COLLAPSE_LISTS.

      --text-range-end-index <TEXT_RANGE_END_INDEX>
          The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges.

      --text-range-start-index <TEXT_RANGE_START_INDEX>
          The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges.

      --text-range-type <TEXT_RANGE_TYPE>
          The type of range. Choices: RANGE_TYPE_UNSPECIFIED, FIXED_RANGE, FROM_START_INDEX, ALL.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-shape-properties

```text
Updates the properties of a Shape.

Usage: joog [OPTIONS] slides <ACCOUNT> update-shape-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `shapeProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the shape background solid fill color, set `fields` to `"shapeBackgroundFill.solidFill.color"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the shape the updates are applied to.

      --shape-properties <JSON>
          The properties of a Shape. If the shape is a placeholder shape as determined by the placeholder field, then these properties may be inherited from a parent placeholder shape. Determining the rendered value of the property depends on the corresponding property_state field value. Any text autofit settings on the shape are automatically deactivated by requests that can impact how text fits in the shape.

      --shape-properties-autofit-autofit-type <SHAPE_PROPERTIES_AUTOFIT_AUTOFIT_TYPE>
          The autofit type of the shape. If the autofit type is AUTOFIT_TYPE_UNSPECIFIED, the autofit type is inherited from a parent placeholder if it exists. The field is automatically set to NONE if a request is made that might affect text fitting within its bounding text box. In this case, the font_scale is applied to the font_size and the line_spacing_reduction is applied to the line_spacing. Both properties are also reset to default values. Choices: AUTOFIT_TYPE_UNSPECIFIED, NONE, TEXT_AUTOFIT, SHAPE_AUTOFIT.

      --shape-properties-autofit-font-scale <SHAPE_PROPERTIES_AUTOFIT_FONT_SCALE>
          The font scale applied to the shape. For shapes with autofit_type NONE or SHAPE_AUTOFIT, this value is the default value of 1. For TEXT_AUTOFIT, this value multiplied by the font_size gives the font size that's rendered in the editor. This property is read-only.

      --shape-properties-autofit-line-spacing-reduction <SHAPE_PROPERTIES_AUTOFIT_LINE_SPACING_REDUCTION>
          The line spacing reduction applied to the shape. For shapes with autofit_type NONE or SHAPE_AUTOFIT, this value is the default value of 0. For TEXT_AUTOFIT, this value subtracted from the line_spacing gives the line spacing that's rendered in the editor. This property is read-only.

      --shape-properties-content-alignment <SHAPE_PROPERTIES_CONTENT_ALIGNMENT>
          The alignment of the content in the shape. If unspecified, the alignment is inherited from a parent placeholder if it exists. If the shape has no parent, the default alignment matches the alignment for new shapes created in the Slides editor. Choices: CONTENT_ALIGNMENT_UNSPECIFIED, CONTENT_ALIGNMENT_UNSUPPORTED, TOP, MIDDLE, BOTTOM.

      --shape-properties-link-page-object-id <SHAPE_PROPERTIES_LINK_PAGE_OBJECT_ID>
          If set, indicates this is a link to the specific page in this presentation with this ID. A page with this ID may not exist.

      --shape-properties-link-relative-link <SHAPE_PROPERTIES_LINK_RELATIVE_LINK>
          If set, indicates this is a link to a slide in this presentation, addressed by its position. Choices: RELATIVE_SLIDE_LINK_UNSPECIFIED, NEXT_SLIDE, PREVIOUS_SLIDE, FIRST_SLIDE, LAST_SLIDE.

      --shape-properties-link-slide-index <SHAPE_PROPERTIES_LINK_SLIDE_INDEX>
          If set, indicates this is a link to the slide at this zero-based index in the presentation. There may not be a slide at this index.

      --shape-properties-link-url <SHAPE_PROPERTIES_LINK_URL>
          If set, indicates this is a link to the external web page at this URL.

      --shape-properties-outline-dash-style <SHAPE_PROPERTIES_OUTLINE_DASH_STYLE>
          The dash style of the outline. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH, DASH_DOT, LONG_DASH, LONG_DASH_DOT.

      --shape-properties-outline-property-state <SHAPE_PROPERTIES_OUTLINE_PROPERTY_STATE>
          The outline property state. Updating the outline on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no outline on a page element, set this field to `NOT_RENDERED`. In this case, any other outline fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --shape-properties-outline-weight-magnitude <SHAPE_PROPERTIES_OUTLINE_WEIGHT_MAGNITUDE>
          The magnitude.

      --shape-properties-outline-weight-unit <SHAPE_PROPERTIES_OUTLINE_WEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --shape-properties-shadow-alignment <SHAPE_PROPERTIES_SHADOW_ALIGNMENT>
          The alignment point of the shadow, that sets the origin for translate, scale and skew of the shadow. This property is read-only. Choices: RECTANGLE_POSITION_UNSPECIFIED, TOP_LEFT, TOP_CENTER, TOP_RIGHT, LEFT_CENTER, CENTER, RIGHT_CENTER, BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT.

      --shape-properties-shadow-alpha <SHAPE_PROPERTIES_SHADOW_ALPHA>
          The alpha of the shadow's color, from 0.0 to 1.0.

      --shape-properties-shadow-blur-radius-magnitude <SHAPE_PROPERTIES_SHADOW_BLUR_RADIUS_MAGNITUDE>
          The magnitude.

      --shape-properties-shadow-blur-radius-unit <SHAPE_PROPERTIES_SHADOW_BLUR_RADIUS_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --shape-properties-shadow-color-theme-color <SHAPE_PROPERTIES_SHADOW_COLOR_THEME_COLOR>
          An opaque theme color. Choices: THEME_COLOR_TYPE_UNSPECIFIED, DARK1, LIGHT1, DARK2, LIGHT2, ACCENT1, ACCENT2, ACCENT3, ACCENT4, ACCENT5, ACCENT6, HYPERLINK, FOLLOWED_HYPERLINK, TEXT1, BACKGROUND1, TEXT2, BACKGROUND2.

      --shape-properties-shadow-property-state <SHAPE_PROPERTIES_SHADOW_PROPERTY_STATE>
          The shadow property state. Updating the shadow on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no shadow on a page element, set this field to `NOT_RENDERED`. In this case, any other shadow fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --shape-properties-shadow-rotate-with-shape[=<BOOL>]
          Whether the shadow should rotate with the shape. This property is read-only.

      --shape-properties-shadow-transform-scale-x <SHAPE_PROPERTIES_SHADOW_TRANSFORM_SCALE_X>
          The X coordinate scaling element.

      --shape-properties-shadow-transform-scale-y <SHAPE_PROPERTIES_SHADOW_TRANSFORM_SCALE_Y>
          The Y coordinate scaling element.

      --shape-properties-shadow-transform-shear-x <SHAPE_PROPERTIES_SHADOW_TRANSFORM_SHEAR_X>
          The X coordinate shearing element.

      --shape-properties-shadow-transform-shear-y <SHAPE_PROPERTIES_SHADOW_TRANSFORM_SHEAR_Y>
          The Y coordinate shearing element.

      --shape-properties-shadow-transform-translate-x <SHAPE_PROPERTIES_SHADOW_TRANSFORM_TRANSLATE_X>
          The X coordinate translation element.

      --shape-properties-shadow-transform-translate-y <SHAPE_PROPERTIES_SHADOW_TRANSFORM_TRANSLATE_Y>
          The Y coordinate translation element.

      --shape-properties-shadow-transform-unit <SHAPE_PROPERTIES_SHADOW_TRANSFORM_UNIT>
          The units for translate elements. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --shape-properties-shadow-type <SHAPE_PROPERTIES_SHADOW_TYPE>
          The type of the shadow. This property is read-only. Choices: SHADOW_TYPE_UNSPECIFIED, OUTER.

      --shape-properties-shape-background-fill-property-state <SHAPE_PROPERTIES_SHAPE_BACKGROUND_FILL_PROPERTY_STATE>
          The background fill property state. Updating the fill on a shape will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a shape, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --shape-properties-shape-background-fill-solid-fill-alpha <SHAPE_PROPERTIES_SHAPE_BACKGROUND_FILL_SOLID_FILL_ALPHA>
          The fraction of this `color` that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-slide-properties

```text
Updates the properties of a Slide

Usage: joog [OPTIONS] slides <ACCOUNT> update-slide-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root 'slideProperties' is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update whether a slide is skipped, set `fields` to `"isSkipped"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the slide the update is applied to.

      --slide-properties <JSON>
          The properties of Page that are only relevant for pages with page_type SLIDE.

      --slide-properties-is-skipped[=<BOOL>]
          Whether the slide is skipped in the presentation mode. Defaults to false.

      --slide-properties-layout-object-id <SLIDE_PROPERTIES_LAYOUT_OBJECT_ID>
          The object ID of the layout that this slide is based on. This property is read-only.

      --slide-properties-master-object-id <SLIDE_PROPERTIES_MASTER_OBJECT_ID>
          The object ID of the master that this slide is based on. This property is read-only.

      --slide-properties-notes-page-layout-properties-display-name <SLIDE_PROPERTIES_NOTES_PAGE_LAYOUT_PROPERTIES_DISPLAY_NAME>
          The human-readable name of the layout.

      --slide-properties-notes-page-layout-properties-master-object-id <SLIDE_PROPERTIES_NOTES_PAGE_LAYOUT_PROPERTIES_MASTER_OBJECT_ID>
          The object ID of the master that this layout is based on.

      --slide-properties-notes-page-layout-properties-name <SLIDE_PROPERTIES_NOTES_PAGE_LAYOUT_PROPERTIES_NAME>
          The name of the layout.

      --slide-properties-notes-page-master-properties-display-name <SLIDE_PROPERTIES_NOTES_PAGE_MASTER_PROPERTIES_DISPLAY_NAME>
          The human-readable name of the master.

      --slide-properties-notes-page-notes-properties-speaker-notes-object-id <SLIDE_PROPERTIES_NOTES_PAGE_NOTES_PROPERTIES_SPEAKER_NOTES_OBJECT_ID>
          The object ID of the shape on this notes page that contains the speaker notes for the corresponding slide. The actual shape may not always exist on the notes page. Inserting text using this object ID will automatically create the shape. In this case, the actual shape may have different object ID. The `GetPresentation` or `GetPage` action will always return the latest object ID.

      --slide-properties-notes-page-object-id <SLIDE_PROPERTIES_NOTES_PAGE_OBJECT_ID>
          The object ID for this page. Object IDs used by Page and PageElement share the same namespace.

      --slide-properties-notes-page-page-elements <JSON>
          The page elements rendered on the page.

      --slide-properties-notes-page-page-type <SLIDE_PROPERTIES_NOTES_PAGE_PAGE_TYPE>
          The type of the page. Choices: SLIDE, MASTER, LAYOUT, NOTES, NOTES_MASTER.

      --slide-properties-notes-page-slide-properties-is-skipped[=<BOOL>]
          Whether the slide is skipped in the presentation mode. Defaults to false.

      --slide-properties-notes-page-slide-properties-layout-object-id <SLIDE_PROPERTIES_NOTES_PAGE_SLIDE_PROPERTIES_LAYOUT_OBJECT_ID>
          The object ID of the layout that this slide is based on. This property is read-only.

      --slide-properties-notes-page-slide-properties-master-object-id <SLIDE_PROPERTIES_NOTES_PAGE_SLIDE_PROPERTIES_MASTER_OBJECT_ID>
          The object ID of the master that this slide is based on. This property is read-only.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-slides-position

```text
Updates the position of a set of slides in the presentation.

Usage: joog [OPTIONS] slides <ACCOUNT> update-slides-position [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --insertion-index <INSERTION_INDEX>
          The index where the slides should be inserted, based on the slide arrangement before the move takes place. Must be between zero and the number of slides in the presentation, inclusive.

      --slide-object-ids <SLIDE_OBJECT_IDS>...
          The IDs of the slides in the presentation that should be moved. The slides in this list must be in existing presentation order, without duplicates.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-table-border-properties

```text
Updates the properties of the table borders in a Table.

Usage: joog [OPTIONS] slides <ACCOUNT> update-table-border-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tableBorderProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the table border solid fill color, set `fields` to `"tableBorderFill.solidFill.color"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --border-position <BORDER_POSITION>
          The border position in the table range the updates should apply to. If a border position is not specified, the updates will apply to all borders in the table range. Choices: ALL, BOTTOM, INNER, INNER_HORIZONTAL, INNER_VERTICAL, LEFT, OUTER, RIGHT, TOP.

      --object-id <OBJECT_ID>
          The object ID of the table.

      --table-border-properties <JSON>
          The border styling properties of the TableBorderCell.

      --table-border-properties-dash-style <TABLE_BORDER_PROPERTIES_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH, DASH_DOT, LONG_DASH, LONG_DASH_DOT.

      --table-border-properties-table-border-fill-solid-fill-alpha <TABLE_BORDER_PROPERTIES_TABLE_BORDER_FILL_SOLID_FILL_ALPHA>
          The fraction of this `color` that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color.

      --table-border-properties-weight-magnitude <TABLE_BORDER_PROPERTIES_WEIGHT_MAGNITUDE>
          The magnitude.

      --table-border-properties-weight-unit <TABLE_BORDER_PROPERTIES_WEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --table-range <JSON>
          A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with location = (0, 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]

      --table-range-column-span <TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-range-location-column-index <TABLE_RANGE_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --table-range-location-row-index <TABLE_RANGE_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-range-row-span <TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-table-cell-properties

```text
Updates the properties of a TableCell.

Usage: joog [OPTIONS] slides <ACCOUNT> update-table-cell-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tableCellProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the table cell background solid fill color, set `fields` to `"tableCellBackgroundFill.solidFill.color"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the table.

      --table-cell-properties <JSON>
          The properties of the TableCell.

      --table-cell-properties-content-alignment <TABLE_CELL_PROPERTIES_CONTENT_ALIGNMENT>
          The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Slides editor. Choices: CONTENT_ALIGNMENT_UNSPECIFIED, CONTENT_ALIGNMENT_UNSUPPORTED, TOP, MIDDLE, BOTTOM.

      --table-cell-properties-table-cell-background-fill-property-state <TABLE_CELL_PROPERTIES_TABLE_CELL_BACKGROUND_FILL_PROPERTY_STATE>
          The background fill property state. Updating the fill on a table cell will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a table cell, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --table-cell-properties-table-cell-background-fill-solid-fill-alpha <TABLE_CELL_PROPERTIES_TABLE_CELL_BACKGROUND_FILL_SOLID_FILL_ALPHA>
          The fraction of this `color` that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color.

      --table-range <JSON>
          A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with location = (0, 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]

      --table-range-column-span <TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-range-location-column-index <TABLE_RANGE_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --table-range-location-row-index <TABLE_RANGE_LOCATION_ROW_INDEX>
          The 0-based row index.

      --table-range-row-span <TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-table-column-properties

```text
Updates the properties of a Table column.

Usage: joog [OPTIONS] slides <ACCOUNT> update-table-column-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tableColumnProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the column width, set `fields` to `"column_width"`. If '"column_width"' is included in the field mask but the property is left unset, the column width will default to 406,400 EMU (32 points).

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --column-indices <COLUMN_INDICES>...
          The list of zero-based indices specifying which columns to update. If no indices are provided, all columns in the table will be updated.

      --object-id <OBJECT_ID>
          The object ID of the table.

      --table-column-properties <JSON>
          Properties of each column in a table.

      --table-column-properties-column-width-magnitude <TABLE_COLUMN_PROPERTIES_COLUMN_WIDTH_MAGNITUDE>
          The magnitude.

      --table-column-properties-column-width-unit <TABLE_COLUMN_PROPERTIES_COLUMN_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-table-row-properties

```text
Updates the properties of a Table row.

Usage: joog [OPTIONS] slides <ACCOUNT> update-table-row-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tableRowProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the minimum row height, set `fields` to `"min_row_height"`. If '"min_row_height"' is included in the field mask but the property is left unset, the minimum row height will default to 0.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the table.

      --row-indices <ROW_INDICES>...
          The list of zero-based indices specifying which rows to update. If no indices are provided, all rows in the table will be updated.

      --table-row-properties <JSON>
          Properties of each row in a table.

      --table-row-properties-min-row-height-magnitude <TABLE_ROW_PROPERTIES_MIN_ROW_HEIGHT_MAGNITUDE>
          The magnitude.

      --table-row-properties-min-row-height-unit <TABLE_ROW_PROPERTIES_MIN_ROW_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-text-style

```text
Updates the styling of text within a Shape or Table.

Usage: joog [OPTIONS] slides <ACCOUNT> update-text-style [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example, to update the text style to bold, set `fields` to `"bold"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

      --style-bold[=<BOOL>]
          Whether or not the text is rendered as bold.

      --style-italic[=<BOOL>]
          Whether or not the text is italicized.

      --style-underline[=<BOOL>]
          Whether or not the text is underlined.

      --text-range <JSON>
          Specifies a contiguous range of an indexed collection, such as characters in text.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --cell-location <JSON>
          A location of a single table cell within a table.

      --cell-location-column-index <CELL_LOCATION_COLUMN_INDEX>
          The 0-based column index.

      --cell-location-row-index <CELL_LOCATION_ROW_INDEX>
          The 0-based row index.

      --object-id <OBJECT_ID>
          The object ID of the shape or table with the text to be styled.

      --style <JSON>
          Represents the styling that can be applied to a TextRun. If this text is contained in a shape with a parent placeholder, then these text styles may be inherited from the parent. Which text styles are inherited depend on the nesting level of lists: * A text run in a paragraph that is not in a list will inherit its text style from the the newline character in the paragraph at the 0 nesting level of the list inside the parent placeholder. * A text run in a paragraph that is in a list will inherit its text style from the newline character in the paragraph at its corresponding nesting level of the list inside the parent placeholder. Inherited text styles are represented as unset fields in this message. If text is contained in a shape without a parent placeholder, unsetting these fields will revert the style to a value matching the defaults in the Slides editor.

      --style-background-color-opaque-color-theme-color <STYLE_BACKGROUND_COLOR_OPAQUE_COLOR_THEME_COLOR>
          An opaque theme color. Choices: THEME_COLOR_TYPE_UNSPECIFIED, DARK1, LIGHT1, DARK2, LIGHT2, ACCENT1, ACCENT2, ACCENT3, ACCENT4, ACCENT5, ACCENT6, HYPERLINK, FOLLOWED_HYPERLINK, TEXT1, BACKGROUND1, TEXT2, BACKGROUND2.

      --style-baseline-offset <STYLE_BASELINE_OFFSET>
          The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. The `font_size` itself is not affected by changes in this field. Choices: BASELINE_OFFSET_UNSPECIFIED, NONE, SUPERSCRIPT, SUBSCRIPT.

      --style-font-family <STYLE_FONT_FAMILY>
          The font family of the text. The font family can be any font from the Font menu in Slides or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`. Some fonts can affect the weight of the text. If an update request specifies values for both `font_family` and `bold`, the explicitly-set `bold` value is used.

      --style-font-size-magnitude <STYLE_FONT_SIZE_MAGNITUDE>
          The magnitude.

      --style-font-size-unit <STYLE_FONT_SIZE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --style-foreground-color-opaque-color-theme-color <STYLE_FOREGROUND_COLOR_OPAQUE_COLOR_THEME_COLOR>
          An opaque theme color. Choices: THEME_COLOR_TYPE_UNSPECIFIED, DARK1, LIGHT1, DARK2, LIGHT2, ACCENT1, ACCENT2, ACCENT3, ACCENT4, ACCENT5, ACCENT6, HYPERLINK, FOLLOWED_HYPERLINK, TEXT1, BACKGROUND1, TEXT2, BACKGROUND2.

      --style-link-page-object-id <STYLE_LINK_PAGE_OBJECT_ID>
          If set, indicates this is a link to the specific page in this presentation with this ID. A page with this ID may not exist.

      --style-link-relative-link <STYLE_LINK_RELATIVE_LINK>
          If set, indicates this is a link to a slide in this presentation, addressed by its position. Choices: RELATIVE_SLIDE_LINK_UNSPECIFIED, NEXT_SLIDE, PREVIOUS_SLIDE, FIRST_SLIDE, LAST_SLIDE.

      --style-link-slide-index <STYLE_LINK_SLIDE_INDEX>
          If set, indicates this is a link to the slide at this zero-based index in the presentation. There may not be a slide at this index.

      --style-link-url <STYLE_LINK_URL>
          If set, indicates this is a link to the external web page at this URL.

      --style-small-caps[=<BOOL>]
          Whether or not the text is in small capital letters.

      --style-strikethrough[=<BOOL>]
          Whether or not the text is struck through.

      --style-weighted-font-family-font-family <STYLE_WEIGHTED_FONT_FAMILY_FONT_FAMILY>
          The font family of the text. The font family can be any font from the Font menu in Slides or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`.

      --style-weighted-font-family-weight <STYLE_WEIGHTED_FONT_FAMILY_WEIGHT>
          The rendered weight of the text. This field can have any value that is a multiple of `100` between `100` and `900`, inclusive. This range corresponds to the numerical values described in the CSS 2.1 Specification, [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with non-numerical values disallowed. Weights greater than or equal to `700` are considered bold, and weights less than `700`are not bold. The default value is `400` ("normal").

      --text-range-end-index <TEXT_RANGE_END_INDEX>
          The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges.

      --text-range-start-index <TEXT_RANGE_START_INDEX>
          The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges.

      --text-range-type <TEXT_RANGE_TYPE>
          The type of range. Choices: RANGE_TYPE_UNSPECIFIED, FIXED_RANGE, FROM_START_INDEX, ALL.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-video-properties

```text
Updates the properties of a Video.

Usage: joog [OPTIONS] slides <ACCOUNT> update-video-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --presentation-id <PRESENTATION_ID>
          The presentation to apply the updates to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `videoProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the video outline color, set `fields` to `"outline.outlineFill.solidFill.color"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --object-id <OBJECT_ID>
          The object ID of the video the updates are applied to.

      --video-properties <JSON>
          The properties of the Video.

      --video-properties-auto-play[=<BOOL>]
          Whether to enable video autoplay when the page is displayed in present mode. Defaults to false.

      --video-properties-end <VIDEO_PROPERTIES_END>
          The time at which to end playback, measured in seconds from the beginning of the video. If set, the end time should be after the start time. If not set or if you set this to a value that exceeds the video's length, the video will be played until its end.

      --video-properties-mute[=<BOOL>]
          Whether to mute the audio during video playback. Defaults to false.

      --video-properties-outline-dash-style <VIDEO_PROPERTIES_OUTLINE_DASH_STYLE>
          The dash style of the outline. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH, DASH_DOT, LONG_DASH, LONG_DASH_DOT.

      --video-properties-outline-property-state <VIDEO_PROPERTIES_OUTLINE_PROPERTY_STATE>
          The outline property state. Updating the outline on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no outline on a page element, set this field to `NOT_RENDERED`. In this case, any other outline fields set in the same request will be ignored. Choices: RENDERED, NOT_RENDERED, INHERIT.

      --video-properties-outline-weight-magnitude <VIDEO_PROPERTIES_OUTLINE_WEIGHT_MAGNITUDE>
          The magnitude.

      --video-properties-outline-weight-unit <VIDEO_PROPERTIES_OUTLINE_WEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, EMU, PT.

      --video-properties-start <VIDEO_PROPERTIES_START>
          The time at which to start playback, measured in seconds from the beginning of the video. If set, the start time should be before the end time. If you set this to a value that exceeds the video's length in seconds, the video will be played from the last second. If not set, the video will be played from the beginning.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
