use std::path::Path;

use sevenz_rust::{decompress_file, Password, SevenZReader};

use crate::{ArchiveError, Result};

fn password_from(opt: Option<&str>) -> Password {
    match opt {
        Some(p) if !p.is_empty() => Password::from(p),
        _ => Password::empty(),
    }
}

fn map_err(e: sevenz_rust::Error) -> ArchiveError {
    ArchiveError::SevenZ(e.to_string())
}

/// 7z okuma — `sevenz-rust` (Apache-2.0), Faz 2.
pub fn list(archive: &Path, password: Option<&str>) -> Result<Vec<String>> {
    let reader = SevenZReader::open(archive, password_from(password)).map_err(map_err)?;
    Ok(reader
        .archive()
        .files
        .iter()
        .map(|e| e.name.clone())
        .collect())
}

pub fn extract(archive: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
    if password.is_some_and(|p| !p.is_empty()) {
        return try_7z_cli_extract(archive, dest, password.unwrap());
    }
    std::fs::create_dir_all(dest)?;
    decompress_file(archive, dest).map_err(map_err)?;
    Ok(())
}

fn try_7z_cli_extract(archive: &Path, dest: &Path, password: &str) -> Result<()> {
    use std::process::Command;
    std::fs::create_dir_all(dest)?;
    for cmd in ["7z", "7za"] {
        let status = Command::new(cmd)
            .args([
                "x",
                "-y",
                &format!("-p{password}"),
                archive.to_str().unwrap_or(""),
                &format!("-o{}", dest.display()),
            ])
            .status();
        if let Ok(s) = status {
            if s.success() {
                return Ok(());
            }
        }
    }
    Err(ArchiveError::SevenZ(
        "Parolalı 7z için sistemde 7z/7za gerekli (Faz 2)".into(),
    ))
}

pub fn test(archive: &Path, password: Option<&str>) -> Result<()> {
    let temp = tempfile::tempdir()?;
    extract(archive, temp.path(), password)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../benchmarks/fixtures/sevenz/sample.7z")
    }

    #[test]
    fn sevenz_fixture_roundtrip() {
        let archive = fixture();
        if !archive.is_file() {
            return;
        }
        let names = list(&archive, None).expect("list");
        assert!(!names.is_empty());
        let dir = tempfile::tempdir().unwrap();
        extract(&archive, dir.path(), None).unwrap();
        test(&archive, None).unwrap();
    }
}
