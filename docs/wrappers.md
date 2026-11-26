# Short command wrappers

Run `make build` to create the wrappers in `bin/`. `make install` installs them
beside `joog`. Add that directory to your `PATH`.

`joog` and its wrappers are native executables. Installing and running them
requires no Python. Building from source requires Rust and Cargo; `make build`
and `make install` also create the wrapper entry points with a native installer.

```sh
jmail ACCOUNT to:lara@example.com body:"Hello\nHow are you?"
jcal ACCOUNT add "Lunch" at:"tomorrow 12:30" for:1h with:lara@example.com
jdocs ACCOUNT append DOCUMENT text-file:notes.txt
```

The short commands and `key:value` parameters belong to the wrappers. Existing
`joog` commands keep their syntax. For example, `joog mail ACCOUNT messages.send
--to lara@example.com --text Hello` remains available.

Quote values containing spaces. Repeat parameters to supply several recipients,
attachments or rows. Inline text accepts `\n`, `\t`, and `\\`.
Use `--` before positional text that looks like a named parameter.

Use `file:PATH` to read parameters from a JSON object. File property names use
camelCase, such as `bodyFile` and `matchCase`. Explicit parameters replace file
values, including an entire repeated list. Explicit inline text replaces its
corresponding file input. Required values are checked after combining the inputs.
Select the command definition under `$defs` in
[wrappers.schema.json](../schemas/wrappers.schema.json) to validate complete input.

Lists default to 20 results unless the command describes a date window. Use
`limit:all` to fetch all results. Downloads and exports require `out:PATH` and
never overwrite existing files. CSV imports write literal values, including
text beginning with `=`. Task due dates use the computer's local date. Calendar
dates use the selected calendar's timezone. Ambiguous times at clock changes
require a timestamp with an explicit offset.

Use `WRAPPER --help` to list its commands, `WRAPPER ACCOUNT COMMAND --help` for
common parameters, and `--help --verbose` for advanced parameters. `json:true`
changes only the output format. Both formats contain the same records and fields,
including message IDs and thread IDs in mail results. Advanced `home:PATH` and
`timeout:SECONDS` select the credential
directory and request timeout.

The wrappers use your existing Joog sign-in. Document, spreadsheet, presentation,
and form listing use Drive. Document sharing and native file exports also use
Drive. Authorize it with the service when needed, for example
`joog auth ACCOUNT login docs,drive`. Photos commands access content created by
this application.

Compound commands can perform several requests. If a later request fails, the
error identifies an already-created resource where available. Check that
resource before retrying a creation command.

## jmail

Read, compose, and organise mail. The default command is `inbox`.

| Command | Description |
| --- | --- |
| `inbox` | Show recent inbox messages. |
| `find` | Search messages. |
| `read` | Read a message and list its attachments. |
| `send` | Compose and send a message. You can omit send when supplying to:, cc:, or bcc:. |
| `reply` | Reply to a message with its threading headers. |
| `forward` | Forward a message, preserving the original MIME message and attachments. |
| `draft` | Save a draft. Recipients may be omitted. |
| `drafts` | List saved drafts. |
| `send-draft` | Send an existing draft. |
| `trash` | Move a message to Trash. |
| `mark` | Change a message's read or star state. |

### inbox

Show recent inbox messages.

