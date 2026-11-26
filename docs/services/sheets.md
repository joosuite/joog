# Google Sheets methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `create` | Creates a spreadsheet, returning the newly created spreadsheet. |
| `get` | Returns the spreadsheet at the given ID |
| `sheets.copy-to` | Copies a single sheet from a spreadsheet to another spreadsheet |
| `values.append` | Appends values to a spreadsheet |
| `values.batch-clear` | Clears one or more ranges of values from a spreadsheet |
| `values.batch-clear-by-data-filter` | Clears one or more ranges of values from a spreadsheet |
| `values.batch-get` | Returns one or more ranges of values from a spreadsheet |
| `values.batch-get-by-data-filter` | Returns one or more ranges of values that match the specified data filters |
| `values.batch-update` | Sets values in one or more ranges of a spreadsheet |
| `values.batch-update-by-data-filter` | Sets values in one or more ranges of a spreadsheet |
| `values.clear` | Clears values from a spreadsheet |
| `values.get` | Returns a range of values from a spreadsheet |
| `values.update` | Sets values in a range of a spreadsheet |

## create

```text
Creates a spreadsheet, returning the newly created spreadsheet.

Usage: joog [OPTIONS] sheets <ACCOUNT> create [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --title <TITLE>
          Title.

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

      --comments <JSON>
          The comment threads associated with the spreadsheet.

      --data-sources <JSON>
          A list of external data sources connected with the spreadsheet.

      --developer-metadata <JSON>
          The developer metadata associated with a spreadsheet.

      --named-ranges <JSON>
          The named ranges defined in a spreadsheet.

      --properties <JSON>
          Properties of a spreadsheet.

      --properties-auto-recalc <PROPERTIES_AUTO_RECALC>
          The amount of time to wait before volatile functions are recalculated. Choices: RECALCULATION_INTERVAL_UNSPECIFIED, ON_CHANGE, MINUTE, HOUR.

      --properties-default-format-background-color-alpha <PROPERTIES_DEFAULT_FORMAT_BACKGROUND_COLOR_ALPHA>
          The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)` This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0).

      --properties-default-format-background-color-blue <PROPERTIES_DEFAULT_FORMAT_BACKGROUND_COLOR_BLUE>
          The amount of blue in the color as a value in the interval [0, 1].

      --properties-default-format-background-color-green <PROPERTIES_DEFAULT_FORMAT_BACKGROUND_COLOR_GREEN>
          The amount of green in the color as a value in the interval [0, 1].

      --properties-default-format-background-color-red <PROPERTIES_DEFAULT_FORMAT_BACKGROUND_COLOR_RED>
          The amount of red in the color as a value in the interval [0, 1].

      --properties-default-format-background-color-style-theme-color <PROPERTIES_DEFAULT_FORMAT_BACKGROUND_COLOR_STYLE_THEME_COLOR>
          Theme color. Choices: THEME_COLOR_TYPE_UNSPECIFIED, TEXT, BACKGROUND, ACCENT1, ACCENT2, ACCENT3, ACCENT4, ACCENT5, ACCENT6, LINK.

      --properties-default-format-horizontal-alignment <PROPERTIES_DEFAULT_FORMAT_HORIZONTAL_ALIGNMENT>
          The horizontal alignment of the value in the cell. Choices: HORIZONTAL_ALIGN_UNSPECIFIED, LEFT, CENTER, RIGHT.

      --properties-default-format-hyperlink-display-type <PROPERTIES_DEFAULT_FORMAT_HYPERLINK_DISPLAY_TYPE>
          If one exists, how a hyperlink should be displayed in the cell. Choices: HYPERLINK_DISPLAY_TYPE_UNSPECIFIED, LINKED, PLAIN_TEXT.

      --properties-default-format-number-format-pattern <PROPERTIES_DEFAULT_FORMAT_NUMBER_FORMAT_PATTERN>
          Pattern string used for formatting. If not set, a default pattern based on the spreadsheet's locale will be used if necessary for the given type. See the [Date and Number Formats guide](https://developers.google.com/workspace/sheets/api/guides/formats) for more information about the supported patterns.

      --properties-default-format-number-format-type <PROPERTIES_DEFAULT_FORMAT_NUMBER_FORMAT_TYPE>
          The type of the number format. When writing, this field must be set. Choices: NUMBER_FORMAT_TYPE_UNSPECIFIED, TEXT, NUMBER, PERCENT, CURRENCY, DATE, TIME, DATE_TIME, SCIENTIFIC.

      --properties-default-format-padding-bottom <PROPERTIES_DEFAULT_FORMAT_PADDING_BOTTOM>
          The bottom padding of the cell.

      --properties-default-format-padding-left <PROPERTIES_DEFAULT_FORMAT_PADDING_LEFT>
          The left padding of the cell.

      --properties-default-format-padding-right <PROPERTIES_DEFAULT_FORMAT_PADDING_RIGHT>
          The right padding of the cell.

      --properties-default-format-padding-top <PROPERTIES_DEFAULT_FORMAT_PADDING_TOP>
          The top padding of the cell.

      --properties-default-format-text-direction <PROPERTIES_DEFAULT_FORMAT_TEXT_DIRECTION>
          The direction of the text in the cell. Choices: TEXT_DIRECTION_UNSPECIFIED, LEFT_TO_RIGHT, RIGHT_TO_LEFT.

      --properties-default-format-text-format-bold[=<BOOL>]
          True if the text is bold.

      --properties-default-format-text-format-font-family <PROPERTIES_DEFAULT_FORMAT_TEXT_FORMAT_FONT_FAMILY>
          The font family.

      --properties-default-format-text-format-font-size <PROPERTIES_DEFAULT_FORMAT_TEXT_FORMAT_FONT_SIZE>
          The size of the font.

      --properties-default-format-text-format-italic[=<BOOL>]
          True if the text is italicized.

      --properties-default-format-text-format-strikethrough[=<BOOL>]
          True if the text has a strikethrough.

      --properties-default-format-text-format-underline[=<BOOL>]
          True if the text is underlined.

      --properties-default-format-text-rotation-angle <PROPERTIES_DEFAULT_FORMAT_TEXT_ROTATION_ANGLE>
          The angle between the standard orientation and the desired orientation. Measured in degrees. Valid values are between -90 and 90. Positive angles are angled upwards, negative are angled downwards. Note: For LTR text direction positive angles are in the counterclockwise direction, whereas for RTL they are in the clockwise direction

      --properties-default-format-text-rotation-vertical[=<BOOL>]
          If true, text reads top to bottom, but the orientation of individual characters is unchanged. For example: | V | | e | | r | | t | | i | | c | | a | | l |

      --properties-default-format-vertical-alignment <PROPERTIES_DEFAULT_FORMAT_VERTICAL_ALIGNMENT>
          The vertical alignment of the value in the cell. Choices: VERTICAL_ALIGN_UNSPECIFIED, TOP, MIDDLE, BOTTOM.

      --properties-default-format-wrap-strategy <PROPERTIES_DEFAULT_FORMAT_WRAP_STRATEGY>
          The wrap strategy for the value in the cell. Choices: WRAP_STRATEGY_UNSPECIFIED, OVERFLOW_CELL, LEGACY_WRAP, CLIP, WRAP.

      --properties-import-functions-external-url-access-allowed[=<BOOL>]
          Whether to allow external URL access for image and import functions. Read only when true. When false, you can set to true. This value will be bypassed and always return true if the admin has enabled the [allowlisting feature](https://support.google.com/a?p=url_allowlist).

      --properties-iterative-calculation-settings-convergence-threshold <PROPERTIES_ITERATIVE_CALCULATION_SETTINGS_CONVERGENCE_THRESHOLD>
          When iterative calculation is enabled and successive results differ by less than this threshold value, the calculation rounds stop.

      --properties-iterative-calculation-settings-max-iterations <PROPERTIES_ITERATIVE_CALCULATION_SETTINGS_MAX_ITERATIONS>
          When iterative calculation is enabled, the maximum number of calculation rounds to perform.

      --properties-locale <PROPERTIES_LOCALE>
          The locale of the spreadsheet in one of the following formats: * an ISO 639-1 language code such as `en` * an ISO 639-2 language code such as `fil`, if no 639-1 code exists * a combination of the ISO language code and country code, such as `en_US` Note: when updating this field, not all locales/languages are supported.

      --properties-spreadsheet-theme-primary-font-family <PROPERTIES_SPREADSHEET_THEME_PRIMARY_FONT_FAMILY>
          Name of the primary font family.

      --properties-spreadsheet-theme-theme-colors <JSON>
          The spreadsheet theme color pairs. To update you must provide all theme color pairs.

      --properties-time-zone <PROPERTIES_TIME_ZONE>
          The time zone of the spreadsheet, in CLDR format such as `America/New_York`. If the time zone isn't recognized, this may be a custom time zone such as `GMT-07:00`.

      --sheets <JSON>
          The sheets that are part of a spreadsheet.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet. This field is read-only.

      --spreadsheet-url <SPREADSHEET_URL>
          The url of the spreadsheet. This field is read-only.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## get

```text
Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. By default, data within grids is not returned. You can include grid data in one of 2 ways: * Specify a [field mask](https://developers.google.com/workspace/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData URL parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, as a best practice, retrieve only the specific spreadsheet fields that you want. To retrieve only subsets of spreadsheet data, use the ranges URL parameter. Ranges are specified using [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). You can define a single cell (for example, `A1`) or multiple cells (for example, `A1:D5`). You can also get cells from other sheets within the same spreadsheet (for example, `Sheet2!A1:C4`) or retrieve multiple ranges at once (for example, `?ranges=A1:D5&ranges=Sheet2!A1:C4`). Limiting the range returns only the portions of the spreadsheet that intersect the requested ranges.

Usage: joog [OPTIONS] sheets <ACCOUNT> get [COMMAND_OPTIONS]

Mandatory parameters:
      --spreadsheet-id <SPREADSHEET_ID>
          The spreadsheet to request.

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
          The comments view mode to apply to the spreadsheet. This allows viewing the spreadsheet with comments omitted or included. If one is not specified, COMMENTS_VIEW_MODE_OMITTED is used. Choices: COMMENTS_VIEW_MODE_UNSPECIFIED, COMMENTS_VIEW_MODE_DEFAULT_FOR_CURRENT_ACCESS, COMMENTS_VIEW_MODE_OMITTED, COMMENTS_VIEW_MODE_INCLUDED.

      --exclude-tables-in-banded-ranges[=<BOOL>]
          True if tables should be excluded in the banded ranges. False if not set.

      --include-grid-data[=<BOOL>]
          True if grid data should be returned. This parameter is ignored if a field mask was set in the request.

      --ranges <RANGES>...
          The ranges to retrieve from the spreadsheet.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## sheets.copy-to

```text
Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.

Usage: joog [OPTIONS] sheets <ACCOUNT> sheets.copy-to [COMMAND_OPTIONS]

Mandatory parameters:
      --sheet-id <SHEET_ID>
          The ID of the sheet to copy.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet containing the sheet to copy.

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

      --destination-spreadsheet-id <DESTINATION_SPREADSHEET_ID>
          The ID of the spreadsheet to copy the sheet to.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.append

```text
Appends values to a spreadsheet. The input range is used to search for existing data and find a "table" within that range. Values will be appended to the next row of the table, starting with the first column of the table. See the [guide](https://developers.google.com/workspace/sheets/api/guides/values#appending_values) and [sample code](https://developers.google.com/workspace/sheets/api/samples/writing#append_values) for specific details of how tables are detected and data is appended. The caller must specify the spreadsheet ID, range, and a valueInputOption. The `valueInputOption` only controls how the input data will be added to the sheet (column-wise or row-wise), it does not influence what cell the data starts being written to.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.append [COMMAND_OPTIONS]

Mandatory parameters:
      --range <RANGE>
          The [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell) of a range to search for a logical table of data. Values are appended after the last row of the table.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-range <BODY_RANGE>
          The range the values cover, in [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended.

      --verbose
          Include advanced parameters in help

      --value-input-option <VALUE_INPUT_OPTION>
          How the input data should be interpreted. Choices: INPUT_VALUE_OPTION_UNSPECIFIED, RAW, USER_ENTERED.

      --values <JSON>
          The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell. For output, empty trailing rows and columns will not be included. For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string.

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

      --include-values-in-response[=<BOOL>]
          Determines if the update response should include the values of the cells that were appended. By default, responses do not include the updated values.

      --insert-data-option <INSERT_DATA_OPTION>
          How the input data should be inserted. Choices: OVERWRITE, INSERT_ROWS.

      --major-dimension <MAJOR_DIMENSION>
          The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS. Choices: DIMENSION_UNSPECIFIED, ROWS, COLUMNS.

      --response-date-time-render-option <RESPONSE_DATE_TIME_RENDER_OPTION>
          Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. Choices: SERIAL_NUMBER, FORMATTED_STRING.

      --response-value-render-option <RESPONSE_VALUE_RENDER_OPTION>
          Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE. Choices: FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.batch-clear

```text
Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges. Only values are cleared -- all other properties of the cell (such as formatting and data validation) are kept.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.batch-clear [COMMAND_OPTIONS]

Mandatory parameters:
      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to update.

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

      --ranges <RANGES>...
          The ranges to clear, in [A1 notation or R1C1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.batch-clear-by-data-filter

```text
Clears one or more ranges of values from a spreadsheet. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc.) are kept.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.batch-clear-by-data-filter [COMMAND_OPTIONS]

Mandatory parameters:
      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to update.

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

      --data-filters <JSON>
          The DataFilters used to determine which ranges to clear.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.batch-get

```text
Returns one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.batch-get [COMMAND_OPTIONS]

Mandatory parameters:
      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to retrieve data from.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --value-render-option <VALUE_RENDER_OPTION>
          How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE. Choices: FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA.

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

      --date-time-render-option <DATE_TIME_RENDER_OPTION>
          How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. Choices: SERIAL_NUMBER, FORMATTED_STRING.

      --major-dimension <MAJOR_DIMENSION>
          The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `ranges=["A1:B2"],majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `ranges=["A1:B2"],majorDimension=COLUMNS` returns `[[1,3],[2,4]]`. Choices: DIMENSION_UNSPECIFIED, ROWS, COLUMNS.

      --ranges <RANGES>...
          The [A1 notation or R1C1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell) of the range to retrieve values from.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.batch-get-by-data-filter

```text
Returns one or more ranges of values that match the specified data filters. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). The caller must specify the spreadsheet ID and one or more DataFilters. Ranges that match any of the data filters in the request will be returned.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.batch-get-by-data-filter [COMMAND_OPTIONS]

Mandatory parameters:
      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to retrieve data from.

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

      --body <JSON>
          Inline JSON request body

      --data-filters <JSON>
          The data filters used to match the ranges of values to retrieve. Ranges that match any of the specified data filters are included in the response.

      --date-time-render-option <DATE_TIME_RENDER_OPTION>
          How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. Choices: SERIAL_NUMBER, FORMATTED_STRING.

      --major-dimension <MAJOR_DIMENSION>
          The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then a request that selects that range and sets `majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas a request that sets `majorDimension=COLUMNS` returns `[[1,3],[2,4]]`. Choices: DIMENSION_UNSPECIFIED, ROWS, COLUMNS.

      --value-render-option <VALUE_RENDER_OPTION>
          How values should be represented in the output. The default render option is FORMATTED_VALUE. Choices: FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.batch-update

```text
Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more ValueRanges.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.batch-update [COMMAND_OPTIONS]

Mandatory parameters:
      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to update.

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

      --data <JSON>
          The new values to apply to the spreadsheet.

      --include-values-in-response[=<BOOL>]
          Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The `updatedData` field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).

      --response-date-time-render-option <RESPONSE_DATE_TIME_RENDER_OPTION>
          Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. Choices: SERIAL_NUMBER, FORMATTED_STRING.

      --response-value-render-option <RESPONSE_VALUE_RENDER_OPTION>
          Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE. Choices: FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA.

      --value-input-option <VALUE_INPUT_OPTION>
          How the input data should be interpreted. Choices: INPUT_VALUE_OPTION_UNSPECIFIED, RAW, USER_ENTERED.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.batch-update-by-data-filter

