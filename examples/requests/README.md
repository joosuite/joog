# JSON request examples

These files contain Joog CLI parameters. Run them from the repository root with
`--file`, using the matching command below:

```sh
joog mail you@example.com messages.list --file examples/requests/mail.json
joog drive you@example.com files.list --file examples/requests/drive.json
```

Replace `ACCOUNT` with your signed-in account and replace resource IDs such as
`DOCUMENT_ID` inside the files with your own IDs. List examples use `"max": 10`
to limit the total number of results across all pages. Explicit command options
override file values, so `--max 5` reduces that limit.

## Read examples

| File | Command |
| --- | --- |
| [mail.json](mail.json) | `joog mail ACCOUNT messages.list` |
| [drive.json](drive.json) | `joog drive ACCOUNT files.list` |
| [calendar.json](calendar.json) | `joog calendar ACCOUNT events.list` |
| [sheets.json](sheets.json) | `joog sheets ACCOUNT values.get` |
| [docs.json](docs.json) | `joog docs ACCOUNT get` |
| [slides.json](slides.json) | `joog slides ACCOUNT get` |
| [forms.json](forms.json) | `joog forms ACCOUNT get` |
| [tasks.json](tasks.json) | `joog tasks ACCOUNT tasks.list` |
| [contacts.json](contacts.json) | `joog contacts ACCOUNT people.connections.list` |
| [youtube.json](youtube.json) | `joog youtube ACCOUNT search.list` |
| [photos.json](photos.json) | `joog photos ACCOUNT media-items.list` |

## Write examples

Append `--file examples/requests/FILE.json` to the corresponding command.

| File | Command |
| --- | --- |
| [mail-write.json](mail-write.json) | `joog mail ACCOUNT messages.modify` |
| [drive-write.json](drive-write.json) | `joog drive ACCOUNT permissions.create` |
| [calendar-write.json](calendar-write.json) | `joog calendar ACCOUNT events.insert` |
| [sheets-write.json](sheets-write.json) | `joog sheets ACCOUNT values.update` |
| [docs-write.json](docs-write.json) | `joog docs ACCOUNT batch-update` |
| [slides-write.json](slides-write.json) | `joog slides ACCOUNT batch-update` |
| [forms-write.json](forms-write.json) | `joog forms ACCOUNT batch-update` |
| [tasks-write.json](tasks-write.json) | `joog tasks ACCOUNT tasks.insert` |
| [contacts-write.json](contacts-write.json) | `joog contacts ACCOUNT people.create-contact` |
| [youtube-write.json](youtube-write.json) | `joog youtube ACCOUNT playlists.insert` |
| [photos-write.json](photos-write.json) | `joog photos ACCOUNT albums.create` |
| [mail-compose.json](mail-compose.json) | `joog mail ACCOUNT messages.send` |
| [docs-insert-text.json](docs-insert-text.json) | `joog docs ACCOUNT insert-text` |
| [slides-create-slide.json](slides-create-slide.json) | `joog slides ACCOUNT create-slide` |

For parameter types, required values, and choices, see the
[JSON parameter file guide](../../docs/file-schemas.md) and command help:

```sh
joog mail ACCOUNT messages.send --help
joog mail ACCOUNT messages.send --help --verbose
```
