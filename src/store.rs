use aes_gcm::{
    Aes256Gcm, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use anyhow::{Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use zeroize::Zeroizing;

#[cfg(not(target_os = "macos"))]
static KEYRING: std::sync::LazyLock<keyring_core::Result<()>> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    let store = windows_native_keyring_store::Store::new()?;
    #[cfg(all(unix, not(target_os = "macos")))]
    let store = dbus_secret_service_keyring_store::Store::new()?;
    #[cfg(any(unix, windows))]
    {
        keyring_core::set_default_store(store);
        Ok(())
    }
    #[cfg(not(any(unix, windows)))]
    Err(keyring_core::Error::NoDefaultStore)
});

const MAGIC: &[u8; 8] = b"GOGVLT01";
const SECRET_LIMIT: u64 = 1024 * 1024;

trait Keychain: Send + Sync {
    fn load(&self, namespace: &str) -> Result<Option<Zeroizing<Vec<u8>>>>;
    fn save(&self, namespace: &str, key: &[u8]) -> Result<()>;
}

struct OsKeychain;

impl OsKeychain {
    #[cfg(not(target_os = "macos"))]
    fn entry(namespace: &str) -> Result<keyring_core::Entry> {
        KEYRING
            .as_ref()
            .map_err(|e| anyhow::anyhow!("OS keychain unavailable: {e}"))?;
        keyring_core::Entry::new(namespace, "master-key").context("cannot access master key")
    }
}

impl Keychain for OsKeychain {
    fn load(&self, namespace: &str) -> Result<Option<Zeroizing<Vec<u8>>>> {
        #[cfg(target_os = "macos")]
        return macos::load(namespace);
        #[cfg(not(target_os = "macos"))]
        match Self::entry(namespace)?.get_secret() {
            Ok(key) => Ok(Some(Zeroizing::new(key))),
            Err(keyring_core::Error::NoEntry) => Ok(None),
            Err(e) => Err(e).context("cannot unlock master key in OS keychain"),
        }
    }

    fn save(&self, namespace: &str, key: &[u8]) -> Result<()> {
        #[cfg(target_os = "macos")]
        return macos::save(namespace, key);
        #[cfg(not(target_os = "macos"))]
        Self::entry(namespace)?
            .set_secret(key)
            .context("cannot save master key in OS keychain")
    }
}

#[cfg(target_os = "macos")]
#[path = "store/macos.rs"]
mod macos;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct VaultIdentity {
    version: u32,
    service: String,
    #[serde(default, skip_serializing_if = "KeyStorage::is_keychain")]
    key_storage: KeyStorage,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum KeyStorage {
    #[default]
    OsKeychain,
    File,
}

impl KeyStorage {
    fn is_keychain(&self) -> bool {
        *self == Self::OsKeychain
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub account: Option<String>,
    #[serde(default)]
    pub accounts: Vec<String>,
}

pub struct Store {
    pub root: PathBuf,
    keychain: Arc<dyn Keychain>,
    legacy_root: Option<PathBuf>,
    default_vault: bool,
    key: Mutex<Option<Zeroizing<Vec<u8>>>>,
}

fn validate_key(key: &[u8]) -> Result<()> {
    anyhow::ensure!(
        key.len() == 32,
        "invalid master key length; refusing to replace it"
    );
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut result, "{byte:02x}").expect("write to string");
    }
    result
}

