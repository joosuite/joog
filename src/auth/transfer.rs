use super::{Credentials, Tokens, masked_prompt};
use crate::{http::limited, store::Store};
use anyhow::{Context, Result, ensure};
use argon2::{Algorithm, Argon2, Params, Version};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

const MAGIC: &[u8; 8] = b"JOOGEXP1";
const LIMIT: u64 = 8 * 1024 * 1024;
const CONTEXT: &str = "joog-credential-export-v1";

#[derive(Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
#[serde(deny_unknown_fields)]
struct Bundle {
    version: u32,
    client: Credentials,
    accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
#[serde(deny_unknown_fields)]
struct Account {
    email: String,
    refresh_token: String,
}

fn email(value: &str) -> Result<String> {
    ensure!(
        value.len() <= 320
            && value.split('@').count() == 2
            && value.split('@').all(|part| !part.is_empty())
            && !value
                .chars()
                .any(|ch| ch.is_whitespace() || ch.is_control()),
        "invalid account email in credential transfer"
    );
    Ok(value.to_ascii_lowercase())
}

fn collect(store: &Store, selected: Option<&str>) -> Result<Bundle> {
    let client: Credentials = store.secret("oauth-client")?;
    let mut emails = if let Some(selected) = selected {
        vec![email(selected)?]
    } else {
        store.config()?.accounts
    };
    emails.sort();
    emails.dedup();
    ensure!(!emails.is_empty(), "no signed-in accounts to export");
    let mut accounts = Vec::new();
    for address in emails {
        let address = email(&address)?;
        let tokens: Tokens = store.secret(&format!("account/{address}"))?;
        accounts.push(Account {
            email: address,
            refresh_token: tokens.refresh_token.clone(),
        });
    }
    let bundle = Bundle {
        version: 1,
        client,
        accounts,
    };
    validate(&bundle)?;
    Ok(bundle)
}

fn validate(bundle: &Bundle) -> Result<()> {
    ensure!(bundle.version == 1, "unsupported credential export version");
    ensure!(
        !bundle.client.client_id.trim().is_empty()
            && !bundle.client.client_secret.trim().is_empty(),
        "credential export is missing its OAuth client credentials"
    );
    ensure!(
        !bundle.accounts.is_empty() && bundle.accounts.len() <= 128,
        "credential export must contain between 1 and 128 accounts"
    );
    let mut seen = std::collections::BTreeSet::new();
    for account in &bundle.accounts {
        let address = email(&account.email)?;
        ensure!(
            seen.insert(address),
            "credential export contains a duplicate account"
        );
        ensure!(
            !account.refresh_token.trim().is_empty() && account.refresh_token.len() <= 64 * 1024,
            "credential export is missing a valid refresh token"
        );
    }
    Ok(())
}

fn derive(passphrase: &str, salt: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
    let params = Params::new(64 * 1024, 3, 1, Some(32))
        .map_err(|_| anyhow::anyhow!("cannot configure export encryption"))?;
    let mut key = Zeroizing::new(vec![0; 32]);
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|_| anyhow::anyhow!("cannot derive export encryption key"))?;
    Ok(key)
}

fn seal(bundle: &Bundle, passphrase: &str) -> Result<Vec<u8>> {
    ensure!(
        passphrase.chars().count() >= 12,
        "use an export passphrase with at least 12 characters"
    );
    let mut salt = [0; 16];
    getrandom::fill(&mut salt).context("operating system randomness unavailable")?;
    let key = derive(passphrase, &salt)?;
    let raw = Zeroizing::new(serde_json::to_vec(bundle)?);
    ensure!(
        raw.len() as u64 <= LIMIT - 128,
        "credential export exceeds size limit"
    );
    let ciphertext = crate::store::encrypt(&key, CONTEXT, &raw)?;
    Ok([MAGIC.as_slice(), salt.as_slice(), &ciphertext].concat())
}

