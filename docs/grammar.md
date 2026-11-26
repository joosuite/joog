# Command reference

For short commands such as `jmail ACCOUNT to:lara@example.com body:Hello`,
see [Short command wrappers](wrappers.md). The wrappers have their own commands
and parameter syntax. All commands documented below remain available in `joog`.

## Usage

```text
joog [OPTIONS] SERVICE ACCOUNT COMMAND [ARGUMENTS] [COMMAND_OPTIONS]
joog [OPTIONS] auth [ACCOUNT] COMMAND
```

`make build` builds the current source and updates `./bin/joog`. Run its help
directly from the workspace:

```sh
./bin/joog mail ACCOUNT messages.send --help
```

`make install` updates the installed `joog` executable.

`joog` and its wrappers run without Python. The build and install steps use Rust
and Cargo. Python is used only by maintainer commands that regenerate schemas
and documentation.

| Option | Description |
| --- | --- |
| `--home PATH` | Store new state in `PATH/joog`. Existing `PATH/rust` vaults stay in place. |
| `--timeout SECONDS` | Request timeout, 1 through 600 seconds; default 30. |
| `--json` | Output the original Google response as JSON. |
| `--version` | Print the installed version. |
| `--help --verbose` | Include advanced parameters in command help. |

Results use plain text by default. Read commands display records or text; writes
confirm the operation and show the resource ID when Google returns one. List
output shows IDs and useful fields instead of transport metadata. `--fields`
prints your selected response fields; `--json` preserves the Google response.

## Authentication

```text
joog auth init [--headless]
joog auth enroll [--stdin]
joog auth test
joog auth list
joog auth ACCOUNT login SERVICE_LIST
joog auth ACCOUNT all
joog auth [ACCOUNT] export FILE
joog auth [ACCOUNT] import FILE [--headless] [--replace]
joog auth ACCOUNT logout
```

`init` creates or reuses the OS-keychain master key; `--headless` selects a private
local key file for a new vault. `enroll` saves your Desktop
app client ID and secret, with masked input. `--stdin` accepts a JSON object
containing `client_id` and `client_secret`. `test` checks Google OAuth connectivity.

`list` prints registered account emails, one per line. Use `joog --json auth list`
for a JSON array. It reads the selected vault's account registrations without
unlocking credentials or contacting Google. An empty vault produces no text,
or `[]` in JSON.

`login` opens your browser and authorizes the comma-separated services, for
example `gmail,drive,calendar`. Use `login all` to request every supported service's
configured scopes. The selected Google account must match `ACCOUNT`.
`auth ACCOUNT all` is the shortcut for `auth ACCOUNT login all`.
`gmail` and `mail` both authorize Gmail. Include all services you want to use
when signing in again. `logout` deletes that account's local tokens; revoke
Google permissions separately in your Google Account settings.

See [sign-in setup](authentication.md).

For a VPS without a browser, export credentials on your laptop with
`joog auth ACCOUNT export credentials.joog`, copy the encrypted file with `scp`,
then run `joog auth import credentials.joog --headless` on the VPS. Both commands
prompt for the transfer passphrase. Export includes the OAuth client and refresh
tokens; import creates the server vault without a desktop keychain. Use
`--replace` to replace existing account credentials. Advanced
`--passphrase-file PATH` reads a private passphrase file for automation.

## Commands

### mail

```text
joog mail ACCOUNT read MESSAGE_ID
joog mail ACCOUNT unread MESSAGE_ID
joog mail ACCOUNT star MESSAGE_ID
joog mail ACCOUNT unstar MESSAGE_ID
```

```sh
joog mail ACCOUNT profile
joog mail ACCOUNT labels.list
joog mail ACCOUNT messages.list --q is:unread
joog mail ACCOUNT messages.get --id MESSAGE_ID
joog mail ACCOUNT messages.send --to alice@example.com --cc bob@example.com --bcc archive@example.com --subject 'Hello' --text 'Hi Alice'
joog mail ACCOUNT messages.send --file examples/requests/mail-compose.json
joog mail ACCOUNT messages.send --upload message.eml
joog mail ACCOUNT drafts.create --to alice@example.com --subject 'Draft' --text-file draft.txt
```

