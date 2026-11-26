use crate::{http, store};
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use zeroize::Zeroizing;

const CAPACITY: u64 = 64 * 1024 * 1024;
const ENTRIES: usize = 32;

pub fn request_identity(
    doc: &crate::discovery::Document,
    method: &crate::discovery::MethodSpec,
    params: &serde_json::Map<String, Value>,
    relative_times: &[&str],
) -> Result<String> {
    let mut params = params.clone();
    for key in relative_times {
        params.remove(*key);
    }
    let mut identity = doc.url(method, &params)?;
    if !relative_times.is_empty() {
        // This fragment distinguishes a relative window from an unbounded request.
        // The identity is only hashed for caching; it is never sent to Google.
        identity.set_fragment(Some(&format!(
            "joog-relative-time={}",
            relative_times.join(",")
        )));
    }
    Ok(identity.into())
}

pub fn body_identity(mut identity: String, body: Option<&Value>) -> Result<String> {
    if let Some(body) = body {
        let digest = Sha256::digest(serde_json::to_vec(body)?);
        identity.push_str(if identity.contains('#') { "&" } else { "#" });
        use std::fmt::Write;
        write!(&mut identity, "joog-body={digest:x}")?;
    }
    Ok(identity)
}

pub struct Responses {
    root: PathBuf,
    account: String,
    key: Zeroizing<Vec<u8>>,
    ttl: Duration,
}

fn directory(root: &Path, account: &str) -> PathBuf {
    root.join("responses").join(format!(
        "{:x}",
        Sha256::digest(account.to_ascii_lowercase().as_bytes())
    ))
}

pub fn invalidate(root: &Path, account: &str) -> Result<()> {
    let directory = directory(root, account);
    if directory.exists() {
        let mut generation = [0_u8; 32];
        getrandom::fill(&mut generation).context("operating system randomness unavailable")?;
        store::atomic(&directory.join("generation"), &generation)?;
    }
    Ok(())
}

impl Responses {
    pub fn new(root: &Path, account: &str, key: Zeroizing<Vec<u8>>, minutes: u32) -> Result<Self> {
        let cache = Self {
            root: directory(root, account),
            account: account.to_ascii_lowercase(),
            key,
            ttl: Duration::from_secs(u64::from(minutes) * 60),
        };
        if minutes > 0 {
            store::private_dir(cache.root.parent().context("cache has no parent")?)?;
            store::private_dir(&cache.root)?;
        }
        Ok(cache)
    }

    fn record(&self, url: &str) -> Result<String> {
        let generation = match fs::File::open(self.root.join("generation")) {
            Ok(file) => http::limited(file, 32)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(error) => return Err(error.into()),
        };
        let mut digest = Sha256::new();
        digest.update(self.account.as_bytes());
        digest.update([0]);
        digest.update(url.as_bytes());
        digest.update([0]);
        digest.update(generation);
        Ok(format!("response/{:x}", digest.finalize()))
    }

    fn path(&self, record: &str) -> PathBuf {
        self.root
            .join(format!("{:x}.enc", Sha256::digest(record.as_bytes())))
    }

    fn read(&self, record: &str) -> Result<Value> {
        let path = self.path(record);
        let meta = fs::symlink_metadata(&path)?;
        ensure!(
            meta.is_file() && meta.len() <= CAPACITY,
            "invalid response cache entry"
        );
        ensure!(
            meta.modified()?.elapsed()? < self.ttl,
            "expired response cache entry"
        );
        let bytes = http::limited(fs::File::open(path)?, CAPACITY)?;
        let plaintext = store::decrypt(&self.key, record, &bytes)?;
        serde_json::from_slice(&plaintext).context("invalid cached response")
    }

