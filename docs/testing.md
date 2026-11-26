# Tests

Run the local suite with:

```sh
make test
make lint
make fmt-check
```

Local contract tests cover the service catalogs, native convenience commands,
and wrappers. They check request translation,
required inputs, incompatible parameters, JSON types, file overrides, output
parity, MIME handling, pagination, URL validation, and OAuth callbacks.
Adversarial cases use local parsers and loopback HTTP servers. They do not send
malformed requests to Google or use account credentials.

Focused transport fixtures check privileged Calendar, Contacts, Docs, and Drive
operations against loopback servers. They assert the HTTP request and error
handling without requiring an eligible Google account. They do not count as
successful live Google executions.

Credential tests cover encrypted export/import, account replacement, headless
storage, and listing accounts without unlocking credentials. They use synthetic
credentials in temporary vaults.

## Native credential storage

CI checks Windows Credential Manager and an isolated Linux Secret Service. These
tests create disposable keys and encrypted records, open a legacy vault,
reopen the store, move the vault, and check that the original key and encrypted
bytes are preserved. Both temporary credential-store entries are deleted.

Run these checks against an available OS credential store with:

```sh
cargo test --locked store::tests::real_ -- --ignored --nocapture
```

Linux needs an unlocked Secret Service on the user's D-Bus session. CI starts a
temporary keyring for this purpose. These tests do not use Google credentials.

## Live account checks

Live tests are ignored by default. Set the explicitly approved test account to
run them:

```sh
JOOG_LIVE_ACCOUNT=test-account@example.com make live-test
```

To investigate a resolved failure without repeating successful services, select
services with `JOOG_LIVE_SERVICES=docs,sheets`. The suite also checks Drive when
it needs Drive for cleanup. Every run verifies the account through the Gmail
profile before creating anything.

The account must already be signed in for the services being checked. Use a
dedicated test account. The suite creates synthetic messages, files,
secondary calendars, spreadsheets, documents, presentations, forms, task lists,
contacts, and YouTube resources. Mail delivery tests address only that
account. Forms tests publish a temporary form and submit one synthetic answer.
Drive sharing tests apply to synthetic files, disable notifications, and remove
the permission afterward. Calendar fixtures have no attendees. The suite does
not empty the account's Trash or clear its primary calendar.
YouTube uploads start private. Comment checks briefly use an unlisted synthetic
video on the test account's own channel, then remove its comments and video.
The subscription check creates and removes one subscription to Google's
developer channel only when that subscription does not already exist.
Watch checks use a temporary owned form as the HTTPS callback. Drive channels
are stopped explicitly. Calendar watches expire after ten seconds, before the
callback form and temporary calendar are deleted. These checks verify channel
creation and cleanup; they do not verify notification delivery.

Photos creation checks need explicit manual cleanup because the Library API
cannot delete uploaded media or albums. To include the full Photos workflow:

```sh
JOOG_LIVE_ACCOUNT=test-account@example.com \
  JOOG_LIVE_SERVICES=photos JOOG_LIVE_PHOTOS_CLEANUP=manual make live-test
```

The report's `manualCleanup` entries contain album and media IDs and links.
Delete the photo from the library as well as the album. Removing an album alone
does not delete its media. The suite creates one image and one album per run.

YouTube banner checks also require `JOOG_LIVE_PHOTOS_CLEANUP=manual` because the
uploaded test image remains in Channel Art. They back up an existing banner,
restore its original image, and verify its full-resolution SHA-256. A new image
URL after restoration is allowed. Keep the restored banner when removing the
test image. The stable backup file sits beside the report for recovery.

Watermark checks require `JOOG_LIVE_YOUTUBE_WATERMARK=disposable`. They replace
the watermark and remove it afterward. Google provides no watermark read method,
so this option is only for a channel whose watermark may be replaced and removed.