[Full command help](services/mail.md).

`messages.send`, `messages.import`, `messages.insert`, `drafts.create`,
`drafts.update`, and `drafts.send` accept `--to`, `--cc`,
`--bcc`, `--from`, `--reply-to`, `--subject`, `--text`, `--html`, `--text-file`,
`--html-file`, and `--attach`. Repeat recipient and attachment options for
multiple values. Addresses use bare ASCII mailboxes, such as
`alice@example.com`. `--from` defaults to `ACCOUNT`; Google must authorize that
sender. Text and HTML together create a multipart message. Attachments use
`application/octet-stream`.

Body files must contain UTF-8 text. File paths resolve from the working directory.
Composition inputs have a combined 16 MiB content limit, and the final encoded
JSON request must also fit within 16 MiB. Use an `.eml` upload for larger messages.
Composition cannot be combined with `--upload` or raw message content.
Sending requires at least one recipient in `to`, `cc`, or `bcc`. Drafts can omit
recipients. An omitted body becomes empty plain text.

`drafts.send` requires the draft ID. With no composition inputs it sends the
existing content. Composition inputs replace the content when sending.

### drive

```text
joog drive ACCOUNT search TEXT [--parent ID] [--type MIME]
joog drive ACCOUNT restore FILE_ID
```

```sh
joog drive ACCOUNT files.list
joog drive ACCOUNT files.get --file-id FILE_ID
joog drive ACCOUNT files.export --file-id FILE_ID --mime-type application/pdf --output report.pdf
joog drive ACCOUNT permissions.create --file-id FILE_ID --type user --role reader --email-address alice@example.com
```

[Full command help](services/drive.md).

### calendar

```text
joog calendar ACCOUNT events [--calendar ID] [--from TIMESTAMP] [--until TIMESTAMP] [--query TEXT]
```

```sh
joog calendar ACCOUNT calendars.list
joog calendar ACCOUNT events.get --event-id EVENT_ID
joog calendar ACCOUNT events.insert --file event.json
```

[Full command help](services/calendar.md).

`events` expands recurring events and orders them by start time. The default window starts now and ends 30 days later. `--from` and `--until` select a different interval. `--calendar` defaults to `primary`.

### sheets

```sh
joog sheets ACCOUNT create --title TITLE
joog sheets ACCOUNT get --spreadsheet-id SPREADSHEET_ID
joog sheets ACCOUNT values.get --spreadsheet-id SPREADSHEET_ID --range 'Sheet1!A1:D10'
joog sheets ACCOUNT values.update --file values.json
```

[Full command help](services/sheets.md).

### docs

```sh
joog docs ACCOUNT create --title TITLE
joog docs ACCOUNT get --document-id DOCUMENT_ID
joog docs ACCOUNT insert-text --document-id DOCUMENT_ID --text Hello --index 1
joog docs ACCOUNT update-text-style --document-id DOCUMENT_ID --body-fields bold --text-style '{"bold":true}' --range '{"startIndex":1,"endIndex":6}'
joog docs ACCOUNT batch-update --file updates.json
```

[Full command help](services/docs.md).

Editing commands accept individual operation fields. `batch-update` applies multiple operations atomically. `--write-control` accepts revision constraints as JSON.

### slides

```sh
joog slides ACCOUNT create --title TITLE
joog slides ACCOUNT get --presentation-id PRESENTATION_ID
joog slides ACCOUNT create-slide --presentation-id PRESENTATION_ID --object-id SLIDE_ID --layout TITLE_AND_BODY
joog slides ACCOUNT insert-text --presentation-id PRESENTATION_ID --object-id SHAPE_ID --text Hello
joog slides ACCOUNT batch-update --file updates.json
```

[Full command help](services/slides.md).

Editing commands accept individual operation fields. `batch-update` applies multiple operations atomically. `--write-control` accepts revision constraints as JSON.

### forms

