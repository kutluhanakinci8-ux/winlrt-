use std::fs::File;
use std::io::{copy, Read, Write};
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

fn add_file(
    writer: &mut ZipWriter<File>,
    path: &Path,
    name: &str,
    profile: CompressionProfile,
) -> Result<()> {
    let method = match profile {
        CompressionProfile::Fast => CompressionMethod::Stored,
        _ => CompressionMethod::Deflated,
    };
    let options = FileOptions::default().compression_method(method);
    writer
        .start_file(name.replace('\\', "/"), options)
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    writer
        .write_all(&buf)
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
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

pub fn test(archive: &Path) -> Result<()> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| ArchiveError::Zip(e.to_string()))?;
        std::io::copy(&mut entry, &mut std::io::sink())?;
    }
    Ok(())
}