On macOS, live tests use a stable executable snapshot under `target/live-test-bin`
and lock it for the duration of the run. Rebuilding another target cannot
replace that running snapshot. Keychain access uses Apple's built-in `security`
tool and needs no local signing certificate. See [macOS Keychain approval](authentication.md#macos-approval-after-a-rebuild).

Each run uses a unique `joog-test-` marker. The suite records each resource's
cleanup command immediately after creation and cleans up after each service,
including after a failed assertion. The suite permanently deletes its own
Drive fixtures.
Cleanup accepts a registered fixture's HTTP 404 as already absent, for example
when deleting its parent also deleted that child.

Commands run sequentially with a 750 ms pause between them. The default budget
is 1,000 CLI invocations, with 32 reserved for cleanup. Set
`JOOG_LIVE_MAX_COMMANDS` to lower the budget. Direct collection checks default to
one page. Each CLI process has a 45-second deadline as well as the 15-second HTTP
timeout. The suite never retries a command. An unexpected failure stops dependent
checks for that service. Authentication, quota, connectivity, server, or cleanup
failures stop the run. The CLI's existing GET backoff may retry a 429 or transient
502, 503, or 504 at most twice; writes are never retried.

The report is saved after commands and resource registration to
`target/live-full-report.json`. Override the path with `JOOG_LIVE_REPORT`. It
records exact CLI and wrapper commands, including API calls made inside wrappers,
and pending cleanup IDs. It excludes tokens and retrieved message and document
contents. Optional `JOOG_LIVE_ERROR_REPORT` writes Google's error responses to a
private diagnostic file. These diagnostics require the test-only
`live-test-diagnostics` feature enabled by `make live-test`.

The coverage report compares successful commands with the checked-in catalogs.
It lists every missing command. Expected permission rejections and unavailable
fixtures remain separate from successful coverage. An unfiltered full run fails
when coverage is incomplete. A selected service or regression run checks that
scenario and still reports its remaining coverage gaps.

Run a focused regression with one matching service:

```sh
JOOG_LIVE_ACCOUNT=test-account@example.com \
  JOOG_LIVE_SERVICES=sheets JOOG_LIVE_CASE=csv make live-test
```

The other cases are `dropdown` and `docs-import` for `docs`, `slides-media` for
`slides`, `mail-draft` for `mail`, `drive-shared` and `drive-watch` for `drive`,
`calendar-owner`, `calendar-subscription`, and `calendar-watch` for `calendar`,
`contacts-other` for `contacts`, and `youtube-remaining` and
`youtube-capabilities`, `youtube-sections`, `youtube-image-update`,
`youtube-channel`, `youtube-banner`, and `youtube-watermark` for `youtube`.
Invalid service names and case combinations fail before accessing credentials.

If cleanup fails or the process is interrupted, inspect the report before
running again. Resolve the cause, then recover only its registered fixtures:

```sh
JOOG_LIVE_ACCOUNT=test-account@example.com \
  JOOG_LIVE_CLEANUP_REPORT=target/live-full-report.json make live-cleanup
```

Recovery verifies the account, accepts only supported cleanup commands, and
writes a separate `target/live-cleanup-report.json`. It makes one attempt per
pending resource and stops at the first unexpected error.
Restoration entries verify the saved channel settings or banner hash before
clearing the ledger. A missing or changed banner backup fails before upload.

Combine recorded command coverage without making Google requests:

```sh
JOOG_LIVE_REPORTS=target/run-one.json:target/run-two.json make live-coverage
```

Use a semicolon instead of a colon on Windows. The audit writes
`target/live-coverage.json`, preserving source report names, historical failure
counts, binary fingerprints when available, and pending cleanup entries.
The separate `manualCleanup` ledger also survives report aggregation.
Retain reports and restoration backups with pending cleanup entries until
those resources have been removed or restored.
Successful executions from different runs do not certify that one current binary
passed every scenario. The unfiltered live test remains the completeness check.