```text
joog forms ACCOUNT responses FORM_ID [--since TIMESTAMP]
joog forms ACCOUNT response FORM_ID RESPONSE_ID
```

```sh
joog forms ACCOUNT create --title TITLE
joog forms ACCOUNT get --form-id FORM_ID
joog forms ACCOUNT batch-update --file updates.json
```

[Full command help](services/forms.md).

### tasks

```sh
joog tasks ACCOUNT tasklists.list
joog tasks ACCOUNT tasks.list
joog tasks ACCOUNT tasks.insert --title TITLE
```

[Full command help](services/tasks.md).

### contacts

```sh
joog contacts ACCOUNT people.get --resource-name people/me --person-fields names,emailAddresses
joog contacts ACCOUNT people.connections.list --resource-name people/me --person-fields names,emailAddresses
joog contacts ACCOUNT people.create-contact --names '[{"givenName":"Alice"}]'
```

[Full command help](services/contacts.md).

### youtube

```text
joog youtube ACCOUNT search QUERY [--type TYPE] [--order ORDER]
```

```sh
joog youtube ACCOUNT channel-sections.list --part snippet --mine
joog youtube ACCOUNT videos.list --id VIDEO_ID
joog youtube ACCOUNT playlists.insert --title Playlist --privacy unlisted
```

[Full command help](services/youtube.md).

### photos

```sh
joog photos ACCOUNT albums.list
joog photos ACCOUNT media-items.list
joog photos ACCOUNT media-items.get --media-item-id MEDIA_ID
```

[Full command help](services/photos.md).

Photos commands access content created by your OAuth app. Google's Library API does not expose your entire personal library.

## Request and output options

| Option | Description |
| --- | --- |
| `--file PATH` | Read request parameters and a body from a JSON file. Named options override file values. |
| `--body JSON` | Supply the request body inline. |
| `--fields MASK` | Select response fields. |
| `--body-fields MASK` | Select the fields to change for editing operations whose body contains a `fields` mask. |
| `--max N` | Limit the total number of results. The default fetches all results. |
| `--max-pages N` | Stop after this many pages. |
| `--ttl MINUTES` | Reuse encrypted read results up to this age. The default, 0, fetches fresh data. |
| `--output PATH`, `-o PATH` | Save a download to a new file. Existing files are preserved. |
| `--upload PATH` | Upload a local file when the command supports media uploads. |
| `--content-type MIME` | Set the upload MIME type. |

Command help lists the available options. Complex fields accept JSON. Repeated
fields accept multiple values or repeated options. Boolean options support
`--option` and `--option=false`.

Request files describe CLI parameters with camelCase property names, such as
`fileId`, `startTime`, and `text`. See [schemas and examples](file-schemas.md).
Commands validate requests before accessing account credentials.

Read commands fetch all pages unless `--max` or `--max-pages` stops them.
Page sizes in request files control individual requests. Paginated JSON output
contains one Google response per line.

`--ttl 60` measures freshness from the original fetch time. Cached account data
stays encrypted. Writes and login changes invalidate its cache. Downloads and
ID or token generation always fetch fresh data. A cached default Calendar window
retains its original start and end until its TTL expires.

## Services

| Service | Google service |
| --- | --- |
| `mail` | Gmail |
| `drive` | Google Drive |
| `calendar` | Google Calendar |
| `sheets` | Google Sheets |
| `docs` | Google Docs |
| `slides` | Google Slides |
| `forms` | Google Forms |
| `tasks` | Google Tasks |
| `contacts` | Google Contacts |
| `youtube` | YouTube |
| `photos` | Google Photos app-created media |

## Help

`--help` lists common parameters. Add `--verbose` to include advanced parameters
and full descriptions. Both views group mandatory and optional parameters.
The `Input choices` section explains alternatives, such as using a date for an
all-day event or a date and time for a timed event.

Mandatory values can come from `--file` or command-line options. `--verbose` is
only available with help.

```sh
joog --help
joog help mail
joog mail ACCOUNT messages.send --help
joog mail ACCOUNT messages.send --help --verbose
joog help docs insert-text
joog help slides create-slide
```
