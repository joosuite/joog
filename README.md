# <img src="assets/brand/logo.png" alt="Joog" width="240" height="120">

Google services from the terminal, with named command options, JSON parameter
files, and short commands through native wrappers.

Joog supports Gmail, Drive, Calendar, Sheets, Docs, Slides, Forms, Tasks,
Contacts, YouTube, and app-created Google Photos content.

Joog is an independent open-source project and is not affiliated with, endorsed
by, or sponsored by Google. It is provided "as is," without warranties of any
kind or guarantees of availability, reliability, or continued access to Google
services.

## Build and install

Building from source requires Rust 1.88 or newer and Cargo. With Make installed:

```sh
make build
./bin/joog --help
```

The build creates `bin/joog` and eleven wrapper entry points. To install them
in Cargo's binary directory:

```sh
make install
```

On Windows, Cargo can build Joog and create the wrappers without Make:

```powershell
cargo build --release --locked
cargo run --release --locked --example install_wrappers -- --directory target/release
```

Joog and its wrappers run without Python. Python is needed only for maintainer
commands that regenerate schemas and documentation.

## Sign in

Create a Google Cloud project, enable the APIs you need, and create OAuth
credentials for a **Desktop app**. Follow the [sign-in setup](docs/authentication.md)
for the Google consent configuration and platform keychain requirements.

```sh
joog auth init
joog auth enroll
joog auth you@example.com all
```

`all` requests the configured permissions for every supported service. To choose
services individually:

```sh
joog auth you@example.com login gmail,drive,calendar
```

Use `joog auth list` to list registered accounts, or `joog --json auth list` for
a JSON array.

Account tokens and client credentials are encrypted locally. Joog stores the
master key in the operating system's keychain by default.

For a VPS without a browser, sign in on your laptop and transfer an encrypted
credential file:

```sh
# Laptop
joog auth you@example.com export credentials.joog
scp credentials.joog user@vps:~/

# VPS
joog auth import credentials.joog --headless
```

Both commands prompt for the transfer passphrase. The headless vault uses a
private local key file and works without a desktop keychain. See the
[VPS setup guide](docs/authentication.md#use-joog-on-a-vps) for account selection,
replacing credentials, and automation.

## Use Joog

```sh
joog mail you@example.com messages.list --q is:unread --max 10
joog drive you@example.com files.list --max 10
joog --json drive you@example.com files.list --max 10
joog mail you@example.com messages.send --help
joog mail you@example.com messages.send --help --verbose
```

Common parameters appear in `--help`; `--help --verbose` includes advanced
parameters. Use `--file PATH` to supply command parameters from a JSON object.

The wrappers provide shorter commands and `key:value` parameters:

```sh
jmail you@example.com inbox limit:10 unread:true
jcal you@example.com today
jdrive you@example.com ls limit:10
```

Short syntax belongs to the wrappers. The corresponding `joog` commands remain
available. A wrapper's `json:true` option changes the output format while
preserving its result data.

## Documentation

- [Command reference](docs/grammar.md)
- [Short commands and wrappers](docs/wrappers.md)
- [JSON parameter files and schemas](docs/file-schemas.md)
- [Request examples and their commands](examples/requests/README.md)
- [Authentication and credential storage](docs/authentication.md)
- [Local and live tests](docs/testing.md)

## Contribute

Read [CONTRIBUTING.md](CONTRIBUTING.md) for development checks and pull requests,
and the [code of conduct](CODE_OF_CONDUCT.md) for participation guidelines.
Report ordinary bugs through
[GitHub issues](https://github.com/joosuite/joog/issues).
For vulnerabilities, follow [SECURITY.md](SECURITY.md).

## License

Joog is licensed under the [Apache License, Version 2.0](LICENSE).
Third-party dependencies retain their respective licenses.
