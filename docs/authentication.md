# Sign-in setup

## Initialize storage

```sh
./bin/joog auth init
```

Joog creates a 256-bit master key in the OS keychain. Running the command again
leaves an existing vault intact without reading its key or changing Keychain
permissions. On macOS, credential reads use Apple's built-in `/usr/bin/security`
tool. Rebuilding Joog does not change that tool's identity. Access to an existing
entry can still require macOS approval.

| Platform | Credential store | Default state directory |
| --- | --- | --- |
| macOS | Keychain through Apple's `security` tool | `~/Library/Application Support/joog` |
| Windows | Windows Credential Manager | `%APPDATA%\joog` |
| Linux | Secret Service over the user's D-Bus session | `$XDG_CONFIG_HOME/joog`, or `~/.config/joog` |

## Create Google client credentials

In the [Google Cloud Console](https://console.cloud.google.com/):

1. Select or create a project.
2. Enable the Google APIs you plan to use.
3. Configure the OAuth consent details in Google Auth Platform. For an external
   audience in testing, add your Google account as a test user.
4. Open [Clients](https://console.cloud.google.com/auth/clients) and create a
   **Desktop app** client.
5. Copy its client ID and client secret.

Desktop app clients support the local callback used by Joog. Google documents
the setup in [OAuth for desktop apps](https://developers.google.com/identity/protocols/oauth2/native-app).

## Enroll the credentials

```sh
./bin/joog auth enroll
```

Enter the client ID and client secret. The prompts mask input with `*`.
Joog encrypts the credentials locally and prints `Google client credentials saved.`

For automation, `joog auth enroll --stdin` reads a JSON object containing
`client_id` and `client_secret` from a protected input source.

When switching OAuth clients, log out accounts using the previous client,
enroll the new credentials, and sign in again.

## Test the connection

```sh
./bin/joog auth test
```

Joog checks Google's OAuth service using the enrolled credentials. On success:

```text
Google connection successful.
```

## Sign in

```sh
./bin/joog auth you@example.com all
```

`all` is a shortcut for `login all`. It requests all configured permissions for
the eleven supported services. To select fewer services, use a comma-separated
list:

```sh
./bin/joog auth you@example.com login gmail,drive,calendar
```

Select the matching Google account in the browser and approve the requested
permissions. Complete sign-in within three minutes. Joog saves encrypted tokens
and refreshes them automatically when needed.

Full method support needs the permissions associated with the selected services.
Sign in again when adding capabilities to an older account grant. Gmail's full
command set includes permanent deletion. Directory operations require an
eligible Workspace organization; some YouTube operations
require creator or partner privileges.

## List registered accounts

```sh
./bin/joog auth list
./bin/joog --json auth list
```

The default output lists one account email per line. JSON output contains an
array of the same emails. Use `--home PATH` to list accounts in another vault.
Listing does not unlock credentials or contact Google. It shows registrations;
Google permissions may still have expired or been revoked. An empty vault
produces no text, or `[]` in JSON.

## Run commands

```sh
./bin/joog drive you@example.com files.list --max 5
./bin/joog --json drive you@example.com files.list --max 5
./bin/joog mail you@example.com messages.list --q is:unread
```

See the [command reference](grammar.md) and [parameter file schemas](file-schemas.md).

## Use Joog on a VPS

Sign in on your laptop, then export the account:

```sh
./bin/joog auth you@example.com all
./bin/joog auth you@example.com export credentials.joog
scp credentials.joog user@vps:~/
```

Export includes the OAuth client ID, client secret, and the account's refresh
token. Choose a transfer passphrase with at least 12 characters when prompted.
The file is encrypted with AES-256-GCM using a key derived with
[Argon2id](https://docs.rs/argon2/0.5.3/argon2/). Existing export files are never
overwritten. Omit the account to export all signed-in accounts:

```sh
./bin/joog auth export credentials-all.joog
```

On the VPS, import the file and enter the same passphrase:

```sh
joog auth import credentials.joog --headless
joog mail you@example.com messages.list --max 5
```

Import needs no browser, Google request, or prior `auth enroll`. `--headless`
creates a vault with a local `master.key` file instead of using a desktop
keychain. The vault directory uses mode `0700` and the key uses `0600` on Unix.
Protect that directory as you would an SSH private key. Subsequent commands and
wrappers select this storage automatically. Access tokens refresh when you next
use a service, using the permissions already granted on the laptop.

Omit `--headless` when importing to a desktop with an OS keychain. To create an
empty headless vault separately, use `joog auth init --headless`. An existing
keychain vault is never converted to file storage; choose a separate directory
with `--home PATH` if needed.

Import merges accounts into the destination vault. Existing account credentials
require `--replace`. An import with different OAuth client credentials cannot
replace the client while leaving accounts that depend on the old client. Use a
separate vault in that case. Add `ACCOUNT` before `import` to select one account
from a multi-account export.

For automation, `--passphrase-file PATH` reads the passphrase from a private
UTF-8 file, with an optional final newline. On Unix, its permissions must exclude
other users. See `auth export --help --verbose` and `auth import --help --verbose`.
Passphrases are never accepted as command-line values or written to the vault.

After importing, remove the transfer files from both machines when you no longer
need them. `auth logout` removes only the local credentials. Revoking the Google
grant affects both machines.

## Sign out

```sh
./bin/joog auth you@example.com logout
```

Joog deletes that account's local tokens. To revoke its Google permissions,
remove the grant in your Google Account settings.

## Stored data

| Data | Storage |
| --- | --- |
| Master key | OS keychain, or private `master.key` with `--headless` |
| Client ID and secret | Encrypted `oauth-client` record |
| Account tokens | Encrypted record per account |
| Authorized account emails | `config.json` |
| Cached Google responses | Encrypted account cache when `--ttl` is positive |

`--home PATH` selects `PATH/joog` and a separate vault.

The default Keychain service is `joog/vault`, with account `master-key`.
Custom vaults have a random identifier saved once in `vault.json`. Moving a vault
preserves that identifier. Builds never create a new vault identity.
On Windows, Credential Manager displays the default entry as
`master-key.joog/vault`. On Linux, its Secret Service attributes are
`service=joog/vault` and `username=master-key`.
Credential files are under `secrets/`. Cached responses are under
`responses/ACCOUNT_HASH/`.

Existing `gogcli-rust` directories, or `PATH/rust` with `--home`, stay in place.
Joog reads their existing keys without copying or renaming them. Normal commands
and `auth init` do not migrate storage or change Keychain permissions. If both
directories contain state, Joog stops without choosing or changing either vault.

Windows and Linux read keys through their native credential stores.
No signing certificate is required. Each command reads the master key once;
wrappers reuse it for all their API operations. An OS store that is locked or
unavailable still needs to be unlocked or made available. On Linux, keychain
storage requires a running Secret Service on the user's D-Bus session. Headless
vaults use their private key file and do not contact Secret Service.

Credentials and cached account data use AES-256-GCM encryption. On Unix,
directories use mode `0700` and credential files use mode `0600`.

Caching is disabled by default. `--ttl 60` keeps read responses for up to an
hour. Each account's cache holds at most 32 entries and 64 MiB. Expired or damaged
entries are refetched from Google.

## Troubleshooting

### macOS approval after a rebuild

New Joog Keychain entries trust `/usr/bin/security`, which Apple signs and supplies
with macOS. Access approval therefore belongs to that stable tool instead of
each rebuilt Joog executable. No local signing certificate is needed.

Existing entries keep their original access rules. Reading them through the
system tool may still require macOS approval. Joog does not rewrite those rules.

The master key travels through private process pipes, never through command-line
arguments, environment variables, or temporary files. Keychain entries do not
allow every application. A process running as your macOS user can invoke the
trusted `security` tool to access the entry.

Within a command, Joog keeps one master-key session in memory and clears it when
the process exits. Wrappers reuse that session for their API operations. A locked
login Keychain can still require unlocking. If an older build asks for access as
`joog`, rebuild with `make build` and use the new binary and its wrappers.

### Error messages

| Error | Action |
| --- | --- |
| Keychain unavailable or locked | Unlock the OS keychain and retry. |
| Master key missing | Restore the original keychain entry. A new key cannot decrypt existing records. |
| Credential authentication failed | Check the vault path and keychain entry, or restore intact credential files. |
| Client credentials missing | Run `joog auth enroll`. |
| Redirect URI mismatch | Check that the enrolled client has application type Desktop app. |
| Browser authorization timed out | Retry login and complete sign-in within three minutes. |
| Callback does not match the current sign-in attempt | Use the browser tab opened by the current login command. |
| Google denies access | Check the consent audience, test users, and enabled APIs. |
| HTTP 403 | Check API enablement, permissions, and the services authorized at login. |

Use `auth export` and `auth import` to move credentials between machines.
Copying encrypted credential records alone is insufficient without their
original master key.
