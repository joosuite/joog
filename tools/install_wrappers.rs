//! Install wrapper entry points beside a built or installed Joog executable.
use anyhow::{Context, Result, ensure};
use std::path::Path;

fn install(directory: &Path) -> Result<usize> {
    let executable = directory.join(format!("joog{}", std::env::consts::EXE_SUFFIX));
    ensure!(
        executable.is_file(),
        "{}: build or install joog first",
        executable.display()
    );
    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/wrappers.schema.json"))?;
    let names = schema["x-wrappers"]
        .as_object()
        .context("missing wrapper definitions")?;
    for name in names.keys() {
        let target = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
        #[cfg(unix)]
        {
            match std::fs::symlink_metadata(&target) {
                Ok(metadata)
                    if metadata.file_type().is_symlink()
                        && std::fs::read_link(&target)? == Path::new("joog") =>
                {
                    continue;
                }
                Ok(_) => anyhow::bail!(
                    "{}: an unrelated file or symbolic link already exists",
                    target.display()
                ),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
            std::os::unix::fs::symlink("joog", &target)?;
        }
        #[cfg(windows)]
        std::fs::copy(&executable, &target)?;
    }
    Ok(names.len())
}

fn main() -> Result<()> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    ensure!(
        args.len() == 2 && args[0] == "--directory",
        "usage: install_wrappers --directory PATH"
    );
    let directory = Path::new(&args[1]);
    let count = install(directory)?;
    use std::io::Write;
    writeln!(
        std::io::stdout().lock(),
        "Installed {count} wrappers in {}",
        directory.display()
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_all_schema_defined_wrappers_and_can_run_again() {
        let directory = tempfile::tempdir().unwrap();
        let binary = directory
            .path()
            .join(format!("joog{}", std::env::consts::EXE_SUFFIX));
        std::fs::write(binary, b"native executable").unwrap();
        let schema: serde_json::Value =
            serde_json::from_str(include_str!("../schemas/wrappers.schema.json")).unwrap();
        let wrappers = schema["x-wrappers"].as_object().unwrap();
        assert_eq!(install(directory.path()).unwrap(), wrappers.len());
        assert_eq!(install(directory.path()).unwrap(), wrappers.len());
        for name in wrappers.keys() {
            let path = directory
                .path()
                .join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
            assert_eq!(std::fs::read(&path).unwrap(), b"native executable");
            #[cfg(unix)]
            assert_eq!(std::fs::read_link(path).unwrap(), Path::new("joog"));
        }
    }

    #[test]
    fn requires_joog_before_creating_wrappers() {
        let directory = tempfile::tempdir().unwrap();
        assert!(install(directory.path()).is_err());
        assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn preserves_unrelated_files_and_symbolic_links() {
        for symlink in [false, true] {
            let directory = tempfile::tempdir().unwrap();
            std::fs::write(directory.path().join("joog"), b"native executable").unwrap();
            let target = directory.path().join("jmail");
            if symlink {
                std::os::unix::fs::symlink("other-program", &target).unwrap();
            } else {
                std::fs::write(&target, b"existing program").unwrap();
            }
            assert!(install(directory.path()).is_err());
            if symlink {
                assert_eq!(
                    std::fs::read_link(target).unwrap(),
                    Path::new("other-program")
                );
            } else {
                assert_eq!(std::fs::read(target).unwrap(), b"existing program");
            }
        }
    }
}
