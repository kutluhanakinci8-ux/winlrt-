use std::fs::File;
use std::io::{copy, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{AesMode, CompressionMethod, ZipArchive, ZipWriter};

use crate::crypto;
use crate::profile::CompressionProfile;
use crate::{ArchiveError, Result};

/// I7 — bozuk CRC / bütünlük hatası kullanıcı mesajı öneki.
pub const ZIP_INTEGRITY_CRC_MSG: &str = "ZIP bütünlük hatası (CRC)";

fn password_bytes(password: Option<&str>) -> Option<&[u8]> {
    password.filter(|p| !p.is_empty()).map(|p| p.as_bytes())
}

fn map_zip_err(e: zip::result::ZipError) -> ArchiveError {
    let msg = e.to_string().to_lowercase();
    if msg.contains("password") || msg.contains("decrypt") {
        return ArchiveError::Zip("ZIP parola hatalı".into());
    }
    if integrity_read_failure(&msg) {
        return ArchiveError::Zip(format!(
            "{ZIP_INTEGRITY_CRC_MSG} — arşiv bozuk veya eksik"
        ));
    }
    ArchiveError::Zip(e.to_string())
}

fn integrity_read_failure(msg: &str) -> bool {
    let m = msg.to_lowercase();
    m.contains("crc")
        || m.contains("checksum")
        || m.contains("corrupt")
        || m.contains("invalid stream")
        || m.contains("incomplete deflate")
        || (m.contains("deflate") && (m.contains("corrupt") || m.contains("invalid") || m.contains("incomplete")))
}

fn map_read_err(e: std::io::Error) -> ArchiveError {
    let msg = e.to_string();
    if integrity_read_failure(&msg) {
        ArchiveError::Zip(format!(
            "{ZIP_INTEGRITY_CRC_MSG} — arşiv bozuk veya eksik"
        ))
    } else {
        ArchiveError::Io(e)
    }
}

fn open_zip_entry<'a>(
    zip: &'a mut ZipArchive<File>,
    index: usize,
    password: Option<&[u8]>,
) -> Result<zip::read::ZipFile<'a, File>> {
    match password {
        Some(p) => zip.by_index_decrypt(index, p).map_err(map_zip_err),
        None => zip.by_index(index).map_err(map_zip_err),
    }
}

pub fn list(archive: &Path, password: Option<&str>) -> Result<Vec<String>> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file).map_err(map_zip_err)?;
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
    let mut zip = ZipArchive::new(file).map_err(map_zip_err)?;
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
            copy(&mut entry, &mut out).map_err(map_read_err)?;
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
pub const ZIP64_SIZE_THRESHOLD: u64 = u32::MAX as u64;

/// WinZip tarzı split ZIP: `arsiv.zip` + `arsiv.z01`, `arsiv.z02`, …
pub const SPLIT_ZIP_MAX_VOLUMES: u32 = 999;

fn eocd_indicates_spanning(bytes: &[u8]) -> bool {
    let Some(max_start) = bytes.len().checked_sub(22) else {
        return false;
    };
    for i in (0..=max_start).rev() {
        if bytes[i..i + 4] != [0x50, 0x4b, 0x05, 0x06] {
            continue;
        }
        let disk = u16::from_le_bytes([bytes[i + 4], bytes[i + 5]]);
        let cd_disk = u16::from_le_bytes([bytes[i + 6], bytes[i + 7]]);
        return disk != 0 || cd_disk != 0;
    }
    false
}

/// Birinci cilt yolu (`*.zip`) için olası split cilt yolları (W20).
pub fn split_zip_expected_companion_paths(first_volume: &Path) -> Vec<PathBuf> {
    let Some(stem) = first_volume.file_stem().and_then(|s| s.to_str()) else {
        return Vec::new();
    };
    let parent = first_volume.parent().unwrap_or_else(|| Path::new("."));
    (1..=SPLIT_ZIP_MAX_VOLUMES)
        .map(|n| parent.join(format!("{stem}.z{n:02}")))
        .collect()
}

/// Diskte mevcut ciltler (ilk cilt dahil).
pub fn split_zip_volumes_on_disk(first_volume: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if first_volume.is_file() {
        out.push(first_volume.to_path_buf());
    }
    for p in split_zip_expected_companion_paths(first_volume) {
        if p.is_file() {
            out.push(p);
        } else if !out.is_empty() && out.len() > 1 {
            break;
        } else if out.len() == 1 {
            continue;
        } else {
            break;
        }
    }
    out
}

/// Split / spanned ZIP göstergesi (EOCD disk alanları veya `.zNN` ciltleri).
pub fn archive_indicates_split_zip(archive: &Path) -> Result<bool> {
    if split_zip_volumes_on_disk(archive).len() > 1 {
        return Ok(true);
    }
    let bytes = std::fs::read(archive).map_err(ArchiveError::Io)?;
    Ok(eocd_indicates_spanning(&bytes))
}