```text
jmail ACCOUNT inbox [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `unread:` | no | Show only unread messages. Default: `false`. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### find

Search messages.

```text
jmail ACCOUNT find QUERY [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `query:` | yes | Gmail search query. |
| `from:` | no | Sender email address. |
| `after:` | no | YYYY-MM-DD, today, or tomorrow. |
| `has:` | no | Require an attachment. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### read

Read a message and list its attachments.

```text
jmail ACCOUNT read MESSAGE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Message ID. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### send

Compose and send a message. You can omit send when supplying to:, cc:, or bcc:.

```text
jmail ACCOUNT send [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `to:` | no | Email addresses. Repeat for multiple values. |
| `cc:` | no | Email addresses. Repeat for multiple values. |
| `bcc:` | no | Email addresses. Repeat for multiple values. |
| `subject:` | no | Message subject. |
| `body:` | no | Inline text. Use \n for a new line. |
| `body-file:` | no | Read the message body from a UTF-8 file. |
| `attach:` | no | Local attachment paths. Repeat for multiple values. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Provide at least one recipient with to:, cc:, or bcc:.

Use body: for inline text or body-file: to read a file.

### reply

Reply to a message with its threading headers.

```text
jmail ACCOUNT reply MESSAGE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `body:` | no | Inline text. Use \n for a new line. |
| `body-file:` | no | Existing local file, up to 1 GiB. |
| `all:` | no | Reply to all original recipients, excluding your account. Default: `false`. |
| `attach:` | no | Local attachment files. Repeat for multiple values. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Write a reply with body: or read it from body-file:.

### forward

Forward a message, preserving the original MIME message and attachments.

```text
jmail ACCOUNT forward MESSAGE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `to:` | yes | Email addresses. Repeat for multiple values. |
| `body:` | no | Inline text. Use \n for a new line. |
| `body-file:` | no | Existing local file, up to 1 GiB. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Write text with body: or read it from body-file:.

### draft

Save a draft. Recipients may be omitted.

```text
jmail ACCOUNT draft [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `to:` | no | Email addresses. Repeat for multiple values. |
| `cc:` | no | Email addresses. Repeat for multiple values. |
| `bcc:` | no | Email addresses. Repeat for multiple values. |
| `subject:` | no | Message subject. |
| `body:` | no | Inline text. Use \n for a new line. |
| `body-file:` | no | Existing local file, up to 1 GiB. |
| `attach:` | no | Local attachment files. Repeat for multiple values. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Write text with body: or read it from body-file:.

### drafts

List saved drafts.

```text
jmail ACCOUNT drafts [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### send-draft

Send an existing draft.

```text
jmail ACCOUNT send-draft DRAFT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### trash

Move a message to Trash.

```text
jmail ACCOUNT trash MESSAGE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### mark

Change a message's read or star state.

```text
jmail ACCOUNT mark MESSAGE STATE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `state:` | yes | New message state. Choices: `read`, `unread`, `starred`, `unstarred`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jdrive

Find, transfer, and share files. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | List files in a folder. Defaults to the root folder. |
| `find` | Find files by name or content. |
| `info` | Show file metadata, owners, location, and sharing. |
| `get` | Download a file or export a Google document. |
| `put` | Upload a local file. |
| `mkdir` | Create a folder. |
| `move` | Move or rename a file, preserving its other metadata. |
| `copy` | Copy a file. |
| `share` | Grant someone access to a file. |
| `trash` | Move a file to Trash. |
| `restore` | Restore a file from Trash. |

### ls

List files in a folder. Defaults to the root folder.

```text
jdrive ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `folder:` | no | File ID or Google Drive URL. Default: `"root"`. |
| `type:` | no | folder, document, spreadsheet, presentation, image, pdf, or a MIME type. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### find

Find files by name or content.

```text
jdrive ACCOUNT find TEXT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `query:` | yes | Search text. |
| `folder:` | no | File ID or Google Drive URL. |
| `type:` | no | Item type. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### info

Show file metadata, owners, location, and sharing.

```text
jdrive ACCOUNT info FILE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | File ID or Google Drive URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### get

Download a file or export a Google document.

```text
jdrive ACCOUNT get FILE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | File ID or Google Drive URL. |
| `out:` | yes | Save to this new file. Existing files are never overwritten. |
| `format:` | no | Export format for native Google files. Defaults to pdf. Choices: `pdf`, `docx`, `xlsx`, `pptx`, `csv`, `txt`, `html`, `odt`, `ods`, `odp`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### put

Upload a local file.

```text
jdrive ACCOUNT put PATH [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `path:` | yes | Existing local file, up to 1 GiB. |
| `folder:` | no | File ID or Google Drive URL. |
| `name:` | no | Name. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### mkdir

Create a folder.

```text
jdrive ACCOUNT mkdir NAME [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `name:` | yes | Name. |
| `folder:` | no | File ID or Google Drive URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### move

Move or rename a file, preserving its other metadata.

```text
jdrive ACCOUNT move FILE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | File ID or Google Drive URL. |
| `to:` | no | Destination folder. |
| `name:` | no | Name. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Provide to: to move the file or name: to rename it.

### copy

Copy a file.

```text
jdrive ACCOUNT copy FILE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | File ID or Google Drive URL. |
| `to:` | no | File ID or Google Drive URL. |
| `name:` | no | Name. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### share

Grant someone access to a file.

```text
jdrive ACCOUNT share FILE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | File ID or Google Drive URL. |
| `to:` | yes | Destination or recipient. |
| `role:` | no | Access granted to the recipient. Choices: `reader`, `writer`, `commenter`. Default: `"reader"`. |
| `notify:` | no | Send email notifications to affected people. Default: `true`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### trash

Move a file to Trash.

```text
jdrive ACCOUNT trash FILE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | File ID or Google Drive URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### restore

Restore a file from Trash.

```text
jdrive ACCOUNT restore FILE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | File ID or Google Drive URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jcal

Manage events and find available times. The default command is `today`.

| Command | Description |
| --- | --- |
| `today` | Show today's events in calendar time. |
| `week` | Show this week's events, Monday through Sunday. |
| `find` | Find events. Date bounds use calendar time. |
| `show` | Show an event and attendee responses. |
| `add` | Create a timed or all-day event. |
| `move` | Reschedule an event. Its duration is preserved unless for: is supplied. |
| `cancel` | Cancel an event. |
| `free` | Find common free slots from 09:00 to 18:00 in calendar time. |
| `calendars` | List available calendars. |

### today

Show today's events in calendar time.

```text
jcal ACCOUNT today [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `timezone:` | no | IANA timezone, such as Europe/Paris. Defaults to the calendar timezone. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### week

Show this week's events, Monday through Sunday.

```text
jcal ACCOUNT week [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `timezone:` | no | IANA timezone, such as Europe/Paris. Defaults to the calendar timezone. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### find

Find events. Date bounds use calendar time.

```text
jcal ACCOUNT find TEXT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `query:` | yes | Search text. |
| `from:` | no | YYYY-MM-DD, today, or tomorrow. |
| `until:` | no | YYYY-MM-DD, today, or tomorrow. |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `timezone:` | no | IANA timezone, such as Europe/Paris. Defaults to the calendar timezone. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### show

Show an event and attendee responses.

```text
jcal ACCOUNT show EVENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### add

Create a timed or all-day event.

```text
jcal ACCOUNT add TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `at:` | no | RFC 3339, YYYY-MM-DD HH:MM, or tomorrow HH:MM. |
| `on:` | no | YYYY-MM-DD, today, or tomorrow. |
| `for:` | no | Timed event duration, such as 30m or 1h. Maximum 366 days. Default: `"30m"`. |
| `with:` | no | Email addresses. Repeat for multiple values. |
| `where:` | no | Event location or meeting URL. |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `timezone:` | no | IANA timezone, such as Europe/Paris. Defaults to the calendar timezone. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Set a time with at: or an all-day date with on:.

for: sets a timed event's duration; omit it for all-day events.

### move

Reschedule an event. Its duration is preserved unless for: is supplied.

```text
jcal ACCOUNT move EVENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `at:` | yes | RFC 3339, YYYY-MM-DD HH:MM, or tomorrow HH:MM. |
| `for:` | no | Positive duration, such as 30m or 1h, up to 366 days. |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `timezone:` | no | IANA timezone, such as Europe/Paris. Defaults to the calendar timezone. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### cancel

Cancel an event.

```text
jcal ACCOUNT cancel EVENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `notify:` | no | Send email notifications to affected people. Default: `true`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### free

Find common free slots from 09:00 to 18:00 in calendar time.

```text
jcal ACCOUNT free [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `with:` | yes | Email addresses. Repeat for multiple values. |
| `on:` | no | YYYY-MM-DD, today, or tomorrow. Default: `"today"`. |
| `for:` | no | Positive duration, such as 30m or 1h, up to 366 days. Default: `"30m"`. |
| `calendar:` | no | Calendar ID. Default: `"primary"`. |
| `timezone:` | no | IANA timezone, such as Europe/Paris. Defaults to the calendar timezone. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### calendars

List available calendars.

```text
jcal ACCOUNT calendars [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jsheets

Read and change spreadsheet cells. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | Find spreadsheets. Requires Drive access. |
| `new` | Create a spreadsheet. |
| `read` | Display cells. Defaults to the first tab, A1:Z20. |
| `set` | Set one cell to a literal value. |
| `append` | Append literal CSV rows. |
| `load` | Write a CSV file starting at a cell. Values are literal. |
| `clear` | Clear an explicit cell range. |
| `export` | Export one tab as CSV or the workbook in another format. |

### ls

Find spreadsheets. Requires Drive access.

```text
jsheets ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `name:` | no | Name. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new

Create a spreadsheet.

```text
jsheets ACCOUNT new TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `tab:` | no | Tab name. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### read

Display cells. Defaults to the first tab, A1:Z20.

```text
jsheets ACCOUNT read SHEET [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Spreadsheet ID or URL. |
| `tab:` | no | Tab name. |
| `range:` | no | A1 cell range, such as A1:D20. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### set

Set one cell to a literal value.

```text
jsheets ACCOUNT set SHEET [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Spreadsheet ID or URL. |
| `tab:` | no | Tab name. |
| `cell:` | yes | One A1 cell, such as B2. |
| `value:` | yes | Literal cell value; formulas are not evaluated. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### append

Append literal CSV rows.

```text
jsheets ACCOUNT append SHEET [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Spreadsheet ID or URL. |
| `tab:` | no | Tab name. |
| `row:` | no | One CSV row per row: parameter. Repeat for multiple values. |
| `csv:` | no | Existing local file, up to 1 GiB. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Provide row: values or csv:PATH.

Use row: for inline records or csv: for a file, choosing one.

### load

Write a CSV file starting at a cell. Values are literal.

```text
jsheets ACCOUNT load SHEET [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Spreadsheet ID or URL. |
| `tab:` | no | Tab name. |
| `csv:` | yes | Existing local file, up to 1 GiB. |
| `at:` | no | At. Default: `"A1"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### clear

Clear an explicit cell range.

```text
jsheets ACCOUNT clear SHEET [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Spreadsheet ID or URL. |
| `tab:` | no | Tab name. |
| `range:` | yes | A1 cell range, such as A1:D20. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### export

Export one tab as CSV or the workbook in another format.

```text
jsheets ACCOUNT export SHEET [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Spreadsheet ID or URL. |
| `tab:` | no | Tab name. |
| `format:` | no | Format. Choices: `csv`, `xlsx`, `pdf`, `ods`. Default: `"csv"`. |
| `out:` | yes | Save to this new file. Existing files are never overwritten. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

tab: selects a CSV export. Other formats export the workbook.

## jdocs

Read and edit documents. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | Find documents. Requires Drive access. |
| `new` | Create a document and optionally add text. |
| `read` | Print document text. |
| `append` | Append text to a document tab. |
| `replace` | Replace matching text throughout a document. |
| `export` | Export a document. Requires Drive access. |
| `share` | Grant someone access to a document. Requires Drive access. |

### ls

Find documents. Requires Drive access.

```text
jdocs ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `name:` | no | Name. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new

Create a document and optionally add text.

```text
jdocs ACCOUNT new TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `text:` | no | Inline text. Use \n for a new line. |
| `text-file:` | no | Existing local file, up to 1 GiB. |
| `folder:` | no | File ID or Google Drive URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Supply text inline with text: or read it from text-file:.

### read

Print document text.

```text
jdocs ACCOUNT read DOCUMENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Document ID or Google Docs URL. |
| `tab:` | no | Document tab ID. Omit to read all tabs. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### append

Append text to a document tab.

```text
jdocs ACCOUNT append DOCUMENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Document ID or Google Docs URL. |
| `tab:` | no | Tab name. |
| `text:` | no | Inline text. Use \n for a new line. |
| `text-file:` | no | Existing local file, up to 1 GiB. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Supply text inline with text: or read it from text-file:.

### replace

Replace matching text throughout a document.

```text
jdocs ACCOUNT replace DOCUMENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Document ID or Google Docs URL. |
| `find:` | yes | Text to find. |
| `with:` | yes | Replacement text. |
| `match-case:` | no | Match the exact letter case. Default: `false`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### export

Export a document. Requires Drive access.

```text
jdocs ACCOUNT export DOCUMENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Document ID or Google Docs URL. |
| `format:` | no | Format. Choices: `pdf`, `docx`, `txt`, `html`, `odt`. Default: `"pdf"`. |
| `out:` | yes | Save to this new file. Existing files are never overwritten. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### share

Grant someone access to a document. Requires Drive access.

```text
jdocs ACCOUNT share DOCUMENT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Document ID or Google Docs URL. |
| `to:` | yes | Destination or recipient. |
| `role:` | no | Access granted to the recipient. Choices: `reader`, `writer`, `commenter`. Default: `"reader"`. |
| `notify:` | no | Send email notifications to affected people. Default: `true`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jslides

Build and edit presentations. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | Find presentations. Requires Drive access. |
| `new` | Create a presentation or copy a template. |
| `slides` | List slides by their visible number and title. |
| `add` | Add a slide and fill its title and body placeholders. |
| `set` | Replace a slide's title or body text. |
| `replace` | Replace matching text on one slide or throughout a presentation. |
| `export` | Export a presentation. Requires Drive access. |

### ls

Find presentations. Requires Drive access.

```text
jslides ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `name:` | no | Name. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new

Create a presentation or copy a template.

```text
jslides ACCOUNT new TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `template:` | no | Presentation ID or URL. |
| `folder:` | no | File ID or Google Drive URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### slides

List slides by their visible number and title.

```text
jslides ACCOUNT slides PRESENTATION [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Presentation ID or URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### add

Add a slide and fill its title and body placeholders.

```text
jslides ACCOUNT add PRESENTATION [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Presentation ID or URL. |
| `title:` | no | Inline text. Use \n for a new line. |
| `body:` | no | Inline text. Use \n for a new line. |
| `layout:` | no | Slide layout. Choices: `title-body`, `title`, `section`, `blank`. Default: `"title-body"`. |
| `after:` | no | Insert after this slide number; 0 inserts first. Omit to append. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

A blank slide has no title or body placeholders; omit title: and body:.

A section slide has a title only. Use layout:title-body to include body: text.

### set

Replace a slide's title or body text.

```text
jslides ACCOUNT set PRESENTATION [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Presentation ID or URL. |
| `slide:` | yes | Visible slide number, starting at 1. |
| `title:` | no | Inline text. Use \n for a new line. |
| `body:` | no | Inline text. Use \n for a new line. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Provide title: or body: to change the slide.

### replace

Replace matching text on one slide or throughout a presentation.

```text
jslides ACCOUNT replace PRESENTATION [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Presentation ID or URL. |
| `find:` | yes | Text to find. |
| `with:` | yes | Replacement text. |
| `slide:` | no | Visible slide number, starting at 1. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### export

Export a presentation. Requires Drive access.

```text
jslides ACCOUNT export PRESENTATION [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Presentation ID or URL. |
| `format:` | no | Format. Choices: `pdf`, `pptx`, `odp`. Default: `"pdf"`. |
| `out:` | yes | Save to this new file. Existing files are never overwritten. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jforms

Build forms and read responses. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | Find forms. Requires Drive access. |
| `new` | Create a form. |
| `show` | Show questions, publication state, and responder link. |
| `question` | Append a question to a form. |
| `answers` | Read responses or save a CSV file. |
| `publish` | Publish a form and return its responder link. |
| `close` | Stop accepting responses while preserving publication state. |
| `open` | Resume accepting responses on a published form. |

### ls

Find forms. Requires Drive access.

```text
jforms ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `name:` | no | Name. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new

Create a form.

```text
jforms ACCOUNT new TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `description:` | no | Description. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### show

Show questions, publication state, and responder link.

```text
jforms ACCOUNT show FORM [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Form ID or editor URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### question

Append a question to a form.

```text
jforms ACCOUNT question FORM TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Form ID or editor URL. |
| `title:` | yes | Title. |
| `type:` | no | Item type. Choices: `text`, `paragraph`, `choice`, `checkboxes`, `date`. Default: `"text"`. |
| `required:` | no | Require an answer to this question. Default: `false`. |
| `option:` | no | Answer choices. Repeat for multiple values. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Choice and checkbox questions require option: values. Other question types do not accept options.

### answers

Read responses or save a CSV file.

```text
jforms ACCOUNT answers FORM [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Form ID or editor URL. |
| `since:` | no | Responses submitted at or after this RFC 3339 timestamp. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `out:` | no | Save to this new file. Existing files are never overwritten. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### publish

Publish a form and return its responder link.

```text
jforms ACCOUNT publish FORM [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Form ID or editor URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### close

Stop accepting responses while preserving publication state.

```text
jforms ACCOUNT close FORM [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Form ID or editor URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### open

Resume accepting responses on a published form.

```text
jforms ACCOUNT open FORM [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Form ID or editor URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jtasks

Manage tasks and task lists. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | Show incomplete tasks in a list. |
| `add` | Create a task. |
| `show` | Show task details. |
| `done` | Mark a task complete. |
| `undo` | Mark a task incomplete. |
| `set` | Change selected task fields. |
| `delete` | Delete a task. |
| `lists` | List task lists. |
| `new-list` | Create a task list. |

### ls

Show incomplete tasks in a list.

```text
jtasks ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `list:` | no | Task list ID. Default: `"@default"`. |
| `due:` | no | YYYY-MM-DD, today, or tomorrow. |
| `completed:` | no | Include completed tasks. Default: `false`. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### add

Create a task.

```text
jtasks ACCOUNT add TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `due:` | no | YYYY-MM-DD, today, or tomorrow. |
| `notes:` | no | Inline text. Use \n for a new line. |
| `list:` | no | Task list ID. Default: `"@default"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### show

Show task details.

```text
jtasks ACCOUNT show TASK [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `list:` | no | Task list ID. Default: `"@default"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### done

Mark a task complete.

```text
jtasks ACCOUNT done TASK [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `list:` | no | Task list ID. Default: `"@default"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### undo

Mark a task incomplete.

```text
jtasks ACCOUNT undo TASK [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `list:` | no | Task list ID. Default: `"@default"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### set

Change selected task fields.

```text
jtasks ACCOUNT set TASK [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `title:` | no | Title. |
| `due:` | no | YYYY-MM-DD, today, or tomorrow. |
| `notes:` | no | Inline text. Use \n for a new line. |
| `list:` | no | Task list ID. Default: `"@default"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Provide title:, due:, or notes: to change the task.

### delete

Delete a task.

```text
jtasks ACCOUNT delete TASK [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `list:` | no | Task list ID. Default: `"@default"`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### lists

List task lists.

```text
jtasks ACCOUNT lists [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new-list

Create a task list.

```text
jtasks ACCOUNT new-list TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jcontacts

Find and maintain contacts. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | List contacts. |
| `find` | Find contacts by name, email, or phone. |
| `show` | Show contact details. |
| `new` | Create a contact. |
| `set` | Change named fields and preserve all others. Repeated email: and phone: values replace those lists. |
| `delete` | Delete a contact. |

### ls

List contacts.

```text
jcontacts ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### find

Find contacts by name, email, or phone.

```text
jcontacts ACCOUNT find TEXT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `query:` | yes | Search text. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### show

Show contact details.

```text
jcontacts ACCOUNT show CONTACT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new

Create a contact.

```text
jcontacts ACCOUNT new NAME [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `name:` | yes | Name. |
| `email:` | no | Email addresses. Repeat for multiple values. |
| `phone:` | no | Phone numbers. Repeat for multiple values. |
| `company:` | no | Company name. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### set

Change named fields and preserve all others. Repeated email: and phone: values replace those lists.

```text
jcontacts ACCOUNT set CONTACT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `name:` | no | Name. |
| `email:` | no | Email addresses. Repeat for multiple values. |
| `phone:` | no | Phone numbers. Repeat for multiple values. |
| `company:` | no | Company name. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Provide name:, email:, phone:, or company: to change the contact.

### delete

Delete a contact.

```text
jcontacts ACCOUNT delete CONTACT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jyt

Find videos and manage your channel and playlists. The default command is `info`.

| Command | Description |
| --- | --- |
| `find` | Search YouTube. |
| `info` | Show video details, or your channel when VIDEO is omitted. |
| `comments` | Read top-level comments. |
| `like` | Like a video. |
| `unlike` | Remove your rating from a video. |
| `playlists` | List your playlists or those of a channel. |
| `playlist` | List videos in a playlist. |
| `new-playlist` | Create a playlist. Defaults to private. |
| `add` | Append videos to a playlist. |
| `upload` | Upload a video. Defaults to private. |

### find

Search YouTube.

```text
jyt ACCOUNT find TEXT [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `query:` | yes | Search text. |
| `channel:` | no | Channel ID. |
| `type:` | no | Item type. Choices: `video`, `channel`, `playlist`. Default: `"video"`. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### info

Show video details, or your channel when VIDEO is omitted.

```text
jyt ACCOUNT info [VIDEO] [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | no | Video ID or YouTube URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### comments

Read top-level comments.

```text
jyt ACCOUNT comments VIDEO [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Video ID or YouTube URL. |
| `order:` | no | Sort order. Choices: `newest`, `relevance`. Default: `"newest"`. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### like

Like a video.

```text
jyt ACCOUNT like VIDEO [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Video ID or YouTube URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### unlike

Remove your rating from a video.

```text
jyt ACCOUNT unlike VIDEO [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Video ID or YouTube URL. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### playlists

List your playlists or those of a channel.

```text
jyt ACCOUNT playlists [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `channel:` | no | Channel ID. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### playlist

List videos in a playlist.

```text
jyt ACCOUNT playlist PLAYLIST [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new-playlist

Create a playlist. Defaults to private.

```text
jyt ACCOUNT new-playlist TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `privacy:` | no | Visibility. Defaults to private. Choices: `private`, `unlisted`, `public`. Default: `"private"`. |
| `description:` | no | Inline text. Use \n for a new line. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### add

Append videos to a playlist.

```text
jyt ACCOUNT add PLAYLIST [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `video:` | yes | Video IDs or YouTube URLs. Repeat for multiple values. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### upload

Upload a video. Defaults to private.

```text
jyt ACCOUNT upload PATH [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `path:` | yes | Existing local file, up to 1 GiB. |
| `title:` | yes | Title. |
| `category:` | yes | Category ID or an available category name. |
| `privacy:` | no | Visibility. Defaults to private. Choices: `private`, `unlisted`, `public`. Default: `"private"`. |
| `description:` | no | Inline text. Use \n for a new line. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

## jphotos

Manage photos, videos, and albums created by this application. The default command is `ls`.

| Command | Description |
| --- | --- |
| `ls` | List app-created media, optionally in an app-created album. |
| `find` | Filter app-created media by date and type. |
| `show` | Show app-created media details. |
| `get` | Download app-created media. |
| `put` | Upload a photo or video and create its media item. |
| `albums` | List app-created albums. |
| `new-album` | Create an album. |
| `add` | Add app-created media to an album. |
| `remove` | Remove media from an album without deleting the media. |

### ls

List app-created media, optionally in an app-created album.

```text
jphotos ACCOUNT ls [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `album:` | no | App-created album ID. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### find

Filter app-created media by date and type.

```text
jphotos ACCOUNT find [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `from:` | no | YYYY-MM-DD, today, or tomorrow. |
| `until:` | no | YYYY-MM-DD, today, or tomorrow. |
| `type:` | no | Item type. Choices: `photo`, `video`. |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

Provide from:, until:, or type: to filter media.

### show

Show app-created media details.

```text
jphotos ACCOUNT show PHOTO [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### get

Download app-created media.

```text
jphotos ACCOUNT get PHOTO [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `out:` | yes | Save to this new file. Existing files are never overwritten. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### put

Upload a photo or video and create its media item.

```text
jphotos ACCOUNT put PATH [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `path:` | yes | Existing local file, up to 1 GiB. |
| `album:` | no | App-created album ID. |
| `caption:` | no | Inline text. Use \n for a new line. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### albums

List app-created albums.

```text
jphotos ACCOUNT albums [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `limit:` | no | Maximum results, or all. Default: `20`. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### new-album

Create an album.

```text
jphotos ACCOUNT new-album TITLE [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `title:` | yes | Title. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### add

Add app-created media to an album.

```text
jphotos ACCOUNT add ALBUM [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `photo:` | yes | App-created media IDs. Repeat for multiple values. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

### remove

Remove media from an album without deleting the media.

```text
jphotos ACCOUNT remove ALBUM [key:value ...]
```

| Parameter | Required | Description |
| --- | --- | --- |
| `target:` | yes | Resource ID shown by its list command. |
| `photo:` | yes | App-created media IDs. Repeat for multiple values. |
| `home:` | no | Store credentials and cache in this directory. Advanced parameter. |
| `timeout:` | no | Request timeout in seconds. Default: `30`. Allowed range: 1 to 600. Advanced parameter. |
| `json:` | no | Print the same result data as JSON. Default: `false`. |

