use std::fs::File;
use std::io::{copy, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::profile::CompressionProfile;
use crate::{ArchiveError, Result};

pub fn list(archive: &Path) -> Result<Vec<String>> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let mut names = Vec::with_capacity(zip.len());
    for i in 0..zip.len() {
        let entry = zip.by_index(i).map_err(|e| ArchiveError::Zip(e.to_string()))?;
        names.push(entry.name().to_string());
    }
    Ok(names)
}

pub fn extract(archive: &Path, dest: &Path) -> Result<()> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| ArchiveError::Zip(e.to_string()))?;
        let name = entry.name().to_string();
        let out_path = dest.join(sanitize_path(&name));
        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out = File::create(&out_path)?;
            copy(&mut entry, &mut out)?;
        }
    }
    Ok(())
}

pub fn create(source: &Path, dest: &Path) -> Result<()> {
    create_with_profile(source, dest, CompressionProfile::Balanced)
}

pub fn create_with_profile(source: &Path, dest: &Path, profile: CompressionProfile) -> Result<()> {
    if !source.exists() {
        return Err(ArchiveError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            source.display().to_string(),
        )));
    }
    let out = File::create(dest)?;
    let mut writer = ZipWriter::new(out);
    let source = source.canonicalize()?;
    if source.is_file() {
        add_file(
            &mut writer,
            &source,
            &source.file_name().unwrap().to_string_lossy(),
            profile,
        )?;
    } else {
        for entry in WalkDir::new(&source).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let rel = path.strip_prefix(&source).unwrap_or(path);
                add_file(&mut writer, path, &rel.to_string_lossy(), profile)?;
            }
        }
    }
    writer
        .finish()
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    Ok(())
}

/// Zip64 eşiği (zip crate `ZIP64_BYTES_THR` ile uyumlu).
const ZIP64_SIZE_THRESHOLD: u64 = u32::MAX as u64;

fn add_file(
    writer: &mut ZipWriter<File>,
    path: &Path,
    name: &str,
    profile: CompressionProfile,
) -> Result<()> {
    let file_len = std::fs::metadata(path)?.len();
    let method = match profile {
        CompressionProfile::Fast => CompressionMethod::Stored,
        _ => CompressionMethod::Deflated,
    };
    let options = FileOptions::default()
        .compression_method(method)
        .large_file(file_len >= ZIP64_SIZE_THRESHOLD);
    writer
        .start_file(name.replace('\\', "/"), options)
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let mut f = File::open(path)?;
    copy(&mut f, writer).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    Ok(())
}

fn sanitize_path(name: &str) -> PathBuf {
    PathBuf::from(
        name.split('/')
            .filter(|p| !p.is_empty() && *p != "..")
            .collect::<Vec<_>>()
            .join("/"),
    )
}

/// Zip64 alanları yazılmış küçük arşiv (Faz 1 hazırlık testi).
pub fn create_zip64_smoke(dest: &Path) -> Result<()> {
    let out = File::create(dest)?;
    let mut writer = ZipWriter::new(out);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .large_file(true);
    writer
        .start_file("zip64-smoke.txt", options)
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    writer
        .write_all(b"zip64")
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    writer
        .finish()
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    Ok(())
}

pub fn test(archive: &Path) -> Result<()> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| ArchiveError::Zip(e.to_string()))?;
        std::io::copy(&mut entry, &mut std::io::sink())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn zip_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("a.txt"), b"zip test").unwrap();
        let archive = dir.path().join("t.zip");
        create_with_profile(&src, &archive, CompressionProfile::Balanced).unwrap();
        test(&archive).unwrap();
        let out = dir.path().join("out");
        extract(&archive, &out).unwrap();
        assert_eq!(std::fs::read(out.join("a.txt")).unwrap(), b"zip test");
    }

    #[test]
    fn zip64_smoke_roundtrip() {
        let dir = tempdir().unwrap();
        let archive = dir.path().join("z64.zip");
        create_zip64_smoke(&archive).unwrap();
        test(&archive).unwrap();
        let out = dir.path().join("out");
        extract(&archive, &out).unwrap();
        assert_eq!(
            std::fs::read(out.join("zip64-smoke.txt")).unwrap(),
            b"zip64"
        );
    }
}
