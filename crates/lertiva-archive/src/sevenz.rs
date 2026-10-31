use std::path::{Path, PathBuf};

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

pub fn extract_one(
    archive: &Path,
    entry_name: &str,
    dest_file: &Path,
    password: Option<&str>,
) -> Result<()> {
    let want = entry_name.replace('\\', "/");
    let names = list(archive, password)?;
    if !names.iter().any(|n| n.replace('\\', "/") == want) {
        return Err(ArchiveError::SevenZ(format!("dosya bulunamadı: {want}")));
    }
    let temp = tempfile::tempdir()?;
    extract(archive, temp.path(), password)?;
    let found = find_entry_in_dir(temp.path(), &want)?;
    if let Some(parent) = dest_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(&found, dest_file).map_err(|e| ArchiveError::SevenZ(e.to_string()))?;
    Ok(())
}

fn find_entry_in_dir(root: &Path, want: &str) -> Result<PathBuf> {
    let direct = root.join(want);
    if direct.is_file() {
        return Ok(direct);
    }
    let leaf = want.rsplit('/').next().unwrap_or(want);
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let p = entry.path();
        let rel = p
            .strip_prefix(root)
            .unwrap_or(p)
            .to_string_lossy()
            .replace('\\', "/");
        if rel == want || rel.ends_with(want) || p.file_name().and_then(|s| s.to_str()) == Some(leaf) {
            return Ok(p.to_path_buf());
        }
    }
    Err(ArchiveError::SevenZ(format!("çıkarılan dosya bulunamadı: {want}")))
}

const SEVENZ_PREVIEW_MAX: usize = 65_536;

pub fn preview_entry_bytes(
    archive: &Path,
    entry_name: &str,
    password: Option<&str>,
    max_bytes: usize,
) -> Result<(Vec<u8>, u64, bool)> {
    let cap = max_bytes.min(SEVENZ_PREVIEW_MAX);
    let temp = tempfile::tempdir()?;
    let preview_path = temp.path().join("preview.bin");
    extract_one(archive, entry_name, &preview_path, password)?;
    let size = std::fs::metadata(&preview_path)?.len();
    let mut file = std::fs::File::open(&preview_path)?;
    let mut buf = vec![0u8; cap.min(size as usize)];
    use std::io::Read;
    let read = file.read(&mut buf)?;
    buf.truncate(read);
    Ok((buf, size, size > cap as u64))
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
