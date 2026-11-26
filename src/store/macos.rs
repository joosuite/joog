//! Keychain access through Apple's stable, signed tool rather than the rebuilt CLI.
use super::{valid_service, validate_key};
use anyhow::{Context, Result, ensure};
use std::{
    io::Write,
    process::{Command, Stdio},
};
use zeroize::{Zeroize, Zeroizing};

fn valid_lookup(service: &str) -> bool {
    valid_service(service)
        || service
            .strip_prefix("gogcli-rust/vault/")
            .is_some_and(|id| id.len() == 64 && id.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

fn decode(bytes: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
    let raw = bytes
        .strip_suffix(b"\n")
        .context("invalid Keychain response")?;
    let key = if raw.len() == 64 && raw.iter().all(u8::is_ascii_hexdigit) {
        let mut key = Zeroizing::new(Vec::with_capacity(32));
        for pair in raw.as_chunks::<2>().0 {
            let text = std::str::from_utf8(pair).context("invalid Keychain response")?;
            key.push(u8::from_str_radix(text, 16).context("invalid Keychain response")?);
        }
        key
    } else {
        Zeroizing::new(raw.to_vec())
    };
    validate_key(&key)?;
    Ok(key)
}

pub(super) fn load(service: &str) -> Result<Option<Zeroizing<Vec<u8>>>> {
    ensure!(valid_lookup(service), "invalid Keychain service");
    let mut output = Command::new("/usr/bin/security")
        .args([
            "find-generic-password",
            "-s",
            service,
            "-a",
            "master-key",
            "-w",
        ])
        .stdin(Stdio::null())
        .output()
        .context("cannot run macOS Keychain access")?;
    let bytes = Zeroizing::new(std::mem::take(&mut output.stdout));
    output.stderr.zeroize();
    if output.status.code() == Some(44) {
        return Ok(None);
    }
    ensure!(
        output.status.success(),
        "cannot unlock master key in macOS Keychain; approve access for Apple's security tool or unlock your login Keychain"
    );
    decode(&bytes).map(Some)
}

fn save_input(service: &str, key: &[u8]) -> Result<Zeroizing<String>> {
    use std::fmt::Write;
    ensure!(valid_service(service), "invalid Keychain service");
    validate_key(key)?;
    let mut input = Zeroizing::new(format!(
        "add-generic-password -s {service} -a master-key -l \"Joog vault master key\" -T /usr/bin/security -X "
    ));
    for byte in key {
        write!(&mut *input, "{byte:02x}")?;
    }
    input.push('\n');
    Ok(input)
}

pub(super) fn save(service: &str, key: &[u8]) -> Result<()> {
    // Interactive input keeps the encryption key out of argv, the environment,
    // shell history and temporary files. No shell or verbose mode is used.
    let input = save_input(service, key)?;
    let mut child = Command::new("/usr/bin/security")
        .args(["-q", "-i"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("cannot run macOS Keychain access")?;
    let written = child
        .stdin
        .take()
        .context("cannot open private Keychain input")?
        .write_all(input.as_bytes());
    let mut output = child
        .wait_with_output()
        .context("cannot finish macOS Keychain access")?;
    output.stdout.zeroize();
    output.stderr.zeroize();
    written.context("cannot send private Keychain input")?;
    ensure!(
        output.status.success(),
        "cannot save master key in macOS Keychain; the existing key was not replaced"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_and_printable_legacy_keys_are_decoded_without_trimming() {
        let printable = b" abcdef0123456789abcdef01234567 ";
        assert_eq!(printable.len(), 32);
        let mut raw = printable.to_vec();
        raw.push(b'\n');
        assert_eq!(decode(&raw).unwrap().as_slice(), printable);
        let bytes = [0_u8, 10, 13, 255].repeat(8);
        let hex = format!("{}\n", crate::store::hex(&bytes));
        assert_eq!(decode(hex.as_bytes()).unwrap().as_slice(), bytes);
        for invalid in [b"".as_slice(), b"short\n", &[b'g'; 65]] {
            assert!(decode(invalid).is_err());
        }
    }

    #[test]
    fn writes_use_private_input_and_trust_only_the_system_tool() {
        let input = save_input("joog/vault", &[7; 32]).unwrap();
        assert!(input.contains("-T /usr/bin/security"));
        assert!(!input.contains("-A"));
        assert!(!input.contains("-U"));
        assert!(input.ends_with(&format!("{}\n", "07".repeat(32))));
        for service in ["joog/vault\nquit", "joog/vault/../other", "-a", "other"] {
            assert!(save_input(service, &[0; 32]).is_err());
        }
        assert!(save_input("joog/vault", &[0; 31]).is_err());
    }
}