fn open(bytes: &[u8], passphrase: &str) -> Result<Bundle> {
    ensure!(
        bytes.len() >= 60 && bytes.len() as u64 <= LIMIT && bytes.starts_with(MAGIC),
        "invalid or unsupported credential export file"
    );
    let key = derive(passphrase, &bytes[8..24])?;
    let raw = crate::store::decrypt(&key, CONTEXT, &bytes[24..]).map_err(|_| {
        anyhow::anyhow!(
            "cannot decrypt credential export; the passphrase is incorrect or the file was damaged"
        )
    })?;
    let bundle: Bundle = serde_json::from_slice(&raw)
        .map_err(|_| anyhow::anyhow!("invalid credential export contents"))?;
    validate(&bundle)?;
    Ok(bundle)
}

fn passphrase(path: Option<&Path>, confirm: bool) -> Result<Zeroizing<String>> {
    let value = if let Some(path) = path {
        let metadata = fs::symlink_metadata(path).context("cannot read passphrase file")?;
        ensure!(
            metadata.is_file(),
            "passphrase file must be a regular file, not a symbolic link"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            ensure!(
                metadata.permissions().mode() & 0o077 == 0,
                "passphrase file must be readable only by its owner; use chmod 600"
            );
        }
        let raw = Zeroizing::new(limited(fs::File::open(path)?, 16 * 1024)?);
        let mut value = Zeroizing::new(
            std::str::from_utf8(&raw)
                .context("passphrase file must contain UTF-8 text")?
                .to_owned(),
        );
        if value.ends_with('\n') {
            value.pop();
            if value.ends_with('\r') {
                value.pop();
            }
        }
        value
    } else {
        let value = Zeroizing::new(masked_prompt("Transfer passphrase: ")?);
        if confirm {
            let repeated = Zeroizing::new(masked_prompt("Confirm transfer passphrase: ")?);
            ensure!(
                bool::from(value.as_bytes().ct_eq(repeated.as_bytes())),
                "transfer passphrases do not match"
            );
        }
        value
    };
    ensure!(!value.is_empty(), "transfer passphrase cannot be empty");
    Ok(value)
}

pub fn export(
    store: &Store,
    selected: Option<&str>,
    path: &Path,
    passphrase_file: Option<&Path>,
) -> Result<Vec<String>> {
    ensure!(
        fs::symlink_metadata(path).is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound),
        "export destination already exists or cannot be accessed; choose a new file"
    );
    let bundle = collect(store, selected)?;
    let password = passphrase(passphrase_file, true)?;
    let bytes = seal(&bundle, &password)?;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut file = tempfile::NamedTempFile::new_in(parent).context("cannot create export file")?;
    file.write_all(&bytes)?;
    file.as_file().sync_all()?;
    file.persist_noclobber(path)
        .map_err(|error| error.error)
        .context("cannot save export file; existing files are never overwritten")?;
    Ok(bundle
        .accounts
        .iter()
        .map(|account| account.email.clone())
        .collect())
}

pub fn import(
    store: &Store,
    selected: Option<&str>,
    path: &Path,
    passphrase_file: Option<&Path>,
    headless: bool,
    replace: bool,
) -> Result<Vec<String>> {
    let bytes = limited(
        fs::File::open(path).context("cannot open credential export")?,
        LIMIT,
    )?;
    ensure!(
        bytes.starts_with(MAGIC) && bytes.len() >= 60,
        "invalid or unsupported credential export file"
    );
    let password = passphrase(passphrase_file, false)?;
    restore(
        store,
        &open(&bytes, &password)?,
        selected,
        headless,
        replace,
    )
}

fn restore(
    store: &Store,
    bundle: &Bundle,
    selected: Option<&str>,
    headless: bool,
    replace: bool,
) -> Result<Vec<String>> {
    validate(bundle)?;
    let selected = selected.map(email).transpose()?;
    let mut accounts = Vec::new();
    let mut records = vec![(
        "oauth-client".to_owned(),
        Zeroizing::new(serde_json::to_vec(&bundle.client)?),
    )];
    for account in &bundle.accounts {
        let address = email(&account.email)?;
        if selected
            .as_ref()
            .is_some_and(|selected| *selected != address)
        {
            continue;
        }
        let tokens = Tokens {
            access_token: String::new(),
            refresh_token: account.refresh_token.clone(),
            expires_at: 0,
        };
        records.push((
            format!("account/{address}"),
            Zeroizing::new(serde_json::to_vec(&tokens)?),
        ));
        accounts.push(address);
    }
    ensure!(
        !accounts.is_empty(),
        "the selected account is not in this credential export"
    );
    store.import_credentials(&records, &accounts, replace, headless)?;
    for account in &accounts {
        let _ = crate::cache::invalidate(&store.root, account);
    }
    Ok(accounts)
}

