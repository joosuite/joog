# Google Docs methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `accept-suggestion` | Accepts a suggestion. |
| `add-comment-reply` | Adds a reply to a CommentThread or SuggestionThread. |
| `add-document-tab` | Adds a document tab. |
| `batch-update` | Applies one or more updates to the document |
| `create` | Creates a blank document using the title given in the request |
| `create-dropdown-definition` | Creates a DropdownDefinition. |
| `create-footer` | Creates a footer. |
| `create-footnote` | Creates a footnote. |
| `create-header` | Creates a header. |
| `create-named-range` | Creates a named range. |
| `create-paragraph-bullets` | Creates bullets for paragraphs. |
| `delete-comment` | Deletes a CommentThread. |
| `delete-comment-reply` | Deletes a reply Post from a CommentThread or SuggestionThread. |
| `delete-content-range` | Deletes content from the document. |
| `delete-dropdown-definition` | Deletes a DropdownDefinition. |
| `delete-footer` | Deletes a footer from the document. |
| `delete-header` | Deletes a header from the document. |
| `delete-named-range` | Deletes a named range. |
| `delete-paragraph-bullets` | Deletes bullets from paragraphs. |
| `delete-positioned-object` | Deletes a positioned object from the document. |
| `delete-suggestion` | Deletes a suggestion. |
| `delete-tab` | Deletes a document tab. |
| `delete-table-column` | Deletes a column from a table. |
| `delete-table-row` | Deletes a row from a table. |
| `get` | Gets the latest version of the specified document. |
| `insert-comment` | Inserts a CommentThread into the document. |
| `insert-date` | Inserts a date. |
| `insert-dropdown` | Inserts a Dropdown at the specified location. |
| `insert-inline-image` | Inserts an inline image at the specified location. |
| `insert-page-break` | Inserts a page break at the specified location. |
| `insert-person` | Inserts a person mention. |
| `insert-rich-link` | Insert a rich link. |
| `insert-section-break` | Inserts a section break at the specified location. |
| `insert-table` | Inserts a table at the specified location. |
| `insert-table-column` | Inserts an empty column into a table. |
| `insert-table-row` | Inserts an empty row into a table. |
| `insert-text` | Inserts text at the specified location. |
| `merge-table-cells` | Merges cells in a table. |
| `pin-table-header-rows` | Updates the number of pinned header rows in a table. |
| `reject-suggestion` | Rejects a suggestion. |
| `replace-all-text` | Replaces all instances of the specified text. |
| `replace-image` | Replaces an image in the document. |
| `replace-named-range-content` | Replaces the content in a named range. |
| `unmerge-table-cells` | Unmerges cells in a table. |
| `update-comment-post` | Updates an existing post (head post or reply) of a CommentThread or SuggestionThread. |
| `update-document-style` | Updates the style of the document. |
| `update-document-tab-properties` | Updates the properties of a document tab. |
| `update-dropdown-definition-properties` | Updates the properties of a DropdownDefinition. |
| `update-dropdown-properties` | Updates the properties of a Dropdown. |
| `update-named-style` | Updates a named style. |
| `update-paragraph-style` | Updates the paragraph style at the specified range. |
| `update-section-style` | Updates the section style of the specified range. |
| `update-table-cell-style` | Updates the style of table cells. |
| `update-table-column-properties` | Updates the properties of columns in a table. |
| `update-table-row-style` | Updates the row style in a table. |
| `update-text-style` | Updates the text style at the specified range. |

## accept-suggestion

```text
Accepts a suggestion.

Usage: joog [OPTIONS] docs <ACCOUNT> accept-suggestion [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --suggestion-id <SUGGESTION_ID>
          The ID of the suggestion.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## add-comment-reply

```text
Adds a reply to a CommentThread or SuggestionThread.

