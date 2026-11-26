use aes_gcm::{
    Aes256Gcm, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Output},
};

fn private_file(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
    }
}

// A synthetic laptop vault. The test never reads the user's vault or contacts Google.
fn laptop(home: &Path) {
    let root = home.join("joog");
    fs::create_dir_all(root.join("secrets")).unwrap();
    let key = [0x5a; 32];
    private_file(&root.join("master.key"), &key);
    private_file(
        &root.join("vault.json"),
        br#"{"version":1,"service":"joog/vault","key_storage":"file"}"#,
    );
    private_file(
        &root.join("config.json"),
        br#"{"account":"test@example.com","accounts":["test@example.com"]}"#,
    );
    for (name, value) in [
        (
            "oauth-client",
            json!({"client_id":"synthetic-client-id","client_secret":"synthetic-client-secret"}),
        ),
        (
            "account/test@example.com",
            json!({"access_token":"synthetic-access-token","refresh_token":"synthetic-refresh-token","expires_at":1}),
        ),
    ] {
        let mut nonce = [0; 12];
        getrandom::fill(&mut nonce).unwrap();
        let encrypted = Aes256Gcm::new_from_slice(&key)
            .unwrap()
            .encrypt(
                &Nonce::from(nonce),
                Payload {
                    msg: &serde_json::to_vec(&value).unwrap(),
                    aad: &[b"GOGVLT01".as_slice(), name.as_bytes()].concat(),
                },
            )
            .unwrap();
        private_file(
            &root
                .join("secrets")
                .join(format!("{:x}.enc", Sha256::digest(name.as_bytes()))),
            &[b"GOGVLT01".as_slice(), &nonce, &encrypted].concat(),
        );
    }
}

fn run(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_joog"))
        .args(["--home", home.to_str().unwrap(), "--json", "auth"])
        .args(args)
        .env(
            "DBUS_SESSION_BUS_ADDRESS",
            "unix:path=/nonexistent-joog-test-session",
        )
        .output()
        .unwrap()
}

#[test]
fn laptop_export_and_headless_server_import_work_through_the_executable() {
    let laptop_dir = tempfile::tempdir().unwrap();
    let server_dir = tempfile::tempdir().unwrap();
    laptop(laptop_dir.path());
    let mut passphrase = tempfile::NamedTempFile::new().unwrap();
    passphrase
        .write_all(b"synthetic transfer passphrase\n")
        .unwrap();
    let password = passphrase.path().to_str().unwrap();
    let export = laptop_dir.path().join("credentials.joog");
    let result = run(
        laptop_dir.path(),
        &[
            "test@example.com",
            "export",
            export.to_str().unwrap(),
            "--passphrase-file",
            password,
        ],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let status: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(status["accounts"], json!(["test@example.com"]));
    assert!(!String::from_utf8_lossy(&result.stdout).contains("synthetic-"));

    let copied = server_dir.path().join("credentials.joog");
    fs::copy(&export, &copied).unwrap();
    let result = run(
        server_dir.path(),
        &[
            "import",
            copied.to_str().unwrap(),
            "--headless",
            "--passphrase-file",
            password,
        ],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&result.stdout).unwrap()["accounts"],
        json!(["test@example.com"])
    );
    assert!(server_dir.path().join("joog/master.key").is_file());
    assert_ne!(
        fs::read(laptop_dir.path().join("joog/master.key")).unwrap(),
        fs::read(server_dir.path().join("joog/master.key")).unwrap()
    );

    let reexport = server_dir.path().join("server-copy.joog");
    let result = run(
        server_dir.path(),
        &[
            "export",
            reexport.to_str().unwrap(),
            "--passphrase-file",
            password,
        ],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(reexport.is_file());
    let collision = run(
        server_dir.path(),
        &[
            "import",
            copied.to_str().unwrap(),
            "--headless",
            "--passphrase-file",
            password,
        ],
    );
    assert!(!collision.status.success());
    assert!(String::from_utf8_lossy(&collision.stderr).contains("--replace"));
    let replacement = run(
        server_dir.path(),
        &[
            "import",
            copied.to_str().unwrap(),
            "--replace",
            "--passphrase-file",
            password,
        ],
    );
    assert!(
        replacement.status.success(),
        "{}",
        String::from_utf8_lossy(&replacement.stderr)
    );
}

#[test]
fn headless_init_needs_no_keychain_and_preserves_its_key_on_reinitialization() {
    let directory = tempfile::tempdir().unwrap();
    let first = run(directory.path(), &["init", "--headless"]);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&first.stdout).unwrap()["master_key"],
        "file"
    );
    let key = fs::read(directory.path().join("joog/master.key")).unwrap();
    let second = run(directory.path(), &["init"]);
    assert!(second.status.success());
    assert_eq!(
        fs::read(directory.path().join("joog/master.key")).unwrap(),
        key
    );
}