fn valid_service(service: &str) -> bool {
    service == "joog/vault"
        || service
            .strip_prefix("joog/vault/")
            .is_some_and(|id| id.len() == 32 && id.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

fn legacy_namespace(root: &Path) -> Result<String> {
    Ok(format!(
        "gogcli-rust/vault/{:x}",
        Sha256::digest(std::path::absolute(root)?.to_string_lossy().as_bytes())
    ))
}

fn has_state(root: &Path) -> Result<bool> {
    match fs::symlink_metadata(root) {
        Ok(metadata) => {
            anyhow::ensure!(metadata.is_dir(), "credential path is not a directory");
            Ok(fs::read_dir(root)?.next().is_some())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

impl Store {
    pub fn new(home: Option<&str>) -> Result<Self> {
        let (root, legacy_root) = if let Some(home) = home {
            (
                PathBuf::from(home).join("joog"),
                PathBuf::from(home).join("rust"),
            )
        } else {
            let base = directories_next::BaseDirs::new()
                .context("cannot resolve config directory; use --home")?
                .config_dir()
                .to_owned();
            (base.join("joog"), base.join("gogcli-rust"))
        };
        let root = if has_state(&legacy_root)? {
            anyhow::ensure!(
                !has_state(&root)?,
                "both {} and {} contain state; refusing to choose or change either vault",
                legacy_root.display(),
                root.display()
            );
            legacy_root.clone()
        } else {
            root
        };
        Ok(Self {
            root,
            keychain: Arc::new(OsKeychain),
            legacy_root: Some(legacy_root),
            default_vault: home.is_none(),
            key: Mutex::new(None),
        })
    }

    pub fn config(&self) -> Result<Config> {
        match fs::File::open(self.root.join("config.json")) {
            Ok(file) => serde_json::from_slice(&crate::http::limited(file, SECRET_LIMIT)?)
                .context("invalid config.json"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        atomic(
            &self.root.join("config.json"),
            &serde_json::to_vec_pretty(config)?,
        )
    }

    fn identity(&self) -> Result<Option<VaultIdentity>> {
        let path = self.root.join("vault.json");
        match fs::symlink_metadata(&path) {
            Ok(metadata) => {
                anyhow::ensure!(metadata.is_file(), "invalid vault identity file");
                let identity: VaultIdentity =
                    serde_json::from_slice(&crate::http::limited(fs::File::open(&path)?, 1024)?)
                        .context("invalid vault identity")?;
                anyhow::ensure!(
                    identity.version == 1 && valid_service(&identity.service),
                    "invalid vault identity"
                );
                Ok(Some(identity))
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn namespace(&self) -> Result<String> {
        if let Some(identity) = self.identity()? {
            Ok(identity.service)
        } else if self.legacy_root.as_ref() == Some(&self.root) {
            legacy_namespace(&self.root)
        } else if self.default_vault {
            Ok("joog/vault".to_owned())
        } else {
            anyhow::bail!("vault is not initialized; run joog auth init")
        }
    }

    pub fn uses_key_file(&self) -> Result<bool> {
        Ok(self
            .identity()?
            .is_some_and(|identity| identity.key_storage == KeyStorage::File))
    }

    fn load_file_key(&self) -> Result<Option<Zeroizing<Vec<u8>>>> {
        let path = self.root.join("master.key");
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        anyhow::ensure!(
            metadata.is_file(),
            "invalid master.key file; symbolic links are not accepted"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            anyhow::ensure!(
                metadata.permissions().mode() & 0o077 == 0,
                "master.key must be readable only by its owner; run chmod 600 on that file"
            );
        }
        let key = Zeroizing::new(crate::http::limited(fs::File::open(&path)?, 32)?);
        validate_key(&key)?;
        Ok(Some(key))
    }

    fn lock(&self) -> Result<fs::File> {
        private_dir(&self.root)?;
        let path = self.root.join(".vault.lock");
        if let Ok(meta) = fs::symlink_metadata(&path) {
            anyhow::ensure!(meta.is_file(), "invalid vault lock file");
        }
        let mut options = fs::OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options.open(path)?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match FileExt::try_lock_exclusive(&file) {
                Ok(()) => return Ok(file),
                Err(e)
                    if e.raw_os_error() == fs2::lock_contended_error().raw_os_error()
                        && std::time::Instant::now() < deadline =>
                {
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                Err(e) => return Err(e).context("cannot lock credential vault"),
            }
        }
    }

    pub(crate) fn master_key(&self) -> Result<Zeroizing<Vec<u8>>> {
        let _lock = self.lock()?;
        self.load_key()?
            .context("master key missing; existing encrypted secrets require their original key")
    }

    fn load_key(&self) -> Result<Option<Zeroizing<Vec<u8>>>> {
        let mut cached = self
            .key
            .lock()
            .map_err(|_| anyhow::anyhow!("credential session unavailable"))?;
        if let Some(key) = cached.as_ref() {
            return Ok(Some(key.clone()));
        }
        if self.uses_key_file()? {
            let key = self.load_file_key()?;
            *cached = key.clone();
            return Ok(key);
        }
        let namespace = self.namespace()?;
        let key = if let Some(key) = self.keychain.load(&namespace)? {
            Some(key)
        } else if let Some(root) = &self.legacy_root {
            let old_service = legacy_namespace(root)?;
            if old_service == namespace {
                None
            } else {
                self.keychain.load(&old_service)?
            }
        } else {
            None
        };
        if let Some(key) = key {
            validate_key(&key)?;
            *cached = Some(key.clone());
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn retain_key(&self, namespace: &str, key: &[u8]) -> Result<()> {
        self.keychain.save(namespace, key)?;
        let saved = self.keychain.load(namespace)?
            .context("master key missing; run joog auth init; existing encrypted secrets require their original key")?;
        anyhow::ensure!(
            bool::from(subtle::ConstantTimeEq::ct_eq(key, saved.as_slice())),
            "OS keychain did not retain the master key"
        );
        Ok(())
    }

    pub fn initialize(&self) -> Result<()> {
        let _lock = self.lock()?;
        self.initialize_locked(false)
    }

    pub fn initialize_headless(&self) -> Result<()> {
        let _lock = self.lock()?;
        self.initialize_locked(true)
    }

    fn initialize_locked(&self, headless: bool) -> Result<()> {
        if headless {
            anyhow::ensure!(
                self.legacy_root.as_ref() != Some(&self.root),
                "this is a legacy keychain vault; use --home with a new directory for a headless vault"
            );
            anyhow::ensure!(
                self.identity()?.is_none() || self.uses_key_file()?,
                "this vault uses the OS keychain; use --home with a new directory for a headless vault"
            );
        }
        let records = self.root.join("secrets");
        if records.exists() && fs::read_dir(&records)?.next().is_some() {
            anyhow::ensure!(
                !headless || self.uses_key_file()?,
                "existing encrypted credentials require their original keychain; use --home with a new directory"
            );
            // Initializing storage is not a credential read or an upgrade step.
            // Existing credentials and their Keychain permissions stay intact.
            return Ok(());
        }
        if !self.root.join("vault.json").exists() && self.legacy_root.as_ref() != Some(&self.root) {
            let service = if self.default_vault {
                "joog/vault".to_owned()
            } else {
                let mut id = [0_u8; 16];
                getrandom::fill(&mut id).context("operating system randomness unavailable")?;
                format!("joog/vault/{}", hex(&id))
            };
            atomic(
                &self.root.join("vault.json"),
                &serde_json::to_vec(&VaultIdentity {
                    version: 1,
                    service,
                    key_storage: if headless {
                        KeyStorage::File
                    } else {
                        KeyStorage::OsKeychain
                    },
                })?,
            )?;
        }
        if self.load_key()?.is_some() {
            return Ok(());
        }
        let mut key = Zeroizing::new(vec![0_u8; 32]);
        getrandom::fill(&mut key).context("operating system randomness unavailable")?;
        if self.uses_key_file()? {
            let path = self.root.join("master.key");
            anyhow::ensure!(
                !path.try_exists()?,
                "refusing to replace an existing master.key"
            );
            atomic(&path, &key)?;
        } else {
            self.retain_key(&self.namespace()?, &key)?;
        }
        *self
            .key
            .lock()
            .map_err(|_| anyhow::anyhow!("credential session unavailable"))? = Some(key);
        Ok(())
    }

    fn record(&self, name: &str) -> PathBuf {
        self.root
            .join("secrets")
            .join(format!("{:x}.enc", Sha256::digest(name.as_bytes())))
    }

    pub fn secret<T: DeserializeOwned>(&self, name: &str) -> Result<T> {
        let _lock = self.lock()?;
        let key = self
            .load_key()?
            .context("master key missing; existing encrypted secrets require their original key")?;
        let path = self.record(name);
        let meta = fs::symlink_metadata(&path).context(
            "encrypted credential missing; run joog auth enroll or joog auth <account> login <service-list>",
        )?;
        anyhow::ensure!(meta.is_file(), "invalid encrypted credential file");
        let bytes = crate::http::limited(fs::File::open(path)?, SECRET_LIMIT + 64)?;
        let raw = decrypt(&key, name, &bytes)?;
        serde_json::from_slice(&raw).context("invalid encrypted credential data")
    }

    pub fn set_secret(&self, name: &str, secret: &impl Serialize) -> Result<()> {
        let _lock = self.lock()?;
        let key = self
            .load_key()?
            .context("master key missing; existing encrypted secrets require their original key")?;
        let raw = Zeroizing::new(serde_json::to_vec(secret)?);
        anyhow::ensure!(
            raw.len() as u64 <= SECRET_LIMIT,
            "credential exceeds size limit"
        );
        atomic(&self.record(name), &encrypt(&key, name, &raw)?)
    }

    pub fn delete_secret(&self, name: &str) -> Result<()> {
        let _lock = self.lock()?;
        match fs::remove_file(self.record(name)) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e).context("cannot remove encrypted credential"),
        }
    }

    pub(crate) fn import_credentials(
        &self,
        records: &[(String, Zeroizing<Vec<u8>>)],
        accounts: &[String],
        replace: bool,
        headless: bool,
    ) -> Result<()> {
        let _lock = self.lock()?;
        self.initialize_locked(headless)?;
        let key = self
            .load_key()?
            .context("master key missing; existing encrypted secrets require their original key")?;
        let mut config = self.config()?;
        let mut writes = Vec::new();
        for (name, raw) in records {
            let path = self.record(name);
            let previous = match fs::symlink_metadata(&path) {
                Ok(metadata) => {
                    anyhow::ensure!(metadata.is_file(), "invalid encrypted credential file");
                    Some(crate::http::limited(
                        fs::File::open(&path)?,
                        SECRET_LIMIT + 64,
                    )?)
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
                Err(error) => return Err(error.into()),
            };
            if let Some(previous) = &previous {
                let current = decrypt(&key, name, previous)?;
                if name == "oauth-client" {
                    if current.as_slice() == raw.as_slice() {
                        continue;
                    }
                    anyhow::ensure!(
                        replace,
                        "this vault has different OAuth client credentials; use --replace to replace them, or --home for a separate vault"
                    );
                    anyhow::ensure!(
                        config
                            .accounts
                            .iter()
                            .all(|account| accounts.contains(account)),
                        "replacing this OAuth client would affect other accounts; use --home with a separate vault"
                    );
                } else {
                    anyhow::ensure!(
                        replace,
                        "credentials for {} already exist; use --replace to replace them",
                        name.strip_prefix("account/").unwrap_or(name)
                    );
                }
            }
            anyhow::ensure!(
                raw.len() as u64 <= SECRET_LIMIT,
                "credential exceeds size limit"
            );
            writes.push((path, encrypt(&key, name, raw)?, previous));
        }
        for account in accounts {
            if !config.accounts.contains(account) {
                config.accounts.push(account.clone());
            }
        }
        if config.account.is_none() {
            config.account = accounts.first().cloned();
        }
        let config_path = self.root.join("config.json");
        let previous_config = match fs::File::open(&config_path) {
            Ok(file) => Some(crate::http::limited(file, SECRET_LIMIT)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.into()),
        };
        writes.push((
            config_path,
            serde_json::to_vec_pretty(&config)?,
            previous_config,
        ));
        for (index, (path, bytes, _)) in writes.iter().enumerate() {
            if let Err(error) = atomic(path, bytes) {
                for (path, _, previous) in writes[..index].iter().rev() {
                    match previous {
                        Some(previous) => atomic(path, previous),
                        None => fs::remove_file(path).map_err(Into::into),
                    }
                    .context("credential import failed and could not restore the previous vault")?;
                }
                return Err(error)
                    .context("credential import failed; previous credentials restored");
            }
        }
        Ok(())
    }
}

fn aad(name: &str) -> Vec<u8> {
    [MAGIC.as_slice(), name.as_bytes()].concat()
}

pub(crate) fn encrypt(key: &[u8], name: &str, raw: &[u8]) -> Result<Vec<u8>> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|_| anyhow::anyhow!("invalid master key"))?;
    let mut nonce = [0_u8; 12];
    getrandom::fill(&mut nonce).context("operating system randomness unavailable")?;
    let ciphertext = cipher
        .encrypt(
            &Nonce::from(nonce),
            Payload {
                msg: raw,
                aad: &aad(name),
            },
        )
        .map_err(|_| anyhow::anyhow!("credential encryption failed"))?;
    Ok([MAGIC.as_slice(), &nonce, &ciphertext].concat())
}

pub(crate) fn decrypt(key: &[u8], name: &str, bytes: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
    anyhow::ensure!(
        bytes.len() >= 36 && &bytes[..8] == MAGIC,
        "invalid encrypted credential format"
    );
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|_| anyhow::anyhow!("invalid master key"))?;
    let nonce: [u8; 12] = bytes[8..20].try_into()?;
    let raw = cipher
        .decrypt(
            &Nonce::from(nonce),
            Payload {
                msg: &bytes[20..],
                aad: &aad(name),
            },
        )
        .map_err(|_| {
            anyhow::anyhow!(
                "credential authentication failed; data was modified or the master key is incorrect"
            )
        })?;
    Ok(Zeroizing::new(raw))
}

pub fn private_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

pub fn atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().context("file has no parent directory")?;
    private_dir(parent)?;
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    tmp.write_all(bytes)?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map_err(|e| e.error)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MemoryKeys {
        keys: std::sync::Mutex<std::collections::BTreeMap<String, Vec<u8>>>,
        saves: std::sync::atomic::AtomicUsize,
        locked: std::sync::atomic::AtomicBool,
        loads: std::sync::atomic::AtomicUsize,
        legacy_needs_approval: std::sync::atomic::AtomicBool,
    }

    impl Keychain for MemoryKeys {
        fn load(&self, namespace: &str) -> Result<Option<Zeroizing<Vec<u8>>>> {
            self.loads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            anyhow::ensure!(
                !self.locked.load(std::sync::atomic::Ordering::SeqCst),
                "keychain locked"
            );
            anyhow::ensure!(
                !namespace.starts_with("gogcli-rust/")
                    || !self
                        .legacy_needs_approval
                        .load(std::sync::atomic::Ordering::SeqCst),
                "legacy approval required"
            );
            Ok(self
                .keys
                .lock()
                .unwrap()
                .get(namespace)
                .cloned()
                .map(Zeroizing::new))
        }
        fn save(&self, namespace: &str, key: &[u8]) -> Result<()> {
            self.keys
                .lock()
                .unwrap()
                .insert(namespace.to_owned(), key.to_vec());
            self.saves.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        }
    }

    fn vault() -> (tempfile::TempDir, Store, Arc<MemoryKeys>) {
        let dir = tempfile::tempdir().unwrap();
        let keys = Arc::new(MemoryKeys::default());
        let store = Store {
            root: dir.path().join("vault"),
            keychain: keys.clone(),
            legacy_root: None,
            default_vault: false,
            key: Mutex::new(None),
        };
        (dir, store, keys)
    }

    #[test]
    fn headless_vaults_work_across_processes_without_contacting_a_keychain() {
        let (_directory, store, keys) = vault();
        keys.locked.store(true, std::sync::atomic::Ordering::SeqCst);
        store.initialize_headless().unwrap();
        store
            .set_secret("test", &serde_json::json!({"secret":"private-value"}))
            .unwrap();
        let reopened = Store {
            root: store.root.clone(),
            keychain: keys.clone(),
            legacy_root: None,
            default_vault: false,
            key: Mutex::new(None),
        };
        reopened.initialize().unwrap();
        assert_eq!(
            reopened.secret::<serde_json::Value>("test").unwrap()["secret"],
            "private-value"
        );
        assert_eq!(keys.loads.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert_eq!(keys.saves.load(std::sync::atomic::Ordering::SeqCst), 0);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&store.root).unwrap().permissions().mode() & 0o777,
                0o700
            );
            assert_eq!(
                fs::metadata(store.root.join("master.key"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
    }

    #[test]
    fn headless_initialization_never_converts_an_existing_keychain_vault() {
        let (_directory, store, _keys) = vault();
        store.initialize().unwrap();
        let identity = fs::read(store.root.join("vault.json")).unwrap();
        assert!(store.initialize_headless().is_err());
        assert_eq!(fs::read(store.root.join("vault.json")).unwrap(), identity);
        assert!(!store.root.join("master.key").exists());
    }

    #[cfg(unix)]
    #[test]
    fn headless_keys_refuse_public_permissions_symlinks_and_regeneration_for_existing_records() {
        use std::os::unix::fs::{PermissionsExt, symlink};
        let (directory, store, keys) = vault();
        store.initialize_headless().unwrap();
        store.set_secret("test", &"private-value").unwrap();
        let key_path = store.root.join("master.key");
        let key = Zeroizing::new(fs::read(&key_path).unwrap());
        let encrypted = fs::read(store.record("test")).unwrap();
        let reopened = || Store {
            root: store.root.clone(),
            keychain: keys.clone(),
            legacy_root: None,
            default_vault: false,
            key: Mutex::new(None),
        };
        fs::set_permissions(&key_path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(
            reopened()
                .secret::<String>("test")
                .unwrap_err()
                .to_string()
                .contains("chmod 600")
        );
        fs::remove_file(&key_path).unwrap();
        let outside = directory.path().join("outside-key");
        fs::write(&outside, &key).unwrap();
        symlink(&outside, &key_path).unwrap();
        assert!(
            reopened()
                .secret::<String>("test")
                .unwrap_err()
                .to_string()
                .contains("symbolic links")
        );
        fs::remove_file(&key_path).unwrap();
        reopened().initialize_headless().unwrap();
        assert!(!key_path.exists());
        assert!(reopened().secret::<String>("test").is_err());
        assert_eq!(fs::read(store.record("test")).unwrap(), encrypted);
        assert_eq!(keys.loads.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[test]
    fn import_into_a_keychain_vault_reencrypts_records_with_its_own_key() {
        let (_directory, store, keys) = vault();
        let client = Zeroizing::new(
            br#"{"client_id":"private-id","client_secret":"private-secret"}"#.to_vec(),
        );
        let token = Zeroizing::new(br#"{"refresh_token":"private-refresh"}"#.to_vec());
        store
            .import_credentials(
                &[
                    ("oauth-client".into(), client),
                    ("account/me@example.com".into(), token),
                ],
                &["me@example.com".into()],
                false,
                false,
            )
            .unwrap();
        assert!(!store.uses_key_file().unwrap());
        assert_eq!(keys.saves.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(
            store.secret::<serde_json::Value>("oauth-client").unwrap()["client_secret"],
            "private-secret"
        );
        assert_eq!(store.config().unwrap().accounts, ["me@example.com"]);
    }

    #[test]
    fn credentials_and_tokens_are_encrypted_with_one_master_key() {
        let (_dir, store, keys) = vault();
        store.initialize().unwrap();
        store.initialize().unwrap();
        for (name, value) in [
            (
                "oauth-client",
                serde_json::json!({"client_id":"private-client-id", "client_secret":"private-client-secret"}),
            ),
            (
                "account/me@example.com",
                serde_json::json!({"access_token":"private-access-token", "refresh_token":"private-refresh-token"}),
            ),
        ] {
            store.set_secret(name, &value).unwrap();
            let first = fs::read(store.record(name)).unwrap();
            for secret in [
                "private-client-id",
                "private-client-secret",
                "private-access-token",
                "private-refresh-token",
            ] {
                assert!(!first.windows(secret.len()).any(|w| w == secret.as_bytes()));
            }
            assert_eq!(store.secret::<serde_json::Value>(name).unwrap(), value);
            store.set_secret(name, &value).unwrap();
            assert_ne!(first, fs::read(store.record(name)).unwrap());
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                assert_eq!(
                    fs::metadata(store.record(name))
                        .unwrap()
                        .permissions()
                        .mode()
                        & 0o777,
                    0o600
                );
            }
        }
        assert_eq!(keys.saves.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(keys.keys.lock().unwrap().len(), 1);
        assert_eq!(keys.keys.lock().unwrap().values().next().unwrap().len(), 32);
        store.delete_secret("oauth-client").unwrap();
        assert!(!store.record("oauth-client").exists());
        assert!(
            store
                .secret::<serde_json::Value>("account/me@example.com")
                .is_ok()
        );
    }

    #[test]
    fn rejects_tampering_truncation_wrong_key_and_record_swapping() {
        let (_dir, store, keys) = vault();
        store.initialize().unwrap();
        store.set_secret("first", &"secret-value").unwrap();
        let bytes = fs::read(store.record("first")).unwrap();
        for offset in [0, 8, 20, bytes.len() - 1] {
            let mut altered = bytes.clone();
            altered[offset] ^= 1;
            fs::write(store.record("first"), altered).unwrap();
            assert!(store.secret::<String>("first").is_err());
        }
        fs::write(store.record("first"), &bytes[..30]).unwrap();
        assert!(store.secret::<String>("first").is_err());
        fs::write(store.record("first"), &bytes).unwrap();
        fs::write(store.record("second"), &bytes).unwrap();
        assert!(store.secret::<String>("second").is_err());
        keys.keys
            .lock()
            .unwrap()
            .insert(store.namespace().unwrap(), vec![5; 32]);
        *store.key.lock().unwrap() = None;
        assert!(store.secret::<String>("first").is_err());
    }

    #[test]
    fn missing_or_locked_keys_never_regenerate_or_overwrite_secrets() {
        let (_dir, store, keys) = vault();
        assert!(store.set_secret("client", &"secret-value").is_err());
        store.initialize().unwrap();
        store.set_secret("client", &"secret-value").unwrap();
        let bytes = fs::read(store.record("client")).unwrap();
        *store.key.lock().unwrap() = None;
        keys.locked.store(true, std::sync::atomic::Ordering::SeqCst);
        let loads = keys.loads.load(std::sync::atomic::Ordering::SeqCst);
        store.initialize().unwrap();
        assert_eq!(keys.loads.load(std::sync::atomic::Ordering::SeqCst), loads);
        assert!(store.secret::<String>("client").is_err());
        assert!(store.set_secret("client", &"replacement").is_err());
        keys.locked
            .store(false, std::sync::atomic::Ordering::SeqCst);
        keys.keys.lock().unwrap().clear();
        store.initialize().unwrap();
        assert!(store.secret::<String>("client").is_err());
        assert_eq!(fs::read(store.record("client")).unwrap(), bytes);
        assert_eq!(keys.saves.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn concurrent_initialization_creates_exactly_one_key() {
        let (_dir, store, keys) = vault();
        let store = Arc::new(store);
        let threads = (0..8)
            .map(|_| {
                let store = store.clone();
                std::thread::spawn(move || store.initialize().unwrap())
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(keys.saves.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn one_keychain_read_serves_the_whole_credential_session() {
        let (_dir, store, keys) = vault();
        store.initialize().unwrap();
        store.set_secret("client", &"retained client").unwrap();
        *store.key.lock().unwrap() = None;
        keys.loads.store(0, std::sync::atomic::Ordering::SeqCst);
        for _ in 0..4 {
            assert_eq!(store.secret::<String>("client").unwrap(), "retained client");
            store.set_secret("token", &"updated token").unwrap();
            store.master_key().unwrap();
            store.initialize().unwrap();
        }
        assert_eq!(keys.loads.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn legacy_access_is_read_only_and_init_never_requests_keychain_access() {
        let dir = tempfile::tempdir().unwrap();
        let legacy = dir.path().join("rust");
        private_dir(&legacy.join("secrets")).unwrap();
        let key = [29; 32];
        let encrypted = encrypt(
            &key,
            "oauth-client",
            br#"{"client_id":"existing","client_secret":"retained"}"#,
        )
        .unwrap();
        let filename = format!("{:x}.enc", Sha256::digest(b"oauth-client"));
        atomic(&legacy.join("secrets").join(&filename), &encrypted).unwrap();
        atomic(
            &legacy.join("config.json"),
            br#"{"accounts":["existing@example.com"]}"#,
        )
        .unwrap();
        let keys = Arc::new(MemoryKeys::default());
        let old_service = legacy_namespace(&legacy).unwrap();
        keys.keys
            .lock()
            .unwrap()
            .insert(old_service.clone(), key.to_vec());
        keys.legacy_needs_approval
            .store(true, std::sync::atomic::Ordering::SeqCst);
        let mut store = Store::new(dir.path().to_str()).unwrap();
        store.keychain = keys.clone();
        assert!(legacy.exists());
        assert_eq!(store.root, legacy);
        assert_eq!(store.config().unwrap().accounts, ["existing@example.com"]);
        assert!(store.secret::<serde_json::Value>("oauth-client").is_err());
        let loads = keys.loads.load(std::sync::atomic::Ordering::SeqCst);
        store.initialize().unwrap();
        assert_eq!(keys.loads.load(std::sync::atomic::Ordering::SeqCst), loads);
        assert!(store.secret::<serde_json::Value>("oauth-client").is_err());
        keys.legacy_needs_approval
            .store(false, std::sync::atomic::Ordering::SeqCst);
        let credential: serde_json::Value = store.secret("oauth-client").unwrap();
        assert_eq!(credential["client_secret"], "retained");
        assert_eq!(fs::read(store.record("oauth-client")).unwrap(), encrypted);
        let service = store.namespace().unwrap();
        assert_eq!(service, old_service);
        assert_eq!(keys.keys.lock().unwrap()[&service], key);
        assert_eq!(keys.keys.lock().unwrap()[&old_service], key);
        assert!(!store.root.join("vault.json").exists());
        assert!(!dir.path().join("joog").exists());
        assert_eq!(keys.saves.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert_eq!(keys.keys.lock().unwrap().len(), 1);
    }

    #[test]
    fn conflicting_directories_and_malformed_identity_never_replace_credentials() {
        let dir = tempfile::tempdir().unwrap();
        for name in ["rust", "joog"] {
            private_dir(&dir.path().join(name)).unwrap();
            atomic(&dir.path().join(name).join("config.json"), name.as_bytes()).unwrap();
        }
        assert!(Store::new(dir.path().to_str()).is_err());
        assert_eq!(
            fs::read(dir.path().join("rust/config.json")).unwrap(),
            b"rust"
        );
        assert_eq!(
            fs::read(dir.path().join("joog/config.json")).unwrap(),
            b"joog"
        );
        let (_dir, store, keys) = vault();
        private_dir(&store.root).unwrap();
        for service in ["joog/vault\nquit", "other", "joog/vault/../other"] {
            atomic(
                &store.root.join("vault.json"),
                &serde_json::to_vec(&serde_json::json!({"version":1,"service":service})).unwrap(),
            )
            .unwrap();
            assert!(store.initialize().is_err());
        }
        assert_eq!(keys.saves.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[test]
    #[ignore = "requires an unlocked OS keychain"]
    fn real_os_keychain_encrypts_and_decrypts() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_str()).unwrap();
        store.initialize().unwrap();
        struct Cleanup(String);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                #[cfg(target_os = "macos")]
                let _ = std::process::Command::new("/usr/bin/security")
                    .args(["delete-generic-password", "-s", &self.0, "-a", "master-key"])
                    .output();
                #[cfg(not(target_os = "macos"))]
                let _ = OsKeychain::entry(&self.0)
                    .and_then(|entry| entry.delete_credential().map_err(Into::into));
            }
        }
        let _cleanup = Cleanup(store.namespace().unwrap());
        store
            .set_secret("test-client", &"test-secret-value")
            .unwrap();
        assert_eq!(
            store.secret::<String>("test-client").unwrap(),
            "test-secret-value"
        );
        let namespace = store.namespace().unwrap();
        let encrypted = fs::read(store.record("test-client")).unwrap();
        drop(store);
        for _ in 0..2 {
            let reopened = Store::new(dir.path().to_str()).unwrap();
            assert_eq!(reopened.namespace().unwrap(), namespace);
            reopened.initialize().unwrap();
            assert_eq!(
                reopened.secret::<String>("test-client").unwrap(),
                "test-secret-value"
            );
            assert_eq!(fs::read(reopened.record("test-client")).unwrap(), encrypted);
        }
        #[cfg(not(target_os = "macos"))]
        assert_native_legacy_access_is_read_only();
    }

    #[cfg(not(target_os = "macos"))]
    fn assert_native_legacy_access_is_read_only() {
        let directory = tempfile::tempdir().unwrap();
        let legacy = directory.path().join("rust");
        private_dir(&legacy.join("secrets")).unwrap();
        let old_service = legacy_namespace(&legacy).unwrap();
        let key = [47; 32];
        OsKeychain.save(&old_service, &key).unwrap();
        struct Cleanup(Vec<String>);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                for service in &self.0 {
                    if let Ok(entry) = OsKeychain::entry(service) {
                        let _ = entry.delete_credential();
                    }
                }
            }
        }
        let mut cleanup = Cleanup(vec![old_service.clone()]);
        let filename = format!("{:x}.enc", Sha256::digest(b"test-client"));
        let encrypted = encrypt(&key, "test-client", br#""existing test credential""#).unwrap();
        atomic(&legacy.join("secrets").join(&filename), &encrypted).unwrap();
        let store = Store::new(directory.path().to_str()).unwrap();
        let service = store.namespace().unwrap();
        assert_eq!(service, old_service);
        assert_eq!(store.root, legacy);
        store.initialize().unwrap();
        assert_eq!(
            store.secret::<String>("test-client").unwrap(),
            "existing test credential"
        );
        assert_eq!(
            OsKeychain.load(&old_service).unwrap().unwrap().as_slice(),
            key
        );
        assert_eq!(OsKeychain.load(&service).unwrap().unwrap().as_slice(), key);
        assert_eq!(fs::read(store.record("test-client")).unwrap(), encrypted);
        assert!(!directory.path().join("joog").exists());
        assert!(!legacy.join("vault.json").exists());
        for service in &cleanup.0 {
            OsKeychain::entry(service)
                .unwrap()
                .delete_credential()
                .unwrap();
            assert!(OsKeychain.load(service).unwrap().is_none());
        }
        cleanup.0.clear();
    }

    #[test]
    fn config_root_is_isolated_and_private() {
        let tmp = tempfile::tempdir().unwrap();
        let store = Store::new(tmp.path().to_str()).unwrap();
        assert_eq!(store.root, tmp.path().join("joog"));
        assert!(store.config().unwrap().accounts.is_empty());
        store
            .save_config(&Config {
                account: Some("me@example.com".into()),
                accounts: vec!["me@example.com".into()],
            })
            .unwrap();
        assert_eq!(
            store.config().unwrap().account.as_deref(),
            Some("me@example.com")
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(store.root.join("config.json"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
    }
}