    fn write(&self, record: &str, value: &Value) -> Result<()> {
        let raw = Zeroizing::new(serde_json::to_vec(value)?);
        if raw.len() as u64 + 36 > CAPACITY {
            return Ok(());
        }
        let bytes = store::encrypt(&self.key, record, &raw)?;
        let lock_path = self.root.join(".lock");
        if let Ok(meta) = fs::symlink_metadata(&lock_path) {
            ensure!(meta.is_file(), "invalid response cache lock");
        }
        let mut options = fs::OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let lock = options.open(lock_path)?;
        fs2::FileExt::try_lock_exclusive(&lock)?;
        let target = self.path(record);
        let mut entries = fs::read_dir(&self.root)?
            .filter_map(Result::ok)
            .filter_map(|entry| {
                if entry.path() == target {
                    return None;
                }
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.len() != 68
                    || !name.ends_with(".enc")
                    || !name[..64].bytes().all(|byte| byte.is_ascii_hexdigit())
                {
                    return None;
                }
                let meta = fs::symlink_metadata(entry.path()).ok()?;
                meta.is_file().then(|| {
                    (
                        meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                        meta.len(),
                        entry.path(),
                    )
                })
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.0);
        let mut total = entries.iter().fold(bytes.len() as u64, |total, entry| {
            total.saturating_add(entry.1)
        });
        let mut count = entries.len() + 1;
        for (_, size, path) in entries {
            if count <= ENTRIES && total <= CAPACITY {
                break;
            }
            fs::remove_file(path)?;
            count -= 1;
            total = total.saturating_sub(size);
        }
        store::atomic(&target, &bytes)
    }

    pub fn fetch(&self, url: &str, fetch: impl FnOnce() -> Result<Value>) -> Result<Value> {
        if self.ttl.is_zero() {
            return fetch();
        }
        let Ok(record) = self.record(url) else {
            return fetch();
        };
        if let Ok(value) = self.read(&record) {
            return Ok(value);
        }
        let value = fetch()?;
        // Caching must not turn a successful Google request into an error.
        let _ = self.write(&record, &value);
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_post_cache_keys_include_the_body_without_exposing_its_contents() {
        let first = body_identity(
            "https://photoslibrary.googleapis.com/v1/mediaItems:search".into(),
            Some(&serde_json::json!({"albumId":"private-album-one"})),
        )
        .unwrap();
        let second = body_identity(
            "https://photoslibrary.googleapis.com/v1/mediaItems:search".into(),
            Some(&serde_json::json!({"albumId":"private-album-two"})),
        )
        .unwrap();
        assert_ne!(first, second);
        assert!(!first.contains("private-album"));
        assert_eq!(
            body_identity("https://example.googleapis.com/v1/read".into(), None).unwrap(),
            "https://example.googleapis.com/v1/read"
        );
    }
    use serde_json::json;
    use std::cell::Cell;

    const URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me/labels";
    fn cache(root: &Path, account: &str, ttl: u32) -> Responses {
        Responses::new(root, account, Zeroizing::new(vec![7; 32]), ttl).unwrap()
    }

    #[test]
    fn zero_ttl_always_fetches_and_creates_no_cache() {
        let root = tempfile::tempdir().unwrap();
        let cache = cache(root.path(), "me@example.com", 0);
        let calls = Cell::new(0);
        for _ in 0..2 {
            cache
                .fetch(URL, || {
                    calls.set(calls.get() + 1);
                    Ok(json!({"labels":[]}))
                })
                .unwrap();
        }
        assert_eq!(calls.get(), 2);
        assert!(!root.path().join("responses").exists());
    }

    #[test]
    fn cache_reuses_only_fresh_entries_and_keeps_response_data_encrypted() {
        let root = tempfile::tempdir().unwrap();
        let cache = cache(root.path(), "private-account@example.com", 60);
        let value = json!({"labels":[{"name":"private-response"}]});
        assert_eq!(cache.fetch(URL, || Ok(value.clone())).unwrap(), value);
        assert_eq!(
            cache
                .fetch(URL, || panic!("fresh data should be cached"))
                .unwrap(),
            value
        );
        let file = cache.path(&cache.record(URL).unwrap());
        let bytes = fs::read(&file).unwrap();
        for secret in ["private-response", "private-account@example.com", URL] {
            assert!(
                !bytes
                    .windows(secret.len())
                    .any(|window| window == secret.as_bytes())
            );
        }
        let modified = std::time::SystemTime::now() - Duration::from_secs(3601);
        fs::OpenOptions::new()
            .write(true)
            .open(file)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(modified))
            .unwrap();
        assert_eq!(
            cache.fetch(URL, || Ok(json!({"fresh":true}))).unwrap(),
            json!({"fresh":true})
        );
    }

    #[test]
    fn account_url_and_generation_isolate_cached_responses() {
        let root = tempfile::tempdir().unwrap();
        let first = cache(root.path(), "first@example.com", 60);
        let second = cache(root.path(), "second@example.com", 60);
        first.fetch(URL, || Ok(json!({"account":"first"}))).unwrap();
        assert_eq!(
            second
                .fetch(URL, || Ok(json!({"account":"second"})))
                .unwrap(),
            json!({"account":"second"})
        );
        let other = format!("{URL}?fields=labels(id)");
        assert_eq!(
            first
                .fetch(&other, || Ok(json!({"different":true})))
                .unwrap(),
            json!({"different":true})
        );
        invalidate(root.path(), "first@example.com").unwrap();
        assert_eq!(
            first.fetch(URL, || Ok(json!({"updated":true}))).unwrap(),
            json!({"updated":true})
        );
        assert_eq!(
            second
                .fetch(URL, || panic!("other account must keep its own cache"))
                .unwrap(),
            json!({"account":"second"})
        );
    }

    #[test]
    fn invalidation_during_a_request_prevents_reusing_its_old_result() {
        let root = tempfile::tempdir().unwrap();
        let cache = cache(root.path(), "me@example.com", 60);
        cache
            .fetch(URL, || {
                invalidate(root.path(), "me@example.com")?;
                Ok(json!({"stale":true}))
            })
            .unwrap();
        assert_eq!(
            cache.fetch(URL, || Ok(json!({"fresh":true}))).unwrap(),
            json!({"fresh":true})
        );
    }

    #[test]
    fn corrupt_wrong_key_or_swapped_entries_refetch_instead_of_returning_data() {
        let root = tempfile::tempdir().unwrap();
        let cache = cache(root.path(), "me@example.com", 60);
        cache
            .fetch(URL, || Ok(json!({"private":"original"})))
            .unwrap();
        let file = cache.path(&cache.record(URL).unwrap());
        let mut ciphertext = fs::read(&file).unwrap();
        ciphertext[30] ^= 1;
        fs::write(&file, ciphertext).unwrap();
        assert_eq!(
            cache.fetch(URL, || Ok(json!({"refetched":true}))).unwrap(),
            json!({"refetched":true})
        );
        let wrong_key = Responses::new(
            root.path(),
            "me@example.com",
            Zeroizing::new(vec![8; 32]),
            60,
        )
        .unwrap();
        assert_eq!(
            wrong_key
                .fetch(URL, || Ok(json!({"wrong_key_refetched":true})))
                .unwrap(),
            json!({"wrong_key_refetched":true})
        );
        let other = format!("{URL}?fields=labels(id)");
        fs::copy(&file, cache.path(&cache.record(&other).unwrap())).unwrap();
        assert_eq!(
            cache
                .fetch(&other, || Ok(json!({"swapped_refetched":true})))
                .unwrap(),
            json!({"swapped_refetched":true})
        );
    }

    #[test]
    fn failures_are_not_cached_and_cache_storage_is_bounded() {
        let root = tempfile::tempdir().unwrap();
        let cache = cache(root.path(), "me@example.com", 60);
        assert!(
            cache
                .fetch(URL, || anyhow::bail!("Google request failed"))
                .is_err()
        );
        for index in 0..40 {
            cache
                .fetch(&format!("{URL}?page={index}"), || Ok(json!({"id":index})))
                .unwrap();
        }
        let entries = fs::read_dir(&cache.root)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "enc")
            })
            .count();
        assert_eq!(entries, ENTRIES);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for entry in fs::read_dir(&cache.root).unwrap() {
                assert_eq!(
                    entry.unwrap().metadata().unwrap().permissions().mode() & 0o777,
                    0o600
                );
            }
            assert_eq!(
                fs::metadata(&cache.root).unwrap().permissions().mode() & 0o777,
                0o700
            );
        }
    }
}