```text
Sets values in one or more ranges of a spreadsheet. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.batch-update-by-data-filter [COMMAND_OPTIONS]

Mandatory parameters:
      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to update.

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

      --data <JSON>
          The new values to apply to the spreadsheet. If more than one range is matched by the specified DataFilter the specified values are applied to all of those ranges.

      --include-values-in-response[=<BOOL>]
          Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The `updatedData` field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).

      --response-date-time-render-option <RESPONSE_DATE_TIME_RENDER_OPTION>
          Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. Choices: SERIAL_NUMBER, FORMATTED_STRING.

      --response-value-render-option <RESPONSE_VALUE_RENDER_OPTION>
          Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE. Choices: FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA.

      --value-input-option <VALUE_INPUT_OPTION>
          How the input data should be interpreted. Choices: INPUT_VALUE_OPTION_UNSPECIFIED, RAW, USER_ENTERED.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.clear

```text
Clears values from a spreadsheet. The caller must specify the spreadsheet ID and range. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.clear [COMMAND_OPTIONS]

Mandatory parameters:
      --range <RANGE>
          The [A1 notation or R1C1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell) of the values to clear.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to update.

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

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.get

```text
Returns a range of values from a spreadsheet. The caller must specify the spreadsheet ID and a range.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.get [COMMAND_OPTIONS]

Mandatory parameters:
      --range <RANGE>
          The [A1 notation or R1C1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell) of the range to retrieve values from.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to retrieve data from.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --value-render-option <VALUE_RENDER_OPTION>
          How values should be represented in the output. The default render option is FORMATTED_VALUE. Choices: FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA.

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

      --date-time-render-option <DATE_TIME_RENDER_OPTION>
          How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. Choices: SERIAL_NUMBER, FORMATTED_STRING.

      --major-dimension <MAJOR_DIMENSION>
          The major dimension that results should use. For example, if the spreadsheet data in Sheet1 is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=Sheet1!A1:B2?majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=Sheet1!A1:B2?majorDimension=COLUMNS` returns `[[1,3],[2,4]]`. Choices: DIMENSION_UNSPECIFIED, ROWS, COLUMNS.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## values.update

