use std::fs::File;
use std::io::{copy, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{AesMode, CompressionMethod, ZipArchive, ZipWriter};

use crate::crypto;
use crate::profile::CompressionProfile;
use crate::{ArchiveError, Result};

fn password_bytes(password: Option<&str>) -> Option<&[u8]> {
    password.filter(|p| !p.is_empty()).map(|p| p.as_bytes())
}

fn open_zip_entry<'a>(
    zip: &'a mut ZipArchive<File>,
    index: usize,
    password: Option<&[u8]>,
) -> Result<zip::read::ZipFile<'a, File>> {
    match password {
        Some(p) => zip.by_index_decrypt(index, p).map_err(map_zip_password_err),
        None => zip
            .by_index(index)
            .map_err(|e| ArchiveError::Zip(e.to_string())),
    }
}

fn map_zip_password_err(e: zip::result::ZipError) -> ArchiveError {
    let msg = e.to_string().to_lowercase();
    if msg.contains("password") || msg.contains("decrypt") {
        ArchiveError::Zip("ZIP parola hatalı".into())
    } else {
        ArchiveError::Zip(e.to_string())
    }
}

pub fn list(archive: &Path, password: Option<&str>) -> Result<Vec<String>> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let pw = password_bytes(password);
    let mut names = Vec::with_capacity(zip.len());
    for i in 0..zip.len() {
        let entry = open_zip_entry(&mut zip, i, pw)?;
        names.push(entry.name().to_string());
    }
    Ok(names)
}

pub fn extract(archive: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let pw = password_bytes(password);
    for i in 0..zip.len() {
        let mut entry = open_zip_entry(&mut zip, i, pw)?;
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
    create_with_profile(source, dest, CompressionProfile::Balanced, None)
}

pub fn create_with_profile(
    source: &Path,
    dest: &Path,
    profile: CompressionProfile,
    password: Option<&str>,
) -> Result<()> {
    if !source.exists() {
        return Err(ArchiveError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            source.display().to_string(),
        )));
    }
    if password.is_some_and(|p| !p.is_empty()) {
        let fb = crypto::evaluate_password(password.unwrap(), true);
        if !fb.ok_for_create {
            return Err(ArchiveError::Zip("parola politikası karşılanmıyor".into()));
        }
        if crate::enterprise::modern_crypto_gate_active() {
            // Yalnızca AES-256 (WinZip AE-2) yolu kullanılır; ZipCrypto desteklenmez.
        }
    }
    let out = File::create(dest)?;
    let mut writer = ZipWriter::new(out);
    let source = source.canonicalize()?;
    if source.is_file() {
        let name = zip_entry_name_from_path(source.file_name().unwrap().as_ref())?;
        add_file(&mut writer, &source, &name, profile, password)?;
    } else {
        for entry in WalkDir::new(&source).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let rel = path.strip_prefix(&source).unwrap_or(path);
                let name = zip_entry_name_from_path(rel)?;
                add_file(&mut writer, path, &name, profile, password)?;
            }
        }
    }
    writer
        .finish()
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    Ok(())
}

pub fn create_from_paths(
    sources: &[PathBuf],
    dest: &Path,
    profile: CompressionProfile,
    password: Option<&str>,
) -> Result<()> {
    if sources.is_empty() {
        return Err(ArchiveError::Zip("sıkıştırılacak öğe yok".into()));
    }
    if password.is_some_and(|p| !p.is_empty()) {
        let fb = crypto::evaluate_password(password.unwrap(), true);
        if !fb.ok_for_create {
            return Err(ArchiveError::Zip("parola çok kısa (min. 8)".into()));
        }
    }
    let files = crate::wlrt::selection_file_list(sources)?;
    let out = File::create(dest)?;
    let mut writer = ZipWriter::new(out);
    for (path, rel) in &files {
        add_file(&mut writer, path, rel, profile, password)?;
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
    password: Option<&str>,
) -> Result<()> {
    let file_len = std::fs::metadata(path)?.len();
    let method = match profile {
        CompressionProfile::Fast => CompressionMethod::Stored,
        _ => CompressionMethod::Deflated,
    };
    let mut options = SimpleFileOptions::default()
        .compression_method(method)
        .large_file(file_len >= ZIP64_SIZE_THRESHOLD);
    if let Some(pw) = password_bytes(password) {
        let text = std::str::from_utf8(pw).map_err(|_| {
            ArchiveError::Zip("ZIP AES parolası UTF-8 olmalı".into())
        })?;
        options = options.with_aes_encryption(AesMode::Aes256, text);
    }
    writer
        .start_file(name.replace('\\', "/"), options)
        .map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let mut f = File::open(path)?;
    copy(&mut f, writer).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    Ok(())
}

/// ZIP giriş adı — UTF-8 (I5 / Türkçe dosya adları).
pub fn zip_entry_name_from_path(rel: &Path) -> Result<String> {
    let s = rel
        .to_str()
        .ok_or_else(|| ArchiveError::Zip("dosya yolu UTF-8 olmalı (I5)".into()))?;
    Ok(s.replace('\\', "/"))
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
    let options = SimpleFileOptions::default()
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

pub fn test(archive: &Path, password: Option<&str>) -> Result<()> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let pw = password_bytes(password);
    for i in 0..zip.len() {
        let mut entry = open_zip_entry(&mut zip, i, pw)?;
        std::io::copy(&mut entry, &mut std::io::sink())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn zip_turkish_utf8_entry_names() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        let nested = src.join("Şirket").join("iğüş");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join("Özet_2026.txt"), b"turkce").unwrap();
        let archive = dir.path().join("utf8.zip");
        create_with_profile(&src, &archive, CompressionProfile::Balanced, None).unwrap();
        let entries = list(&archive, None).unwrap();
        assert!(entries.iter().any(|e| e.contains("Özet_2026.txt")));
        assert!(entries.iter().any(|e| e.contains("Şirket")));
        let out = dir.path().join("out");
        extract(&archive, &out, None).unwrap();
        assert_eq!(
            std::fs::read(out.join("Şirket/iğüş/Özet_2026.txt")).unwrap(),
            b"turkce"
        );
    }

    #[test]
    fn zip_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("a.txt"), b"zip test").unwrap();
        let archive = dir.path().join("t.zip");
        create_with_profile(&src, &archive, CompressionProfile::Balanced, None).unwrap();
        test(&archive, None).unwrap();
        let out = dir.path().join("out");
        extract(&archive, &out, None).unwrap();
        assert_eq!(std::fs::read(out.join("a.txt")).unwrap(), b"zip test");
    }

    #[test]
    fn zip_aes_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("secret.txt"), b"zip aes").unwrap();
        let archive = dir.path().join("enc.zip");
        create_with_profile(
            &src,
            &archive,
            CompressionProfile::Balanced,
            Some("test-password-12"),
        )
        .unwrap();
        test(&archive, Some("test-password-12")).unwrap();
        let out = dir.path().join("out");
        extract(&archive, &out, Some("test-password-12")).unwrap();
        assert_eq!(
            std::fs::read(out.join("secret.txt")).unwrap(),
            b"zip aes"
        );
    }

    #[test]
    fn zip64_smoke_roundtrip() {
        let dir = tempdir().unwrap();
        let archive = dir.path().join("z64.zip");
        create_zip64_smoke(&archive).unwrap();
        test(&archive, None).unwrap();
        let out = dir.path().join("out");
        extract(&archive, &out, None).unwrap();
        assert_eq!(
            std::fs::read(out.join("zip64-smoke.txt")).unwrap(),
            b"zip64"
        );
    }
}