/// Test: EOCD disk numaralarını span simülasyonu için yazar (gerçek çoklu cilt değil).
pub fn mark_split_zip_eocd_smoke_for_test(archive: &Path) -> Result<()> {
    let mut bytes = std::fs::read(archive).map_err(ArchiveError::Io)?;
    let pos = bytes
        .windows(4)
        .rposition(|w| w == [0x50, 0x4b, 0x05, 0x06])
        .ok_or_else(|| ArchiveError::Zip("EOCD bulunamadi".into()))?;
    if pos + 8 > bytes.len() {
        return Err(ArchiveError::Zip("EOCD cok kisa".into()));
    }
    bytes[pos + 4] = 1;
    bytes[pos + 5] = 0;
    bytes[pos + 6] = 1;
    bytes[pos + 7] = 0;
    std::fs::write(archive, bytes).map_err(ArchiveError::Io)?;
    Ok(())
}

/// Arşivde Zip64 EOCD (`PK\x06\x06`) veya Zip64 yerel başlık biti (I3 doğrulama).
pub fn archive_indicates_zip64(archive: &Path) -> Result<bool> {
    let bytes = std::fs::read(archive).map_err(|e| ArchiveError::Io(e))?;
    if bytes.windows(4).any(|w| w == [0x50, 0x4b, 0x06, 0x06]) {
        return Ok(true);
    }
    for (i, w) in bytes.windows(4).enumerate() {
        if w != [0x50, 0x4b, 0x03, 0x04] || i + 26 > bytes.len() {
            continue;
        }
        let gp = u16::from_le_bytes([bytes[i + 6], bytes[i + 7]]);
        if gp & 0x0800 != 0 {
            return Ok(true);
        }
        let zip64_size_sentinel = [0xFF, 0xFF, 0xFF, 0xFF];
        if bytes[i + 18..i + 22] == zip64_size_sentinel
            || bytes[i + 22..i + 26] == zip64_size_sentinel
        {
            return Ok(true);
        }
    }
    Ok(false)
}

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
    let mut zip = ZipArchive::new(file).map_err(map_zip_err)?;
    let pw = password_bytes(password);
    for i in 0..zip.len() {
        let mut entry = open_zip_entry(&mut zip, i, pw)?;
        std::io::copy(&mut entry, &mut std::io::sink()).map_err(map_read_err)?;
    }
    Ok(())
}

/// Test / I7: ilk dosya girdisinin sıkıştırılmış gövdesinde tek bayt bozar (CRC tetikler).
pub fn corrupt_first_local_crc_for_test(archive: &Path) -> Result<()> {
    let mut bytes = std::fs::read(archive)?;
    let pos = bytes
        .windows(4)
        .position(|w| w == [0x50, 0x4b, 0x03, 0x04])
        .ok_or_else(|| ArchiveError::Zip("ZIP yerel baslik bulunamadi".into()))?;
    if pos + 30 > bytes.len() {
        return Err(ArchiveError::Zip("ZIP baslik cok kisa".into()));
    }
    let fn_len = u16::from_le_bytes([bytes[pos + 26], bytes[pos + 27]]) as usize;
    let extra_len = u16::from_le_bytes([bytes[pos + 28], bytes[pos + 29]]) as usize;
    let data_start = pos + 30 + fn_len + extra_len;
    if data_start >= bytes.len() {
        return Err(ArchiveError::Zip("ZIP girdi verisi yok".into()));
    }
    bytes[data_start] ^= 0xff;
    std::fs::write(archive, bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn zip_corrupt_crc_controlled_error() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("a.txt");
        std::fs::write(&src, b"integrity").unwrap();
        let archive = dir.path().join("bad.zip");
        create_with_profile(&src, &archive, CompressionProfile::Balanced, None).unwrap();
        corrupt_first_local_crc_for_test(&archive).unwrap();
        let err = test(&archive, None).unwrap_err().to_string();
        assert!(
            err.contains(ZIP_INTEGRITY_CRC_MSG),
            "beklenen CRC mesaji, alinan: {err}"
        );
    }

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
    fn zip64_smoke_has_zip64_marker() {
        let dir = tempdir().unwrap();
        let archive = dir.path().join("z64.zip");
        create_zip64_smoke(&archive).unwrap();
        assert!(archive_indicates_zip64(&archive).unwrap(), "zip64 smoke marker yok");
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

    #[test]
    fn split_zip_eocd_smoke_marker() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("a.txt");
        std::fs::write(&src, b"split").unwrap();
        let archive = dir.path().join("split.zip");
        create_with_profile(&src, &archive, CompressionProfile::Balanced, None).unwrap();
        assert!(!archive_indicates_split_zip(&archive).unwrap());
        mark_split_zip_eocd_smoke_for_test(&archive).unwrap();
        assert!(archive_indicates_split_zip(&archive).unwrap());
    }

    #[test]
    fn split_zip_companion_volume_detection() {
        let dir = tempdir().unwrap();
        let first = dir.path().join("paket.zip");
        std::fs::write(&first, b"PK").unwrap();
        std::fs::write(dir.path().join("paket.z01"), b"vol2").unwrap();
        let vols = split_zip_volumes_on_disk(&first);
        assert_eq!(vols.len(), 2);
        assert!(archive_indicates_split_zip(&first).unwrap());
    }
}
