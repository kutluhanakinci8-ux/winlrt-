use std::path::Path;
use std::process::Command;

use rar::Archive;

use crate::{ArchiveError, Result};

const LIST_FALLBACK_MAX_BYTES: u64 = 50 * 1024 * 1024;

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| ArchiveError::Rar("geçersiz dosya yolu".into()))
}

fn map_rar(err: rar::error::RarError) -> ArchiveError {
    ArchiveError::Rar(err.to_string())
}

/// Sistem `unrar` / `UnRAR` ile dosya listesi (içerik çıkarmadan).
fn try_unrar_list(archive: &Path) -> Option<Result<Vec<String>>> {
    for cmd in ["unrar", "UnRAR"] {
        let output = Command::new(cmd)
            .args(["lb", "-p-", path_str(archive).ok()?])
            .output()
            .ok()?;
        if !output.status.success() {
            continue;
        }
        let text = String::from_utf8_lossy(&output.stdout);
        let names: Vec<String> = text
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect();
        return Some(Ok(names));
    }
    None
}

fn list_via_rar_crate(archive: &Path, password: &str) -> Result<Vec<String>> {
    let len = std::fs::metadata(archive)?.len();
    if len > LIST_FALLBACK_MAX_BYTES {
        return Err(ArchiveError::Rar(
            "Büyük RAR: listelemek için sistemde unrar/UnRAR kurun.".into(),
        ));
    }
    let temp = tempfile::tempdir()?;
    let arch = Archive::extract_all(path_str(archive)?, path_str(temp.path())?, password)
        .map_err(map_rar)?;
    Ok(arch.files.into_iter().map(|f| f.name).collect())
}

/// RAR / RAR5 okuma — RAR5 için `rar` crate (MIT); RAR4 sınırlı.
pub fn list(archive: &Path, password: Option<&str>) -> Result<Vec<String>> {
    if let Some(res) = try_unrar_list(archive) {
        return res;
    }
    let pw = password.unwrap_or("");
    list_via_rar_crate(archive, pw)
}

pub fn extract(archive: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
    std::fs::create_dir_all(dest)?;
    let pw = password.unwrap_or("");
    Archive::extract_all(path_str(archive)?, path_str(dest)?, pw).map_err(map_rar)?;
    Ok(())
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
            .join("../../benchmarks/fixtures/rar/rar5-text.rar")
    }

    #[test]
    fn rar5_fixture_extract_list() {
        let archive = fixture();
        if !archive.is_file() {
            return;
        }
        let names = list(&archive, Some("test")).expect("list");
        assert!(names.iter().any(|n| n.contains("text.txt")));
        let dir = tempfile::tempdir().unwrap();
        extract(&archive, dir.path(), Some("test")).unwrap();
        assert!(dir.path().join("text.txt").is_file());
    }
}