Usage: joog [OPTIONS] docs <ACCOUNT> add-comment-reply [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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
          Represents a single post in a comment or suggestion thread.

      --post-assignee-email <POST_ASSIGNEE_EMAIL>
          Optional. The email of the user who is being newly assigned to the thread as part of this post. Returns a 400 bad request error if: - The parent thread is a CommentThread whose headPost does not have an assignee. - The parent thread is a SuggestionThread. - commentAction is specified as `RESOLVE` or `REOPEN`. - `assigneeEmail` exceeds 2048 UTF-8 code units.

      --post-author-anonymous[=<BOOL>]
          Whether the user is anonymous.

      --post-author-display-name <POST_AUTHOR_DISPLAY_NAME>
          The display name of the user. May be absent if the author is anonymous.

      --post-author-me[=<BOOL>]
          Whether the user is the authenticated user making the request.

      --post-author-user <POST_AUTHOR_USER>
          The resource name of the post author user, which can also be used to identify the user in the [Google People API](https://developers.google.com/people/api/rest/v1/people). Format: `users/{user}`. Will not be populated if the anonymous field is `true` or if the post is from an imported document.

      --post-comment-action <POST_COMMENT_ACTION>
          Optional. The action type for comment posts. Choices: COMMENT_ACTION_TYPE_UNSPECIFIED, NO_COMMENT_ACTION_CHANGE, RESOLVE, REOPEN.

      --post-content <POST_CONTENT>
          The content of the post. Required to be non-empty if commentAction is not `RESOLVE` or `REOPEN`. This text content will be handled similarly to comments created in the Docs editor. It will have similar behaviors for formatting, notifications, etc. May not exceed 2048 UTF-8 code units.

      --suggestion-id <SUGGESTION_ID>
          The ID of the SuggestionThread to add the reply to.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## add-document-tab

```text
Adds a document tab.

Usage: joog [OPTIONS] docs <ACCOUNT> add-document-tab [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --tab-properties <JSON>
          Properties of a tab.

      --tab-properties-icon-emoji <TAB_PROPERTIES_ICON_EMOJI>
          Optional. The emoji icon displayed with the tab. A valid emoji icon is represented by a non-empty Unicode string. Any set of characters that don't represent a single emoji is invalid. If an emoji is invalid, a 400 bad request error is returned. If this value is unset or empty, the tab will display the default tab icon.

      --tab-properties-index <TAB_PROPERTIES_INDEX>
          The zero-based index of the tab within the parent.

      --tab-properties-parent-tab-id <TAB_PROPERTIES_PARENT_TAB_ID>
          Optional. The ID of the parent tab. Empty when the current tab is a root-level tab, which means it doesn't have any parents.

      --tab-properties-tab-id <TAB_PROPERTIES_TAB_ID>
          The immutable ID of the tab.

      --tab-properties-title <TAB_PROPERTIES_TITLE>
          The user-visible name of the tab.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## batch-update

```text
Applies one or more updates to the document. Each request is validated before being applied. If any request is not valid, then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. Other requests do not need to return information; these each return an empty reply. The order of replies matches that of the requests. For example, suppose you call batchUpdate with four updates, and only the third one returns information. The response would have two empty replies, the reply to the third request, and another empty reply, in that order. Because other users may be editing the document, the document might not exactly reflect your changes: your changes may be altered with respect to collaborator changes. If there are no collaborators, the document should reflect your changes. In any case, the updates in your request are guaranteed to be applied together atomically.

Usage: joog [OPTIONS] docs <ACCOUNT> batch-update [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --requests <JSON>
          A list of updates to apply to the document.

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
          The optional revision ID of the document the write request is applied to. If this is not the latest revision of the document, the request is not processed and returns a 400 bad request error. When a required revision ID is returned in a response, it indicates the revision ID of the document after the request was applied.

      --write-control-target-revision-id <WRITE_CONTROL_TARGET_REVISION_ID>
          The optional target revision ID of the document the write request is applied to. If collaborator changes have occurred after the document was read using the API, the changes produced by this write request are applied against the collaborator changes. This results in a new revision of the document that incorporates both the collaborator changes and the changes in the request, with the Docs server resolving conflicting changes. When using target revision ID, the API client can be thought of as another collaborator of the document. The target revision ID can only be used to write to recent versions of a document. If the target revision is too far behind the latest revision, the request is not processed and returns a 400 bad request error. The request should be tried again after retrieving the latest version of the document. Usually a revision ID remains valid for use as a target revision for several minutes after it's read, but for frequently edited documents this window might be shorter.

      --write-control-write-mode <WRITE_CONTROL_WRITE_MODE>
          How the request updates should be applied to the document. If unspecified, the request updates will be applied as normal edits. Choices: WRITE_MODE_UNSPECIFIED, EDIT, SUGGEST.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create

```text
Creates a blank document using the title given in the request. Other fields in the request, including any provided content, are ignored. Returns the created document.

Usage: joog [OPTIONS] docs <ACCOUNT> create [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --title <TITLE>
          The title of the document.

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

      --body-body <JSON>
          The document body. The body typically contains the full document contents except for headers, footers, and footnotes.

      --body-content <JSON>
          The contents of the body. The indexes for the body's content begin at zero.

      --document-style <JSON>
          The style of the document.

      --document-style-default-footer-id <DOCUMENT_STYLE_DEFAULT_FOOTER_ID>
          The ID of the default footer. If not set, there's no default footer. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-default-header-id <DOCUMENT_STYLE_DEFAULT_HEADER_ID>
          The ID of the default header. If not set, there's no default header. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-document-format-document-mode <DOCUMENT_STYLE_DOCUMENT_FORMAT_DOCUMENT_MODE>
          Whether the document has pages or is pageless. Choices: DOCUMENT_MODE_UNSPECIFIED, PAGES, PAGELESS.

      --document-style-even-page-footer-id <DOCUMENT_STYLE_EVEN_PAGE_FOOTER_ID>
          The ID of the footer used only for even pages. The value of use_even_page_header_footer determines whether to use the default_footer_id or this value for the footer on even pages. If not set, there's no even page footer. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-even-page-header-id <DOCUMENT_STYLE_EVEN_PAGE_HEADER_ID>
          The ID of the header used only for even pages. The value of use_even_page_header_footer determines whether to use the default_header_id or this value for the header on even pages. If not set, there's no even page header. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-first-page-footer-id <DOCUMENT_STYLE_FIRST_PAGE_FOOTER_ID>
          The ID of the footer used only for the first page. If not set then a unique footer for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_footer_id or this value for the footer on the first page. If not set, there's no first page footer. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-first-page-header-id <DOCUMENT_STYLE_FIRST_PAGE_HEADER_ID>
          The ID of the header used only for the first page. If not set then a unique header for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_header_id or this value for the header on the first page. If not set, there's no first page header. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-flip-page-orientation[=<BOOL>]
          Optional. Indicates whether to flip the dimensions of the page_size, which allows changing the page orientation between portrait and landscape. If DocumentMode is PAGELESS, this property will not be rendered.

      --document-style-margin-bottom-magnitude <DOCUMENT_STYLE_MARGIN_BOTTOM_MAGNITUDE>
          The magnitude.

      --document-style-margin-bottom-unit <DOCUMENT_STYLE_MARGIN_BOTTOM_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-footer-magnitude <DOCUMENT_STYLE_MARGIN_FOOTER_MAGNITUDE>
          The magnitude.

      --document-style-margin-footer-unit <DOCUMENT_STYLE_MARGIN_FOOTER_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-header-magnitude <DOCUMENT_STYLE_MARGIN_HEADER_MAGNITUDE>
          The magnitude.

      --document-style-margin-header-unit <DOCUMENT_STYLE_MARGIN_HEADER_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-left-magnitude <DOCUMENT_STYLE_MARGIN_LEFT_MAGNITUDE>
          The magnitude.

      --document-style-margin-left-unit <DOCUMENT_STYLE_MARGIN_LEFT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-right-magnitude <DOCUMENT_STYLE_MARGIN_RIGHT_MAGNITUDE>
          The magnitude.

      --document-style-margin-right-unit <DOCUMENT_STYLE_MARGIN_RIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-top-magnitude <DOCUMENT_STYLE_MARGIN_TOP_MAGNITUDE>
          The magnitude.

      --document-style-margin-top-unit <DOCUMENT_STYLE_MARGIN_TOP_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-page-number-start <DOCUMENT_STYLE_PAGE_NUMBER_START>
          The page number from which to start counting the number of pages. If DocumentMode is PAGELESS, this property will not be rendered.

      --document-style-page-size-height-magnitude <DOCUMENT_STYLE_PAGE_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --document-style-page-size-height-unit <DOCUMENT_STYLE_PAGE_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-page-size-width-magnitude <DOCUMENT_STYLE_PAGE_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --document-style-page-size-width-unit <DOCUMENT_STYLE_PAGE_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-use-custom-header-footer-margins[=<BOOL>]
          Indicates whether DocumentStyle margin_header, SectionStyle margin_header and DocumentStyle margin_footer, SectionStyle margin_footer are respected. When false, the default values in the Docs editor for header and footer margin is used. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-use-even-page-header-footer[=<BOOL>]
          Indicates whether to use the even page header / footer IDs for the even pages. If DocumentMode is PAGELESS, this property will not be rendered.

      --document-style-use-first-page-header-footer[=<BOOL>]
          Indicates whether to use the first page header / footer IDs for the first page. If DocumentMode is PAGELESS, this property will not be rendered.

      --named-styles <JSON>
          The named styles. Paragraphs in the document can inherit their TextStyle and ParagraphStyle from these named styles.

      --named-styles-styles <JSON>
          The named styles. There's an entry for each of the possible named style types.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-dropdown-definition

```text
Creates a DropdownDefinition.

Usage: joog [OPTIONS] docs <ACCOUNT> create-dropdown-definition [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --dropdown-definition <JSON>
          A dropdown definition in the document.

      --dropdown-definition-dropdown-definition-id <DROPDOWN_DEFINITION_DROPDOWN_DEFINITION_ID>
          The ID of this dropdown definition. If you specify an ID, it must be unique among all IDs in the tab. The ID must start with `kix.` and match regex `^kix\.[a-zA-Z0-9_-]{2,14}$` (length 6-18 chars). If you don't specify an ID, a unique one is generated.

      --dropdown-definition-dropdown-definition-properties-options <JSON>
          The list of options defined by this dropdown definition. A dropdown definition must have at least 2 options and at most 50 options.

      --dropdown-definition-dropdown-definition-properties-title <DROPDOWN_DEFINITION_DROPDOWN_DEFINITION_PROPERTIES_TITLE>
          The title of the dropdown definition.

      --dropdown-definition-suggested-deletion-id <DROPDOWN_DEFINITION_SUGGESTED_DELETION_ID>
          ID for suggestion that deletes this dropdown definition.

      --dropdown-definition-suggested-insertion-id <DROPDOWN_DEFINITION_SUGGESTED_INSERTION_ID>
          ID for suggestion that inserts this dropdown definition.

      --tab-id <TAB_ID>
          The ID of the tab to create the dropdown definition in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-footer

```text
Creates a footer.

Usage: joog [OPTIONS] docs <ACCOUNT> create-footer [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --type <TYPE>
          The type of footer to create. Choices: HEADER_FOOTER_TYPE_UNSPECIFIED, DEFAULT.

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

      --section-break-location <JSON>
          A particular location in the document.

      --section-break-location-index <SECTION_BREAK_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --section-break-location-segment-id <SECTION_BREAK_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --section-break-location-tab-id <SECTION_BREAK_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-footnote

```text
Creates a footnote.

Usage: joog [OPTIONS] docs <ACCOUNT> create-footnote [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-header

```text
Creates a header.

Usage: joog [OPTIONS] docs <ACCOUNT> create-header [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --type <TYPE>
          The type of header to create. Choices: HEADER_FOOTER_TYPE_UNSPECIFIED, DEFAULT.

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

      --section-break-location <JSON>
          A particular location in the document.

      --section-break-location-index <SECTION_BREAK_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --section-break-location-segment-id <SECTION_BREAK_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --section-break-location-tab-id <SECTION_BREAK_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-named-range

```text
Creates a named range.

Usage: joog [OPTIONS] docs <ACCOUNT> create-named-range [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --name <NAME>
          The name of the NamedRange. Names do not need to be unique. Names must be at least 1 character and no more than 256 characters, measured in UTF-16 code units.

      --range <JSON>
          Specifies a contiguous range of text.

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

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create-paragraph-bullets

```text
Creates bullets for paragraphs.

Usage: joog [OPTIONS] docs <ACCOUNT> create-paragraph-bullets [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --range <JSON>
          Specifies a contiguous range of text.

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
          The kinds of bullet glyphs to be used. Choices: BULLET_GLYPH_PRESET_UNSPECIFIED, BULLET_DISC_CIRCLE_SQUARE, BULLET_DIAMONDX_ARROW3D_SQUARE, BULLET_CHECKBOX, BULLET_ARROW_DIAMOND_DISC, BULLET_STAR_CIRCLE_SQUARE, BULLET_ARROW3D_CIRCLE_SQUARE, BULLET_LEFTTRIANGLE_DIAMOND_DISC, BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE, BULLET_DIAMOND_CIRCLE_SQUARE, NUMBERED_DECIMAL_ALPHA_ROMAN, NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS, NUMBERED_DECIMAL_NESTED, NUMBERED_UPPERALPHA_ALPHA_ROMAN, NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL, NUMBERED_ZERODECIMAL_ALPHA_ROMAN.

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-comment

```text
Deletes a CommentThread.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-comment [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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
Deletes a reply Post from a CommentThread or SuggestionThread.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-comment-reply [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --suggestion-id <SUGGESTION_ID>
          The ID of the SuggestionThread which the post belongs to.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-content-range

```text
Deletes content from the document.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-content-range [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --range <JSON>
          Specifies a contiguous range of text.

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

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-dropdown-definition

```text
Deletes a DropdownDefinition.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-dropdown-definition [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --dropdown-definition-id <DROPDOWN_DEFINITION_ID>
          The ID of the DropdownDefinition to delete.

      --tab-id <TAB_ID>
          The ID of the tab that contains the dropdown definition to delete. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-footer

```text
Deletes a footer from the document.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-footer [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --footer-id <FOOTER_ID>
          The id of the footer to delete. If this footer is defined on DocumentStyle, the reference to this footer is removed, resulting in no footer of that type for the first section of the document. If this footer is defined on a SectionStyle, the reference to this footer is removed and the footer of that type is now continued from the previous section.

      --tab-id <TAB_ID>
          The tab that contains the footer to delete. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-header

```text
Deletes a header from the document.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-header [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --header-id <HEADER_ID>
          The id of the header to delete. If this header is defined on DocumentStyle, the reference to this header is removed, resulting in no header of that type for the first section of the document. If this header is defined on a SectionStyle, the reference to this header is removed and the header of that type is now continued from the previous section.

      --tab-id <TAB_ID>
          The tab containing the header to delete. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-named-range

```text
Deletes a named range.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-named-range [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --name <NAME>
          The name of the range(s) to delete. All named ranges with the given name will be deleted.

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

      --named-range-id <NAMED_RANGE_ID>
          The ID of the named range to delete.

      --tabs-criteria <JSON>
          A criteria that specifies in which tabs a request executes.

      --tabs-criteria-tab-ids <TABS_CRITERIA_TAB_IDS>...
          The list of tab IDs in which the request executes.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-paragraph-bullets

```text
Deletes bullets from paragraphs.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-paragraph-bullets [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --range <JSON>
          Specifies a contiguous range of text.

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

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-positioned-object

```text
Deletes a positioned object from the document.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-positioned-object [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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
          The ID of the positioned object to delete.

      --tab-id <TAB_ID>
          The tab that the positioned object to delete is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-suggestion

```text
Deletes a suggestion.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-suggestion [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --suggestion-id <SUGGESTION_ID>
          The ID of the suggestion.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-tab

```text
Deletes a document tab.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-tab [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --tab-id <TAB_ID>
          The ID of the tab to delete.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-table-column

```text
Deletes a column from a table.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-table-column [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --table-cell-location <JSON>
          Location of a single cell within a table.

      --table-cell-location-column-index <TABLE_CELL_LOCATION_COLUMN_INDEX>
          The zero-based column index. For example, the second column in the table has a column index of 1.

      --table-cell-location-row-index <TABLE_CELL_LOCATION_ROW_INDEX>
          The zero-based row index. For example, the second row in the table has a row index of 1.

      --table-cell-location-table-start-location-index <TABLE_CELL_LOCATION_TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-cell-location-table-start-location-segment-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-cell-location-table-start-location-tab-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## delete-table-row

```text
Deletes a row from a table.

Usage: joog [OPTIONS] docs <ACCOUNT> delete-table-row [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --table-cell-location <JSON>
          Location of a single cell within a table.

      --table-cell-location-column-index <TABLE_CELL_LOCATION_COLUMN_INDEX>
          The zero-based column index. For example, the second column in the table has a column index of 1.

      --table-cell-location-row-index <TABLE_CELL_LOCATION_ROW_INDEX>
          The zero-based row index. For example, the second row in the table has a row index of 1.

      --table-cell-location-table-start-location-index <TABLE_CELL_LOCATION_TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-cell-location-table-start-location-segment-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-cell-location-table-start-location-tab-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## get

```text
Gets the latest version of the specified document.

Usage: joog [OPTIONS] docs <ACCOUNT> get [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to retrieve.

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
          The comments view mode to apply to the document. This allows viewing the document with comments omitted or included. If one is not specified, COMMENTS_VIEW_MODE_OMITTED is used. If you set comments_view_mode to any value, you must also set include_tabs_content to `true` or use a field mask that references the Document.tabs field (or any subfield). If you set comments_view_mode to COMMENTS_VIEW_MODE_INCLUDED, you must also explicitly set suggestions_view_mode to SUGGESTIONS_INLINE. If you set comments_view_mode to COMMENTS_VIEW_MODE_INCLUDED or COMMENTS_VIEW_MODE_DEFAULT_FOR_CURRENT_ACCESS, you may not set suggestions_view_mode to PREVIEW_WITHOUT_SUGGESTIONS or PREVIEW_SUGGESTIONS_ACCEPTED. Choices: COMMENTS_VIEW_MODE_UNSPECIFIED, COMMENTS_VIEW_MODE_DEFAULT_FOR_CURRENT_ACCESS, COMMENTS_VIEW_MODE_OMITTED, COMMENTS_VIEW_MODE_INCLUDED.

      --include-tabs-content[=<BOOL>]
          Whether to populate the `Document.tabs` field instead of the text content fields like `body` and `documentStyle` on `Document`. - When `true`: Document content populates in the `Document.tabs` field instead of the text content fields in `Document`. - When `false`: The content of the document's first tab populates the content fields in `Document` excluding `Document.tabs`. If a document has only one tab, then that tab is used to populate the document content. `Document.tabs` will be empty. If you use a field mask that references the `Document.tabs` field (or any subfield), the API implicitly treats the request as if you set `include_tabs_content` to `true`.

      --suggestions-view-mode <SUGGESTIONS_VIEW_MODE>
          The suggestions view mode to apply to the document. This allows viewing the document with all suggestions inline, accepted or rejected. If one is not specified, DEFAULT_FOR_CURRENT_ACCESS is used. Choices: DEFAULT_FOR_CURRENT_ACCESS, SUGGESTIONS_INLINE, PREVIEW_SUGGESTIONS_ACCEPTED, PREVIEW_WITHOUT_SUGGESTIONS.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-comment

```text
Inserts a CommentThread into the document.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-comment [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --content <CONTENT>
          The text of the comment, as plain text. This text content will be handled similarly to comments created in the Docs editor. It will have similar behaviors for formatting, notifications, etc. This field cannot be empty, and must not exceed 2048 UTF-8 code units.

      --range <JSON>
          Specifies a contiguous range of text.

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

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-date

```text
Inserts a date.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-date [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --date-element-properties <JSON>
          Properties of a DateElement.

      --date-element-properties-date-format <DATE_ELEMENT_PROPERTIES_DATE_FORMAT>
          Determines how the date part of the DateElement will be displayed in the document. If unset, the default value is DATE_FORMAT_MONTH_DAY_YEAR_ABBREVIATED, indicating the DateElement will be formatted as `MMM d, y` in `en`, or locale specific equivalent. Choices: DATE_FORMAT_UNSPECIFIED, DATE_FORMAT_CUSTOM, DATE_FORMAT_MONTH_DAY_ABBREVIATED, DATE_FORMAT_MONTH_DAY_FULL, DATE_FORMAT_MONTH_DAY_YEAR_ABBREVIATED, DATE_FORMAT_ISO8601.

      --date-element-properties-locale <DATE_ELEMENT_PROPERTIES_LOCALE>
          The language code of the DateElement. For example, `en`. If unset, the default locale is `en`. Limited to the following locales: `af`, `am`, `ar`, `as`, `az`, `be`, `bg`, `bn`, `ca`, `cs`, `da`, `de`, `el`, `en`, `en-CA`, `en-GB`, `es`, `es-419`, `et`, `eu`, `fa`, `fi`, `fil`, `fr`, `fr-CA`, `gl`, `gu`, `hi`, `hr`, `hu`, `hy`, `id`, `is`, `it`, `iw`, `ja`, `ka`, `kk`, `km`, `kn`, `ko`, `lo`, `lt`, `lv`, `mk`, `ml`, `mn`, `mr`, `ms`, `ne`, `nl`, `no`, `or`, `pa`, `pl`, `pt-BR`, `pt-PT`, `ro`, `ru`, `si`, `sk`, `sl`, `sq`, `sr`, `sv`, `sw`, `ta`, `te`, `th`, `tr`, `uk`, `ur`, `uz`, `vi`, `zh-CN`, `zh-HK`, `zh-TW`, `zu`, `cy`, `my`.

      --date-element-properties-time-format <DATE_ELEMENT_PROPERTIES_TIME_FORMAT>
          Determines how the time part of the DateElement will be displayed in the document. If unset, the default value is TIME_FORMAT_DISABLED, indicating no time should be shown. Choices: TIME_FORMAT_UNSPECIFIED, TIME_FORMAT_DISABLED, TIME_FORMAT_HOUR_MINUTE, TIME_FORMAT_HOUR_MINUTE_TIMEZONE.

      --date-element-properties-time-zone-id <DATE_ELEMENT_PROPERTIES_TIME_ZONE_ID>
          The time zone of the DateElement, as defined by the Unicode Common Locale Data Repository (CLDR) project. For example, `America/New_York`. If unset, the default time zone is `etc/UTC`.

      --date-element-properties-timestamp <DATE_ELEMENT_PROPERTIES_TIMESTAMP>
          The point in time to represent, in seconds and nanoseconds since Unix epoch: January 1, 1970 at midnight UTC. Timestamp is expected to be in UTC. If time_zone_id is set, the timestamp is adjusted according to the time zone. For example, a timestamp of `18000` with a date format of `DATE_FORMAT_ISO8601` and time format of `TIME_FORMAT_HOUR_MINUTE` would be displayed as `1970-01-01 5:00 AM`. A timestamp of `18000` with date format of `DATE_FORMAT_ISO8601`, time format of `TIME_FORMAT_HOUR_MINUTE`, and time zone set to `America/New_York` will instead be `1970-01-01 12:00 AM`.

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-dropdown

```text
Inserts a Dropdown at the specified location.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-dropdown [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --dropdown-definition-id <DROPDOWN_DEFINITION_ID>
          Required. The DropdownDefinition ID.

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --selected-option-id <SELECTED_OPTION_ID>
          Optional initial value for the dropdown. If this field is not specified, the new dropdown will default to selecting the first option defined in the dropdown definition. If this field is specified but does not reference a valid option in the dropdown definition, a 400 bad request error is returned.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-inline-image

```text
Inserts an inline image at the specified location.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-inline-image [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --object-size <JSON>
          A width and height.

      --object-size-height-magnitude <OBJECT_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --object-size-height-unit <OBJECT_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --object-size-width-magnitude <OBJECT_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --object-size-width-unit <OBJECT_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --uri <URI>
          The image URI. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URI must be publicly accessible and at most 2 kB in length. The URI itself is saved with the image, and exposed via the ImageProperties.content_uri field.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-page-break

```text
Inserts a page break at the specified location.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-page-break [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-person

```text
Inserts a person mention.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-person [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --person-properties <JSON>
          Properties specific to a linked Person.

      --person-properties-email <PERSON_PROPERTIES_EMAIL>
          The email address linked to this Person. This field is always present.

      --person-properties-name <PERSON_PROPERTIES_NAME>
          The name of the person if it's displayed in the link text instead of the person's email address.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-rich-link

```text
Insert a rich link.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-rich-link [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --rich-link-properties <JSON>
          Properties specific to a RichLink.

      --rich-link-properties-mime-type <RICH_LINK_PROPERTIES_MIME_TYPE>
          The [MIME type](https://developers.google.com/drive/api/v3/mime-types) of the RichLink, if there's one (for example, when it's a file in Drive).

      --rich-link-properties-title <RICH_LINK_PROPERTIES_TITLE>
          The title of the RichLink as displayed in the link. This title matches the title of the linked resource at the time of the insertion or last update of the link. This field is always present.

      --rich-link-properties-uri <RICH_LINK_PROPERTIES_URI>
          The URI to the RichLink. This is always present.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-section-break

```text
Inserts a section break at the specified location.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-section-break [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --section-type <SECTION_TYPE>
          The type of section to insert. Choices: SECTION_TYPE_UNSPECIFIED, CONTINUOUS, NEXT_PAGE.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-table

```text
Inserts a table at the specified location.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-table [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

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
          The number of columns in the table.

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --rows <ROWS>
          The number of rows in the table.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-table-column

```text
Inserts an empty column into a table.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-table-column [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --insert-right[=<BOOL>]
          Whether to insert new column to the right of the reference cell location. - `True`: insert to the right. - `False`: insert to the left.

      --table-cell-location <JSON>
          Location of a single cell within a table.

      --table-cell-location-column-index <TABLE_CELL_LOCATION_COLUMN_INDEX>
          The zero-based column index. For example, the second column in the table has a column index of 1.

      --table-cell-location-row-index <TABLE_CELL_LOCATION_ROW_INDEX>
          The zero-based row index. For example, the second row in the table has a row index of 1.

      --table-cell-location-table-start-location-index <TABLE_CELL_LOCATION_TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-cell-location-table-start-location-segment-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-cell-location-table-start-location-tab-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-table-row

```text
Inserts an empty row into a table.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-table-row [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --insert-below[=<BOOL>]
          Whether to insert new row below the reference cell location. - `True`: insert below the cell. - `False`: insert above the cell.

      --table-cell-location <JSON>
          Location of a single cell within a table.

      --table-cell-location-column-index <TABLE_CELL_LOCATION_COLUMN_INDEX>
          The zero-based column index. For example, the second column in the table has a column index of 1.

      --table-cell-location-row-index <TABLE_CELL_LOCATION_ROW_INDEX>
          The zero-based row index. For example, the second row in the table has a row index of 1.

      --table-cell-location-table-start-location-index <TABLE_CELL_LOCATION_TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-cell-location-table-start-location-segment-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-cell-location-table-start-location-tab-id <TABLE_CELL_LOCATION_TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## insert-text

```text
Inserts text at the specified location.

Usage: joog [OPTIONS] docs <ACCOUNT> insert-text [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --index <INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --text <TEXT>
          The text to be inserted. Inserting a newline character will implicitly create a new Paragraph at that index. The paragraph style of the new paragraph will be copied from the paragraph at the current insertion index, including lists and bullets. Text styles for inserted text will be determined automatically, generally preserving the styling of neighboring text. In most cases, the text style for the inserted text will match the text immediately before the insertion index. Some control characters (U+0000-U+0008, U+000C-U+001F) and characters from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF) will be stripped out of the inserted text.

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

      --end-of-segment-location <JSON>
          Location at the end of a body, header, footer or footnote. The location is immediately before the last newline in the document segment.

      --end-of-segment-location-segment-id <END_OF_SEGMENT_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --end-of-segment-location-tab-id <END_OF_SEGMENT_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --location <JSON>
          A particular location in the document.

      --location-segment-id <LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --location-tab-id <LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## merge-table-cells

```text
Merges cells in a table.

Usage: joog [OPTIONS] docs <ACCOUNT> merge-table-cells [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --table-range <JSON>
          A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with table cell location = (table_start_location, row = 0, column = 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]

      --table-range-column-span <TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-range-row-span <TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --table-range-table-cell-location-column-index <TABLE_RANGE_TABLE_CELL_LOCATION_COLUMN_INDEX>
          The zero-based column index. For example, the second column in the table has a column index of 1.

      --table-range-table-cell-location-row-index <TABLE_RANGE_TABLE_CELL_LOCATION_ROW_INDEX>
          The zero-based row index. For example, the second row in the table has a row index of 1.

      --table-range-table-cell-location-table-start-location-index <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-range-table-cell-location-table-start-location-segment-id <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-range-table-cell-location-table-start-location-tab-id <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## pin-table-header-rows

```text
Updates the number of pinned header rows in a table.

Usage: joog [OPTIONS] docs <ACCOUNT> pin-table-header-rows [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --pinned-header-rows-count <PINNED_HEADER_ROWS_COUNT>
          The number of table rows to pin, where 0 implies that all rows are unpinned.

      --table-start-location <JSON>
          A particular location in the document.

      --table-start-location-index <TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-start-location-segment-id <TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-start-location-tab-id <TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## reject-suggestion

```text
Rejects a suggestion.

Usage: joog [OPTIONS] docs <ACCOUNT> reject-suggestion [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --suggestion-id <SUGGESTION_ID>
          The ID of the suggestion.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replace-all-text

```text
Replaces all instances of the specified text.

Usage: joog [OPTIONS] docs <ACCOUNT> replace-all-text [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --find <FIND>
          The text to search for in the document.

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
          A criteria that matches a specific string of text in the document.

      --contains-text-search-by-regex[=<BOOL>]
          Optional. True if the find value should be treated as a regular expression. Any backslashes in the pattern should be escaped. - `True`: the search text is treated as a regular expressions. - `False`: the search text is treated as a substring for matching.

      --replace-text <REPLACE_TEXT>
          The text that will replace the matched text.

      --tabs-criteria <JSON>
          A criteria that specifies in which tabs a request executes.

      --tabs-criteria-tab-ids <TABS_CRITERIA_TAB_IDS>...
          The list of tab IDs in which the request executes.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replace-image

```text
Replaces an image in the document.

Usage: joog [OPTIONS] docs <ACCOUNT> replace-image [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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
          The replacement method. Choices: IMAGE_REPLACE_METHOD_UNSPECIFIED, CENTER_CROP.

      --tab-id <TAB_ID>
          The tab that the image to be replaced is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --uri <URI>
          The URI of the new image. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB, cannot exceed 25 megapixels, and must be in PNG, JPEG, or GIF format. The provided URI can't surpass 2 KB in length. The URI is saved with the image, and exposed through the ImageProperties.source_uri field.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replace-named-range-content

```text
Replaces the content in a named range.

Usage: joog [OPTIONS] docs <ACCOUNT> replace-named-range-content [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --text <TEXT>
          Replaces the content of the specified named range(s) with the given text.

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

      --named-range-id <NAMED_RANGE_ID>
          The ID of the named range whose content will be replaced. If there is no named range with the given ID a 400 bad request error is returned.

      --named-range-name <NAMED_RANGE_NAME>
          The name of the NamedRanges whose content will be replaced. If there are multiple named ranges with the given name, then the content of each one will be replaced. If there are no named ranges with the given name, then the request will be a no-op.

      --tabs-criteria <JSON>
          A criteria that specifies in which tabs a request executes.

      --tabs-criteria-tab-ids <TABS_CRITERIA_TAB_IDS>...
          The list of tab IDs in which the request executes.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## unmerge-table-cells

```text
Unmerges cells in a table.

Usage: joog [OPTIONS] docs <ACCOUNT> unmerge-table-cells [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

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

      --table-range <JSON>
          A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with table cell location = (table_start_location, row = 0, column = 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]

      --table-range-column-span <TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-range-row-span <TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --table-range-table-cell-location-column-index <TABLE_RANGE_TABLE_CELL_LOCATION_COLUMN_INDEX>
          The zero-based column index. For example, the second column in the table has a column index of 1.

      --table-range-table-cell-location-row-index <TABLE_RANGE_TABLE_CELL_LOCATION_ROW_INDEX>
          The zero-based row index. For example, the second row in the table has a row index of 1.

      --table-range-table-cell-location-table-start-location-index <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-range-table-cell-location-table-start-location-segment-id <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-range-table-cell-location-table-start-location-tab-id <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-comment-post

```text
Updates an existing post (head post or reply) of a CommentThread or SuggestionThread.

Usage: joog [OPTIONS] docs <ACCOUNT> update-comment-post [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --content <CONTENT>
          The new text of the comment, as plain text. This text content will be handled similarly to comments created in the Docs editor. It will have similar behaviors for formatting, notifications, etc. This field cannot be empty, and must not exceed 2048 UTF-8 code units.

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

      --suggestion-id <SUGGESTION_ID>
          The ID of the SuggestionThread which the post belongs to.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-document-style

```text
Updates the style of the document.

Usage: joog [OPTIONS] docs <ACCOUNT> update-document-style [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `document_style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the background, set `fields` to `"background"`.

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

      --document-style <JSON>
          The style of the document.

      --document-style-default-footer-id <DOCUMENT_STYLE_DEFAULT_FOOTER_ID>
          The ID of the default footer. If not set, there's no default footer. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-default-header-id <DOCUMENT_STYLE_DEFAULT_HEADER_ID>
          The ID of the default header. If not set, there's no default header. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-document-format-document-mode <DOCUMENT_STYLE_DOCUMENT_FORMAT_DOCUMENT_MODE>
          Whether the document has pages or is pageless. Choices: DOCUMENT_MODE_UNSPECIFIED, PAGES, PAGELESS.

      --document-style-even-page-footer-id <DOCUMENT_STYLE_EVEN_PAGE_FOOTER_ID>
          The ID of the footer used only for even pages. The value of use_even_page_header_footer determines whether to use the default_footer_id or this value for the footer on even pages. If not set, there's no even page footer. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-even-page-header-id <DOCUMENT_STYLE_EVEN_PAGE_HEADER_ID>
          The ID of the header used only for even pages. The value of use_even_page_header_footer determines whether to use the default_header_id or this value for the header on even pages. If not set, there's no even page header. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-first-page-footer-id <DOCUMENT_STYLE_FIRST_PAGE_FOOTER_ID>
          The ID of the footer used only for the first page. If not set then a unique footer for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_footer_id or this value for the footer on the first page. If not set, there's no first page footer. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-first-page-header-id <DOCUMENT_STYLE_FIRST_PAGE_HEADER_ID>
          The ID of the header used only for the first page. If not set then a unique header for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_header_id or this value for the header on the first page. If not set, there's no first page header. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-flip-page-orientation[=<BOOL>]
          Optional. Indicates whether to flip the dimensions of the page_size, which allows changing the page orientation between portrait and landscape. If DocumentMode is PAGELESS, this property will not be rendered.

      --document-style-margin-bottom-magnitude <DOCUMENT_STYLE_MARGIN_BOTTOM_MAGNITUDE>
          The magnitude.

      --document-style-margin-bottom-unit <DOCUMENT_STYLE_MARGIN_BOTTOM_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-footer-magnitude <DOCUMENT_STYLE_MARGIN_FOOTER_MAGNITUDE>
          The magnitude.

      --document-style-margin-footer-unit <DOCUMENT_STYLE_MARGIN_FOOTER_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-header-magnitude <DOCUMENT_STYLE_MARGIN_HEADER_MAGNITUDE>
          The magnitude.

      --document-style-margin-header-unit <DOCUMENT_STYLE_MARGIN_HEADER_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-left-magnitude <DOCUMENT_STYLE_MARGIN_LEFT_MAGNITUDE>
          The magnitude.

      --document-style-margin-left-unit <DOCUMENT_STYLE_MARGIN_LEFT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-right-magnitude <DOCUMENT_STYLE_MARGIN_RIGHT_MAGNITUDE>
          The magnitude.

      --document-style-margin-right-unit <DOCUMENT_STYLE_MARGIN_RIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-margin-top-magnitude <DOCUMENT_STYLE_MARGIN_TOP_MAGNITUDE>
          The magnitude.

      --document-style-margin-top-unit <DOCUMENT_STYLE_MARGIN_TOP_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-page-number-start <DOCUMENT_STYLE_PAGE_NUMBER_START>
          The page number from which to start counting the number of pages. If DocumentMode is PAGELESS, this property will not be rendered.

      --document-style-page-size-height-magnitude <DOCUMENT_STYLE_PAGE_SIZE_HEIGHT_MAGNITUDE>
          The magnitude.

      --document-style-page-size-height-unit <DOCUMENT_STYLE_PAGE_SIZE_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-page-size-width-magnitude <DOCUMENT_STYLE_PAGE_SIZE_WIDTH_MAGNITUDE>
          The magnitude.

      --document-style-page-size-width-unit <DOCUMENT_STYLE_PAGE_SIZE_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --document-style-use-custom-header-footer-margins[=<BOOL>]
          Indicates whether DocumentStyle margin_header, SectionStyle margin_header and DocumentStyle margin_footer, SectionStyle margin_footer are respected. When false, the default values in the Docs editor for header and footer margin is used. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --document-style-use-even-page-header-footer[=<BOOL>]
          Indicates whether to use the even page header / footer IDs for the even pages. If DocumentMode is PAGELESS, this property will not be rendered.

      --document-style-use-first-page-header-footer[=<BOOL>]
          Indicates whether to use the first page header / footer IDs for the first page. If DocumentMode is PAGELESS, this property will not be rendered.

      --tab-id <TAB_ID>
          The tab that contains the style to update. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If not provided, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-document-tab-properties

```text
Updates the properties of a document tab.

Usage: joog [OPTIONS] docs <ACCOUNT> update-document-tab-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tab_properties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.

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

      --tab-properties <JSON>
          Properties of a tab.

      --tab-properties-icon-emoji <TAB_PROPERTIES_ICON_EMOJI>
          Optional. The emoji icon displayed with the tab. A valid emoji icon is represented by a non-empty Unicode string. Any set of characters that don't represent a single emoji is invalid. If an emoji is invalid, a 400 bad request error is returned. If this value is unset or empty, the tab will display the default tab icon.

      --tab-properties-index <TAB_PROPERTIES_INDEX>
          The zero-based index of the tab within the parent.

      --tab-properties-parent-tab-id <TAB_PROPERTIES_PARENT_TAB_ID>
          Optional. The ID of the parent tab. Empty when the current tab is a root-level tab, which means it doesn't have any parents.

      --tab-properties-tab-id <TAB_PROPERTIES_TAB_ID>
          The immutable ID of the tab.

      --tab-properties-title <TAB_PROPERTIES_TITLE>
          The user-visible name of the tab.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-dropdown-definition-properties

```text
Updates the properties of a DropdownDefinition.

Usage: joog [OPTIONS] docs <ACCOUNT> update-dropdown-definition-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `dropdown_definition_properties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. When `dropdown_definition_properties.options` is included in the field mask, the full, complete list of desired options must be provided in `dropdown_definition_properties.options`.

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

      --dropdown-definition-id <DROPDOWN_DEFINITION_ID>
          The ID of the DropdownDefinition to update.

      --dropdown-definition-properties <JSON>
          Properties of a dropdown definition.

      --dropdown-definition-properties-options <JSON>
          The list of options defined by this dropdown definition. A dropdown definition must have at least 2 options and at most 50 options.

      --dropdown-definition-properties-title <DROPDOWN_DEFINITION_PROPERTIES_TITLE>
          The title of the dropdown definition.

      --selected-option-id-replacements <JSON>
          A map of option IDs to their replacements, used to automatically reassign orphaned Dropdown chips when an option is deleted. The keys are the IDs of the options being deleted, and the values are the IDs of their replacement options. If an option being deleted is selected in one or more Dropdown chips in the document, a replacement entry for that option must be provided in this map, and the replacement option ID must exist in the updated DropdownDefinition. If a replacement is required but not provided, a 400 bad request error is returned. Options being deleted that are not selected in any Dropdown chips do not require a replacement. For example, if option A is being replaced by option B, the map should be `{"A": "B"}`.

      --tab-id <TAB_ID>
          The ID of the tab that contains the dropdown definition to update. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-dropdown-properties

```text
Updates the properties of a Dropdown.

Usage: joog [OPTIONS] docs <ACCOUNT> update-dropdown-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `dropdown_properties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.

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

      --dropdown-id <DROPDOWN_ID>
          Required. The Dropdown ID.

      --dropdown-properties <JSON>
          Properties specific to a dropdown.

      --dropdown-properties-display-value <DROPDOWN_PROPERTIES_DISPLAY_VALUE>
          The human-readable display text of the currently selected item. This field is populated by the server based on the dropdown definition and the selected option ID. It may differ from `DropdownOption.display_value` if the underlying option definition was modified or deleted, or during pending suggested changes.

      --dropdown-properties-dropdown-definition-id <DROPDOWN_PROPERTIES_DROPDOWN_DEFINITION_ID>
          The ID of the DropdownDefinition that defines the options for this dropdown.

      --dropdown-properties-selected-option-id <DROPDOWN_PROPERTIES_SELECTED_OPTION_ID>
          The ID of the selected option in this dropdown.

      --tab-id <TAB_ID>
          The ID of the tab that contains the dropdown to update. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-named-style

```text
Updates a named style.

Usage: joog [OPTIONS] docs <ACCOUNT> update-named-style [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The NamedStyle fields that should be updated. At least `named_style_type` must be specified. The root `named_style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example, to update the text style to bold, set `fields` to include `"text_style"` and `"text_style.bold"`. To update the paragraph style's alignment property, set `fields` to include `"paragraph_style"` and `"paragraph_style.alignment"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset. Specifying `"text_style"` or `"paragraph_style"` with an empty TextStyle or ParagraphStyle will reset all of its nested fields.

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

      --named-style <JSON>
          A named style. Paragraphs in the document can inherit their TextStyle and ParagraphStyle from this named style when they have the same named style type.

      --named-style-named-style-type <NAMED_STYLE_NAMED_STYLE_TYPE>
          The type of this named style. Choices: NAMED_STYLE_TYPE_UNSPECIFIED, NORMAL_TEXT, TITLE, SUBTITLE, HEADING_1, HEADING_2, HEADING_3, HEADING_4, HEADING_5, HEADING_6.

      --named-style-paragraph-style-alignment <NAMED_STYLE_PARAGRAPH_STYLE_ALIGNMENT>
          The text alignment for this paragraph. Choices: ALIGNMENT_UNSPECIFIED, START, CENTER, END, JUSTIFIED.

      --named-style-paragraph-style-avoid-widow-and-orphan[=<BOOL>]
          Whether to avoid widows and orphans for the paragraph. If unset, the value is inherited from the parent.

      --named-style-paragraph-style-border-between-dash-style <NAMED_STYLE_PARAGRAPH_STYLE_BORDER_BETWEEN_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --named-style-paragraph-style-border-bottom-dash-style <NAMED_STYLE_PARAGRAPH_STYLE_BORDER_BOTTOM_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --named-style-paragraph-style-border-left-dash-style <NAMED_STYLE_PARAGRAPH_STYLE_BORDER_LEFT_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --named-style-paragraph-style-border-right-dash-style <NAMED_STYLE_PARAGRAPH_STYLE_BORDER_RIGHT_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --named-style-paragraph-style-border-top-dash-style <NAMED_STYLE_PARAGRAPH_STYLE_BORDER_TOP_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --named-style-paragraph-style-direction <NAMED_STYLE_PARAGRAPH_STYLE_DIRECTION>
          The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since paragraph direction is not inherited. Choices: CONTENT_DIRECTION_UNSPECIFIED, LEFT_TO_RIGHT, RIGHT_TO_LEFT.

      --named-style-paragraph-style-heading-id <NAMED_STYLE_PARAGRAPH_STYLE_HEADING_ID>
          The heading ID of the paragraph. If empty, then this paragraph is not a heading. This property is read-only.

      --named-style-paragraph-style-indent-end-magnitude <NAMED_STYLE_PARAGRAPH_STYLE_INDENT_END_MAGNITUDE>
          The magnitude.

      --named-style-paragraph-style-indent-end-unit <NAMED_STYLE_PARAGRAPH_STYLE_INDENT_END_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --named-style-paragraph-style-indent-first-line-magnitude <NAMED_STYLE_PARAGRAPH_STYLE_INDENT_FIRST_LINE_MAGNITUDE>
          The magnitude.

      --named-style-paragraph-style-indent-first-line-unit <NAMED_STYLE_PARAGRAPH_STYLE_INDENT_FIRST_LINE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --named-style-paragraph-style-indent-start-magnitude <NAMED_STYLE_PARAGRAPH_STYLE_INDENT_START_MAGNITUDE>
          The magnitude.

      --named-style-paragraph-style-indent-start-unit <NAMED_STYLE_PARAGRAPH_STYLE_INDENT_START_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --named-style-paragraph-style-keep-lines-together[=<BOOL>]
          Whether all lines of the paragraph should be laid out on the same page or column if possible. If unset, the value is inherited from the parent.

      --named-style-paragraph-style-keep-with-next[=<BOOL>]
          Whether at least a part of this paragraph should be laid out on the same page or column as the next paragraph if possible. If unset, the value is inherited from the parent.

      --named-style-paragraph-style-line-spacing <NAMED_STYLE_PARAGRAPH_STYLE_LINE_SPACING>
          The amount of space between lines, as a percentage of normal, where normal is represented as 100.0. If unset, the value is inherited from the parent.

      --named-style-paragraph-style-named-style-type <NAMED_STYLE_PARAGRAPH_STYLE_NAMED_STYLE_TYPE>
          The named style type of the paragraph. Since updating the named style type affects other properties within ParagraphStyle, the named style type is applied before the other properties are updated. Choices: NAMED_STYLE_TYPE_UNSPECIFIED, NORMAL_TEXT, TITLE, SUBTITLE, HEADING_1, HEADING_2, HEADING_3, HEADING_4, HEADING_5, HEADING_6.

      --named-style-paragraph-style-page-break-before[=<BOOL>]
          Whether the current paragraph should always start at the beginning of a page. If unset, the value is inherited from the parent. Attempting to update page_break_before for paragraphs in unsupported regions, including Table, Header, Footer and Footnote, can result in an invalid document state that returns a 400 bad request error.

      --named-style-paragraph-style-space-above-magnitude <NAMED_STYLE_PARAGRAPH_STYLE_SPACE_ABOVE_MAGNITUDE>
          The magnitude.

      --named-style-paragraph-style-space-above-unit <NAMED_STYLE_PARAGRAPH_STYLE_SPACE_ABOVE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --named-style-paragraph-style-space-below-magnitude <NAMED_STYLE_PARAGRAPH_STYLE_SPACE_BELOW_MAGNITUDE>
          The magnitude.

      --named-style-paragraph-style-space-below-unit <NAMED_STYLE_PARAGRAPH_STYLE_SPACE_BELOW_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --named-style-paragraph-style-spacing-mode <NAMED_STYLE_PARAGRAPH_STYLE_SPACING_MODE>
          The spacing mode for the paragraph. Choices: SPACING_MODE_UNSPECIFIED, NEVER_COLLAPSE, COLLAPSE_LISTS.

      --named-style-paragraph-style-tab-stops <JSON>
          A list of the tab stops for this paragraph. The list of tab stops is not inherited. This property is read-only.

      --named-style-text-style-baseline-offset <NAMED_STYLE_TEXT_STYLE_BASELINE_OFFSET>
          The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. Changes in this field don't affect the `font_size`. Choices: BASELINE_OFFSET_UNSPECIFIED, NONE, SUPERSCRIPT, SUBSCRIPT.

      --named-style-text-style-bold[=<BOOL>]
          Whether or not the text is rendered as bold.

      --named-style-text-style-font-size-magnitude <NAMED_STYLE_TEXT_STYLE_FONT_SIZE_MAGNITUDE>
          The magnitude.

      --named-style-text-style-font-size-unit <NAMED_STYLE_TEXT_STYLE_FONT_SIZE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --named-style-text-style-italic[=<BOOL>]
          Whether or not the text is italicized.

      --named-style-text-style-link-bookmark-id <NAMED_STYLE_TEXT_STYLE_LINK_BOOKMARK_ID>
          The ID of a bookmark in this document. Legacy field: Instead, set includeTabsContent to `true` and use Link.bookmark for read and write operations. This field is only returned when includeTabsContent is set to `false` in documents containing a single tab and links to a bookmark within the singular tab. Otherwise, Link.bookmark is returned. If this field is used in a write request, the bookmark is considered to be from the tab ID specified in the request. If a tab ID is not specified in the request, it is considered to be from the first tab in the document.

      --named-style-text-style-link-heading-id <NAMED_STYLE_TEXT_STYLE_LINK_HEADING_ID>
          The ID of a heading in this document. Legacy field: Instead, set includeTabsContent to `true` and use Link.heading for read and write operations. This field is only returned when includeTabsContent is set to `false` in documents containing a single tab and links to a heading within the singular tab. Otherwise, Link.heading is returned. If this field is used in a write request, the heading is considered to be from the tab ID specified in the request. If a tab ID is not specified in the request, it is considered to be from the first tab in the document.

      --named-style-text-style-link-tab-id <NAMED_STYLE_TEXT_STYLE_LINK_TAB_ID>
          The ID of a tab in this document.

      --named-style-text-style-link-url <NAMED_STYLE_TEXT_STYLE_LINK_URL>
          An external URL.

      --named-style-text-style-small-caps[=<BOOL>]
          Whether or not the text is in small capital letters.

      --named-style-text-style-strikethrough[=<BOOL>]
          Whether or not the text is struck through.

      --named-style-text-style-underline[=<BOOL>]
          Whether or not the text is underlined.

      --named-style-text-style-weighted-font-family-font-family <NAMED_STYLE_TEXT_STYLE_WEIGHTED_FONT_FAMILY_FONT_FAMILY>
          The font family of the text. The font family can be any font from the Font menu in Docs or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`.

      --named-style-text-style-weighted-font-family-weight <NAMED_STYLE_TEXT_STYLE_WEIGHTED_FONT_FAMILY_WEIGHT>
          The weight of the font. This field can have any value that's a multiple of `100` between `100` and `900`, inclusive. This range corresponds to the numerical values described in the CSS 2.1 Specification, [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with non-numerical values disallowed. The default value is `400` ("normal"). The font weight makes up just one component of the rendered font weight. A combination of the `weight` and the text style's resolved `bold` value determine the rendered weight, after accounting for inheritance: * If the text is bold and the weight is less than `400`, the rendered weight is 400. * If the text is bold and the weight is greater than or equal to `400` but is less than `700`, the rendered weight is `700`. * If the weight is greater than or equal to `700`, the rendered weight is equal to the weight. * If the text is not bold, the rendered weight is equal to the weight.

      --tab-id <TAB_ID>
          The document tab to update. By default, the update is applied to the first tab.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-paragraph-style

```text
Updates the paragraph style at the specified range.

Usage: joog [OPTIONS] docs <ACCOUNT> update-paragraph-style [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `paragraph_style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example, to update the paragraph style's alignment property, set `fields` to `"alignment"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

      --range <JSON>
          Specifies a contiguous range of text.

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

      --paragraph-style <JSON>
          Styles that apply to a whole paragraph. Inherited paragraph styles are represented as unset fields in this message. A paragraph style's parent depends on where the paragraph style is defined: * The ParagraphStyle on a Paragraph inherits from the paragraph's corresponding named style type. * The ParagraphStyle on a named style inherits from the normal text named style. * The ParagraphStyle of the normal text named style inherits from the default paragraph style in the Docs editor. * The ParagraphStyle on a Paragraph element that's contained in a table may inherit its paragraph style from the table style. If the paragraph style does not inherit from a parent, unsetting fields will revert the style to a value matching the defaults in the Docs editor.

      --paragraph-style-alignment <PARAGRAPH_STYLE_ALIGNMENT>
          The text alignment for this paragraph. Choices: ALIGNMENT_UNSPECIFIED, START, CENTER, END, JUSTIFIED.

      --paragraph-style-avoid-widow-and-orphan[=<BOOL>]
          Whether to avoid widows and orphans for the paragraph. If unset, the value is inherited from the parent.

      --paragraph-style-border-between-dash-style <PARAGRAPH_STYLE_BORDER_BETWEEN_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --paragraph-style-border-between-padding-magnitude <PARAGRAPH_STYLE_BORDER_BETWEEN_PADDING_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-between-padding-unit <PARAGRAPH_STYLE_BORDER_BETWEEN_PADDING_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-between-width-magnitude <PARAGRAPH_STYLE_BORDER_BETWEEN_WIDTH_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-between-width-unit <PARAGRAPH_STYLE_BORDER_BETWEEN_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-bottom-dash-style <PARAGRAPH_STYLE_BORDER_BOTTOM_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --paragraph-style-border-bottom-padding-magnitude <PARAGRAPH_STYLE_BORDER_BOTTOM_PADDING_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-bottom-padding-unit <PARAGRAPH_STYLE_BORDER_BOTTOM_PADDING_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-bottom-width-magnitude <PARAGRAPH_STYLE_BORDER_BOTTOM_WIDTH_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-bottom-width-unit <PARAGRAPH_STYLE_BORDER_BOTTOM_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-left-dash-style <PARAGRAPH_STYLE_BORDER_LEFT_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --paragraph-style-border-left-padding-magnitude <PARAGRAPH_STYLE_BORDER_LEFT_PADDING_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-left-padding-unit <PARAGRAPH_STYLE_BORDER_LEFT_PADDING_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-left-width-magnitude <PARAGRAPH_STYLE_BORDER_LEFT_WIDTH_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-left-width-unit <PARAGRAPH_STYLE_BORDER_LEFT_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-right-dash-style <PARAGRAPH_STYLE_BORDER_RIGHT_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --paragraph-style-border-right-padding-magnitude <PARAGRAPH_STYLE_BORDER_RIGHT_PADDING_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-right-padding-unit <PARAGRAPH_STYLE_BORDER_RIGHT_PADDING_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-right-width-magnitude <PARAGRAPH_STYLE_BORDER_RIGHT_WIDTH_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-right-width-unit <PARAGRAPH_STYLE_BORDER_RIGHT_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-top-dash-style <PARAGRAPH_STYLE_BORDER_TOP_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --paragraph-style-border-top-padding-magnitude <PARAGRAPH_STYLE_BORDER_TOP_PADDING_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-top-padding-unit <PARAGRAPH_STYLE_BORDER_TOP_PADDING_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-border-top-width-magnitude <PARAGRAPH_STYLE_BORDER_TOP_WIDTH_MAGNITUDE>
          The magnitude.

      --paragraph-style-border-top-width-unit <PARAGRAPH_STYLE_BORDER_TOP_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-direction <PARAGRAPH_STYLE_DIRECTION>
          The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since paragraph direction is not inherited. Choices: CONTENT_DIRECTION_UNSPECIFIED, LEFT_TO_RIGHT, RIGHT_TO_LEFT.

      --paragraph-style-heading-id <PARAGRAPH_STYLE_HEADING_ID>
          The heading ID of the paragraph. If empty, then this paragraph is not a heading. This property is read-only.

      --paragraph-style-indent-end-magnitude <PARAGRAPH_STYLE_INDENT_END_MAGNITUDE>
          The magnitude.

      --paragraph-style-indent-end-unit <PARAGRAPH_STYLE_INDENT_END_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-indent-first-line-magnitude <PARAGRAPH_STYLE_INDENT_FIRST_LINE_MAGNITUDE>
          The magnitude.

      --paragraph-style-indent-first-line-unit <PARAGRAPH_STYLE_INDENT_FIRST_LINE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-indent-start-magnitude <PARAGRAPH_STYLE_INDENT_START_MAGNITUDE>
          The magnitude.

      --paragraph-style-indent-start-unit <PARAGRAPH_STYLE_INDENT_START_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-keep-lines-together[=<BOOL>]
          Whether all lines of the paragraph should be laid out on the same page or column if possible. If unset, the value is inherited from the parent.

      --paragraph-style-keep-with-next[=<BOOL>]
          Whether at least a part of this paragraph should be laid out on the same page or column as the next paragraph if possible. If unset, the value is inherited from the parent.

      --paragraph-style-line-spacing <PARAGRAPH_STYLE_LINE_SPACING>
          The amount of space between lines, as a percentage of normal, where normal is represented as 100.0. If unset, the value is inherited from the parent.

      --paragraph-style-named-style-type <PARAGRAPH_STYLE_NAMED_STYLE_TYPE>
          The named style type of the paragraph. Since updating the named style type affects other properties within ParagraphStyle, the named style type is applied before the other properties are updated. Choices: NAMED_STYLE_TYPE_UNSPECIFIED, NORMAL_TEXT, TITLE, SUBTITLE, HEADING_1, HEADING_2, HEADING_3, HEADING_4, HEADING_5, HEADING_6.

      --paragraph-style-page-break-before[=<BOOL>]
          Whether the current paragraph should always start at the beginning of a page. If unset, the value is inherited from the parent. Attempting to update page_break_before for paragraphs in unsupported regions, including Table, Header, Footer and Footnote, can result in an invalid document state that returns a 400 bad request error.

      --paragraph-style-space-above-magnitude <PARAGRAPH_STYLE_SPACE_ABOVE_MAGNITUDE>
          The magnitude.

      --paragraph-style-space-above-unit <PARAGRAPH_STYLE_SPACE_ABOVE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-space-below-magnitude <PARAGRAPH_STYLE_SPACE_BELOW_MAGNITUDE>
          The magnitude.

      --paragraph-style-space-below-unit <PARAGRAPH_STYLE_SPACE_BELOW_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --paragraph-style-spacing-mode <PARAGRAPH_STYLE_SPACING_MODE>
          The spacing mode for the paragraph. Choices: SPACING_MODE_UNSPECIFIED, NEVER_COLLAPSE, COLLAPSE_LISTS.

      --paragraph-style-tab-stops <JSON>
          A list of the tab stops for this paragraph. The list of tab stops is not inherited. This property is read-only.

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-section-style

```text
Updates the section style of the specified range.

Usage: joog [OPTIONS] docs <ACCOUNT> update-section-style [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `section_style` is implied and must not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the left margin, set `fields` to `"margin_left"`.

      --verbose
          Include advanced parameters in help

      --range <JSON>
          Specifies a contiguous range of text.

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

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --section-style <JSON>
          The styling that applies to a section.

      --section-style-column-properties <JSON>
          The section's columns properties. If empty, the section contains one column with the default properties in the Docs editor. A section can be updated to have no more than 3 columns. When updating this property, setting a concrete value is required. Unsetting this property will result in a 400 bad request error.

      --section-style-column-separator-style <SECTION_STYLE_COLUMN_SEPARATOR_STYLE>
          The style of column separators. This style can be set even when there's one column in the section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error. Choices: COLUMN_SEPARATOR_STYLE_UNSPECIFIED, NONE, BETWEEN_EACH_COLUMN.

      --section-style-content-direction <SECTION_STYLE_CONTENT_DIRECTION>
          The content direction of this section. If unset, the value defaults to LEFT_TO_RIGHT. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error. Choices: CONTENT_DIRECTION_UNSPECIFIED, LEFT_TO_RIGHT, RIGHT_TO_LEFT.

      --section-style-default-footer-id <SECTION_STYLE_DEFAULT_FOOTER_ID>
          The ID of the default footer. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_footer_id. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --section-style-default-header-id <SECTION_STYLE_DEFAULT_HEADER_ID>
          The ID of the default header. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_header_id. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --section-style-even-page-footer-id <SECTION_STYLE_EVEN_PAGE_FOOTER_ID>
          The ID of the footer used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the footers on even pages in the section. If it is false, the footers on even pages use the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_footer_id. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --section-style-even-page-header-id <SECTION_STYLE_EVEN_PAGE_HEADER_ID>
          The ID of the header used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the headers on even pages in the section. If it is false, the headers on even pages use the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_header_id. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --section-style-first-page-footer-id <SECTION_STYLE_FIRST_PAGE_FOOTER_ID>
          The ID of the footer used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the footer on the first page of the section. If it's false, the footer on the first page of the section uses the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_footer_id. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --section-style-first-page-header-id <SECTION_STYLE_FIRST_PAGE_HEADER_ID>
          The ID of the header used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the header on the first page of the section. If it's false, the header on the first page of the section uses the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_header_id. If DocumentMode is PAGELESS, this property will not be rendered. This property is read-only.

      --section-style-flip-page-orientation[=<BOOL>]
          Optional. Indicates whether to flip the dimensions of DocumentStyle's page_size for this section, which allows changing the page orientation between portrait and landscape. If unset, the value inherits from DocumentStyle's flip_page_orientation. If DocumentMode is PAGELESS, this property will not be rendered. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.

      --section-style-margin-bottom-magnitude <SECTION_STYLE_MARGIN_BOTTOM_MAGNITUDE>
          The magnitude.

      --section-style-margin-bottom-unit <SECTION_STYLE_MARGIN_BOTTOM_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --section-style-margin-footer-magnitude <SECTION_STYLE_MARGIN_FOOTER_MAGNITUDE>
          The magnitude.

      --section-style-margin-footer-unit <SECTION_STYLE_MARGIN_FOOTER_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --section-style-margin-header-magnitude <SECTION_STYLE_MARGIN_HEADER_MAGNITUDE>
          The magnitude.

      --section-style-margin-header-unit <SECTION_STYLE_MARGIN_HEADER_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --section-style-margin-left-magnitude <SECTION_STYLE_MARGIN_LEFT_MAGNITUDE>
          The magnitude.

      --section-style-margin-left-unit <SECTION_STYLE_MARGIN_LEFT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --section-style-margin-right-magnitude <SECTION_STYLE_MARGIN_RIGHT_MAGNITUDE>
          The magnitude.

      --section-style-margin-right-unit <SECTION_STYLE_MARGIN_RIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --section-style-margin-top-magnitude <SECTION_STYLE_MARGIN_TOP_MAGNITUDE>
          The magnitude.

      --section-style-margin-top-unit <SECTION_STYLE_MARGIN_TOP_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --section-style-page-number-start <SECTION_STYLE_PAGE_NUMBER_START>
          The page number from which to start counting the number of pages for this section. If unset, page numbering continues from the previous section. If the value is unset in the first SectionBreak, refer to DocumentStyle's page_number_start. If DocumentMode is PAGELESS, this property will not be rendered. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.

      --section-style-use-first-page-header-footer[=<BOOL>]
          Indicates whether to use the first page header / footer IDs for the first page of the section. If unset, it inherits from DocumentStyle's use_first_page_header_footer for the first section. If the value is unset for subsequent sectors, it should be interpreted as false. If DocumentMode is PAGELESS, this property will not be rendered. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-table-cell-style

```text
Updates the style of table cells.

Usage: joog [OPTIONS] docs <ACCOUNT> update-table-cell-style [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tableCellStyle` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the table cell background color, set `fields` to `"backgroundColor"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

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

      --table-cell-style <JSON>
          The style of a TableCell. Inherited table cell styles are represented as unset fields in this message. A table cell style can inherit from the table's style.

      --table-cell-style-border-bottom-dash-style <TABLE_CELL_STYLE_BORDER_BOTTOM_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --table-cell-style-border-bottom-width-magnitude <TABLE_CELL_STYLE_BORDER_BOTTOM_WIDTH_MAGNITUDE>
          The magnitude.

      --table-cell-style-border-bottom-width-unit <TABLE_CELL_STYLE_BORDER_BOTTOM_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-border-left-dash-style <TABLE_CELL_STYLE_BORDER_LEFT_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --table-cell-style-border-left-width-magnitude <TABLE_CELL_STYLE_BORDER_LEFT_WIDTH_MAGNITUDE>
          The magnitude.

      --table-cell-style-border-left-width-unit <TABLE_CELL_STYLE_BORDER_LEFT_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-border-right-dash-style <TABLE_CELL_STYLE_BORDER_RIGHT_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --table-cell-style-border-right-width-magnitude <TABLE_CELL_STYLE_BORDER_RIGHT_WIDTH_MAGNITUDE>
          The magnitude.

      --table-cell-style-border-right-width-unit <TABLE_CELL_STYLE_BORDER_RIGHT_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-border-top-dash-style <TABLE_CELL_STYLE_BORDER_TOP_DASH_STYLE>
          The dash style of the border. Choices: DASH_STYLE_UNSPECIFIED, SOLID, DOT, DASH.

      --table-cell-style-border-top-width-magnitude <TABLE_CELL_STYLE_BORDER_TOP_WIDTH_MAGNITUDE>
          The magnitude.

      --table-cell-style-border-top-width-unit <TABLE_CELL_STYLE_BORDER_TOP_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-column-span <TABLE_CELL_STYLE_COLUMN_SPAN>
          The column span of the cell. This property is read-only.

      --table-cell-style-content-alignment <TABLE_CELL_STYLE_CONTENT_ALIGNMENT>
          The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Docs editor. Choices: CONTENT_ALIGNMENT_UNSPECIFIED, CONTENT_ALIGNMENT_UNSUPPORTED, TOP, MIDDLE, BOTTOM.

      --table-cell-style-padding-bottom-magnitude <TABLE_CELL_STYLE_PADDING_BOTTOM_MAGNITUDE>
          The magnitude.

      --table-cell-style-padding-bottom-unit <TABLE_CELL_STYLE_PADDING_BOTTOM_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-padding-left-magnitude <TABLE_CELL_STYLE_PADDING_LEFT_MAGNITUDE>
          The magnitude.

      --table-cell-style-padding-left-unit <TABLE_CELL_STYLE_PADDING_LEFT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-padding-right-magnitude <TABLE_CELL_STYLE_PADDING_RIGHT_MAGNITUDE>
          The magnitude.

      --table-cell-style-padding-right-unit <TABLE_CELL_STYLE_PADDING_RIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-padding-top-magnitude <TABLE_CELL_STYLE_PADDING_TOP_MAGNITUDE>
          The magnitude.

      --table-cell-style-padding-top-unit <TABLE_CELL_STYLE_PADDING_TOP_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-cell-style-row-span <TABLE_CELL_STYLE_ROW_SPAN>
          The row span of the cell. This property is read-only.

      --table-range <JSON>
          A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with table cell location = (table_start_location, row = 0, column = 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]

      --table-range-column-span <TABLE_RANGE_COLUMN_SPAN>
          The column span of the table range.

      --table-range-row-span <TABLE_RANGE_ROW_SPAN>
          The row span of the table range.

      --table-range-table-cell-location-column-index <TABLE_RANGE_TABLE_CELL_LOCATION_COLUMN_INDEX>
          The zero-based column index. For example, the second column in the table has a column index of 1.

      --table-range-table-cell-location-row-index <TABLE_RANGE_TABLE_CELL_LOCATION_ROW_INDEX>
          The zero-based row index. For example, the second row in the table has a row index of 1.

      --table-range-table-cell-location-table-start-location-index <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-range-table-cell-location-table-start-location-segment-id <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-range-table-cell-location-table-start-location-tab-id <TABLE_RANGE_TABLE_CELL_LOCATION_TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --table-start-location <JSON>
          A particular location in the document.

      --table-start-location-index <TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-start-location-segment-id <TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-start-location-tab-id <TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-table-column-properties

```text
Updates the properties of columns in a table.

Usage: joog [OPTIONS] docs <ACCOUNT> update-table-column-properties [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tableColumnProperties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the column width, set `fields` to `"width"`.

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
          The list of zero-based column indices whose property should be updated. If no indices are specified, all columns will be updated.

      --table-column-properties <JSON>
          The properties of a column in a table.

      --table-column-properties-width-magnitude <TABLE_COLUMN_PROPERTIES_WIDTH_MAGNITUDE>
          The magnitude.

      --table-column-properties-width-type <TABLE_COLUMN_PROPERTIES_WIDTH_TYPE>
          The width type of the column. Choices: WIDTH_TYPE_UNSPECIFIED, EVENLY_DISTRIBUTED, FIXED_WIDTH.

      --table-column-properties-width-unit <TABLE_COLUMN_PROPERTIES_WIDTH_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-start-location <JSON>
          A particular location in the document.

      --table-start-location-index <TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-start-location-segment-id <TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-start-location-tab-id <TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-table-row-style

```text
Updates the row style in a table.

Usage: joog [OPTIONS] docs <ACCOUNT> update-table-row-style [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `tableRowStyle` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example to update the minimum row height, set `fields` to `"min_row_height"`.

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

      --row-indices <ROW_INDICES>...
          The list of zero-based row indices whose style should be updated. If no indices are specified, all rows will be updated.

      --table-row-style <JSON>
          Styles that apply to a table row.

      --table-row-style-min-row-height-magnitude <TABLE_ROW_STYLE_MIN_ROW_HEIGHT_MAGNITUDE>
          The magnitude.

      --table-row-style-min-row-height-unit <TABLE_ROW_STYLE_MIN_ROW_HEIGHT_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --table-row-style-prevent-overflow[=<BOOL>]
          Whether the row cannot overflow across page or column boundaries.

      --table-row-style-table-header[=<BOOL>]
          Whether the row is a table header.

      --table-start-location <JSON>
          A particular location in the document.

      --table-start-location-index <TABLE_START_LOCATION_INDEX>
          The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id.

      --table-start-location-segment-id <TABLE_START_LOCATION_SEGMENT_ID>
          The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body.

      --table-start-location-tab-id <TABLE_START_LOCATION_TAB_ID>
          The tab that the location is in. When omitted, the request is applied to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## update-text-style

```text
Updates the text style at the specified range.

Usage: joog [OPTIONS] docs <ACCOUNT> update-text-style [COMMAND_OPTIONS]

Mandatory parameters:
      --document-id <DOCUMENT_ID>
          The ID of the document to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-fields <BODY_FIELDS>
          The fields that should be updated. At least one field must be specified. The root `text_style` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field. For example, to update the text style to bold, set `fields` to `"bold"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset.

      --verbose
          Include advanced parameters in help

      --range <JSON>
          Specifies a contiguous range of text.

      --text-style-bold[=<BOOL>]
          Whether or not the text is rendered as bold.

      --text-style-italic[=<BOOL>]
          Whether or not the text is italicized.

      --text-style-underline[=<BOOL>]
          Whether or not the text is underlined.

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

      --range-end-index <RANGE_END_INDEX>
          The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-segment-id <RANGE_SEGMENT_ID>
          The ID of the header, footer, or footnote that this range is contained in. An empty segment ID signifies the document's body.

      --range-start-index <RANGE_START_INDEX>
          The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges.

      --range-tab-id <RANGE_TAB_ID>
          The tab that contains this range. When omitted, the request applies to the first tab. In a document containing a single tab: - If provided, must match the singular tab's ID. - If omitted, the request applies to the singular tab. In a document containing multiple tabs: - If provided, the request applies to the specified tab. - If omitted, the request applies to the first tab in the document.

      --text-style <JSON>
          Represents the styling that can be applied to text. Inherited text styles are represented as unset fields in this message. A text style's parent depends on where the text style is defined: * The TextStyle of text in a Paragraph inherits from the paragraph's corresponding named style type. * The TextStyle on a named style inherits from the normal text named style. * The TextStyle of the normal text named style inherits from the default text style in the Docs editor. * The TextStyle on a Paragraph element that's contained in a table may inherit its text style from the table style. If the text style does not inherit from a parent, unsetting fields will revert the style to a value matching the defaults in the Docs editor.

      --text-style-baseline-offset <TEXT_STYLE_BASELINE_OFFSET>
          The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. Changes in this field don't affect the `font_size`. Choices: BASELINE_OFFSET_UNSPECIFIED, NONE, SUPERSCRIPT, SUBSCRIPT.

      --text-style-font-size-magnitude <TEXT_STYLE_FONT_SIZE_MAGNITUDE>
          The magnitude.

      --text-style-font-size-unit <TEXT_STYLE_FONT_SIZE_UNIT>
          The units for magnitude. Choices: UNIT_UNSPECIFIED, PT.

      --text-style-link-bookmark-id <TEXT_STYLE_LINK_BOOKMARK_ID>
          The ID of a bookmark in this document.

      --text-style-link-bookmark-tab-id <TEXT_STYLE_LINK_BOOKMARK_TAB_ID>
          The ID of the tab containing this bookmark.

      --text-style-link-heading-id <TEXT_STYLE_LINK_HEADING_ID>
          The ID of a heading in this document.

      --text-style-link-heading-tab-id <TEXT_STYLE_LINK_HEADING_TAB_ID>
          The ID of the tab containing this heading.

      --text-style-link-tab-id <TEXT_STYLE_LINK_TAB_ID>
          The ID of a tab in this document.

      --text-style-link-url <TEXT_STYLE_LINK_URL>
          An external URL.

      --text-style-small-caps[=<BOOL>]
          Whether or not the text is in small capital letters.

      --text-style-strikethrough[=<BOOL>]
          Whether or not the text is struck through.

      --text-style-weighted-font-family-font-family <TEXT_STYLE_WEIGHTED_FONT_FAMILY_FONT_FAMILY>
          The font family of the text. The font family can be any font from the Font menu in Docs or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`.

      --text-style-weighted-font-family-weight <TEXT_STYLE_WEIGHTED_FONT_FAMILY_WEIGHT>
          The weight of the font. This field can have any value that's a multiple of `100` between `100` and `900`, inclusive. This range corresponds to the numerical values described in the CSS 2.1 Specification, [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with non-numerical values disallowed. The default value is `400` ("normal"). The font weight makes up just one component of the rendered font weight. A combination of the `weight` and the text style's resolved `bold` value determine the rendered weight, after accounting for inheritance: * If the text is bold and the weight is less than `400`, the rendered weight is 400. * If the text is bold and the weight is greater than or equal to `400` but is less than `700`, the rendered weight is `700`. * If the weight is greater than or equal to `700`, the rendered weight is equal to the weight. * If the text is not bold, the rendered weight is equal to the weight.

      --write-control <JSON>
          Provides control over how write requests are executed.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