```text
Sets values in a range of a spreadsheet. The caller must specify the spreadsheet ID, range, and a valueInputOption.

Usage: joog [OPTIONS] sheets <ACCOUNT> values.update [COMMAND_OPTIONS]

Mandatory parameters:
      --range <RANGE>
          The [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell) of the values to update.

      --spreadsheet-id <SPREADSHEET_ID>
          The ID of the spreadsheet to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --body-range <BODY_RANGE>
          The range the values cover, in [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended.

      --verbose
          Include advanced parameters in help

      --value-input-option <VALUE_INPUT_OPTION>
          How the input data should be interpreted. Choices: INPUT_VALUE_OPTION_UNSPECIFIED, RAW, USER_ENTERED.

      --values <JSON>
          The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell. For output, empty trailing rows and columns will not be included. For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string.

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

      --include-values-in-response[=<BOOL>]
          Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).

      --major-dimension <MAJOR_DIMENSION>
          The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS. Choices: DIMENSION_UNSPECIFIED, ROWS, COLUMNS.

      --response-date-time-render-option <RESPONSE_DATE_TIME_RENDER_OPTION>
          Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. Choices: SERIAL_NUMBER, FORMATTED_STRING.

      --response-value-render-option <RESPONSE_VALUE_RENDER_OPTION>
          Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE. Choices: FORMATTED_VALUE, UNFORMATTED_VALUE, FORMULA.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