use subtle::ConstantTimeEq;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Config;

    fn bundle() -> Bundle {
        Bundle {
            version: 1,
            client: Credentials {
                client_id: "private-client-id".into(),
                client_secret: "private-client-secret".into(),
            },
            accounts: vec![
                Account {
                    email: "alice@example.com".into(),
                    refresh_token: "private-alice-refresh".into(),
                },
                Account {
                    email: "other@example.com".into(),
                    refresh_token: "private-other-refresh".into(),
                },
            ],
        }
    }

    fn store(directory: &Path) -> Store {
        Store::new(Some(directory.to_str().unwrap())).unwrap()
    }

    fn password_file(directory: &Path, value: &[u8]) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new_in(directory).unwrap();
        file.write_all(value).unwrap();
        file
    }

    #[test]
    fn encrypted_transfer_round_trip_is_portable_and_rejects_wrong_passwords_and_tampering() {
        let laptop = tempfile::tempdir().unwrap();
        let source = store(laptop.path());
        restore(&source, &bundle(), None, true, false).unwrap();
        let password = password_file(laptop.path(), b"a long private transfer passphrase\n");
        let file = laptop.path().join("credentials.joog");
        assert_eq!(
            export(
                &source,
                Some("ALICE@example.com"),
                &file,
                Some(password.path())
            )
            .unwrap(),
            ["alice@example.com"]
        );
        let bytes = fs::read(&file).unwrap();
        for value in [
            "private-client-id",
            "private-client-secret",
            "private-alice-refresh",
            "alice@example.com",
            "a long private transfer passphrase",
        ] {
            assert!(
                !bytes
                    .windows(value.len())
                    .any(|window| window == value.as_bytes())
            );
        }
        assert!(export(&source, None, &file, Some(password.path())).is_err());
        assert_eq!(fs::read(&file).unwrap(), bytes);

        let vps = tempfile::tempdir().unwrap();
        let destination = store(vps.path());
        let wrong = password_file(vps.path(), b"an incorrect transfer passphrase");
        let error = import(&destination, None, &file, Some(wrong.path()), true, false).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("cannot decrypt credential export")
        );
        assert!(!destination.root.exists());
        let mut damaged = bytes.clone();
        *damaged.last_mut().unwrap() ^= 1;
        fs::write(&file, damaged).unwrap();
        assert!(
            import(
                &destination,
                None,
                &file,
                Some(password.path()),
                true,
                false
            )
            .is_err()
        );
        assert!(!destination.root.exists());
        fs::write(&file, &bytes).unwrap();
        assert_eq!(
            import(
                &destination,
                None,
                &file,
                Some(password.path()),
                true,
                false
            )
            .unwrap(),
            ["alice@example.com"]
        );

        let reopened = store(vps.path());
        let restored: Tokens = reopened.secret("account/alice@example.com").unwrap();
        assert_eq!(restored.refresh_token, "private-alice-refresh");
        assert!(restored.access_token.is_empty());
        assert_eq!(restored.expires_at, 0);
        let client: Credentials = reopened.secret("oauth-client").unwrap();
        assert_eq!(client.client_secret, "private-client-secret");
        assert_ne!(
            fs::read(source.root.join("master.key")).unwrap(),
            fs::read(reopened.root.join("master.key")).unwrap()
        );
        assert_eq!(reopened.config().unwrap().accounts, ["alice@example.com"]);
        assert!(reopened.uses_key_file().unwrap());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&file).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
    }

    #[test]
    fn imports_preserve_other_accounts_and_refuse_client_changes_that_would_break_them() {
        let directory = tempfile::tempdir().unwrap();
        let destination = store(directory.path());
        restore(&destination, &bundle(), None, true, false).unwrap();
        let mut config = destination.config().unwrap();
        config.account = Some("other@example.com".into());
        destination.save_config(&config).unwrap();
        let mut incoming = bundle();
        incoming.accounts[0].refresh_token = "new-alice-refresh".into();
        assert!(
            restore(
                &destination,
                &incoming,
                Some("alice@example.com"),
                false,
                false
            )
            .unwrap_err()
            .to_string()
            .contains("--replace")
        );
        let old: Tokens = destination.secret("account/alice@example.com").unwrap();
        assert_eq!(old.refresh_token, "private-alice-refresh");
        restore(
            &destination,
            &incoming,
            Some("alice@example.com"),
            false,
            true,
        )
        .unwrap();
        let replaced: Tokens = destination.secret("account/alice@example.com").unwrap();
        assert_eq!(replaced.refresh_token, "new-alice-refresh");
        let other: Tokens = destination.secret("account/other@example.com").unwrap();
        assert_eq!(other.refresh_token, "private-other-refresh");
        assert_eq!(
            destination.config().unwrap().account.as_deref(),
            Some("other@example.com")
        );

        incoming.client.client_id = "different-client".into();
        assert!(
            restore(
                &destination,
                &incoming,
                Some("alice@example.com"),
                false,
                true
            )
            .unwrap_err()
            .to_string()
            .contains("other accounts")
        );
        let client: Credentials = destination.secret("oauth-client").unwrap();
        assert_eq!(client.client_id, "private-client-id");
        restore(&destination, &incoming, None, false, true).unwrap();
        let client: Credentials = destination.secret("oauth-client").unwrap();
        assert_eq!(client.client_id, "different-client");
        assert_eq!(destination.config().unwrap().accounts.len(), 2);
    }

    #[test]
    fn invalid_or_unselected_accounts_do_not_initialize_a_destination() {
        let directory = tempfile::tempdir().unwrap();
        let destination = store(directory.path());
        assert!(
            restore(
                &destination,
                &bundle(),
                Some("missing@example.com"),
                true,
                false
            )
            .is_err()
        );
        let mut invalid = bundle();
        invalid.accounts[1].email = "ALICE@example.com".into();
        assert!(restore(&destination, &invalid, None, true, false).is_err());
        invalid.accounts[1].email = "other@example.com".into();
        invalid.accounts[0].refresh_token.clear();
        assert!(restore(&destination, &invalid, None, true, false).is_err());
        assert!(!destination.root.exists());
        let invalid_file = directory.path().join("invalid.joog");
        fs::write(&invalid_file, b"not a credential export").unwrap();
        assert!(
            import(&destination, None, &invalid_file, None, true, false)
                .unwrap_err()
                .to_string()
                .contains("invalid or unsupported")
        );
        assert!(!destination.root.exists());
    }

    #[test]
    fn passphrase_files_accept_one_final_newline_and_refuse_public_files() {
        let directory = tempfile::tempdir().unwrap();
        let file = password_file(directory.path(), b"private passphrase\r\n");
        assert_eq!(
            passphrase(Some(file.path()), false).unwrap().as_str(),
            "private passphrase"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(file.path(), fs::Permissions::from_mode(0o644)).unwrap();
            assert!(
                passphrase(Some(file.path()), false)
                    .unwrap_err()
                    .to_string()
                    .contains("chmod 600")
            );
        }
    }

    #[test]
    fn exporting_all_accounts_uses_the_enrolled_client_and_refresh_tokens_only() {
        let directory = tempfile::tempdir().unwrap();
        let source = store(directory.path());
        restore(&source, &bundle(), None, true, false).unwrap();
        source
            .save_config(&Config {
                account: Some("other@example.com".into()),
                accounts: vec!["other@example.com".into(), "alice@example.com".into()],
            })
            .unwrap();
        let collected = collect(&source, None).unwrap();
        assert_eq!(
            collected
                .accounts
                .iter()
                .map(|account| account.email.as_str())
                .collect::<Vec<_>>(),
            ["alice@example.com", "other@example.com"]
        );
        let raw = Zeroizing::new(serde_json::to_string(&collected).unwrap());
        assert!(!raw.contains("access_token"));
        assert!(!raw.contains("master"));
        assert_eq!(collected.client.client_id, "private-client-id");
    }
}
