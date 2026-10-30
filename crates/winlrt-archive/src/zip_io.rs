use std::fs::File;
use std::io::{copy, Write};
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

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
    let opened = SplitZipOpen::resolve(archive)?;
    let file = File::open(&opened.path)?;
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
    let opened = SplitZipOpen::resolve(archive)?;
    let file = File::open(&opened.path)?;
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

/// WinZip / Info-ZIP split: `arsiv.z01` … `arsiv.zNN` + son cilt `arsiv.zip`.
pub const SPLIT_ZIP_MAX_VOLUMES: u32 = 999;
/// Info-ZIP `zip -s` ile uyumlu minimum cilt boyutu (64 KiB).
pub const SPLIT_ZIP_MIN_VOLUME_BYTES: u64 = 64 * 1024;

/// Split yazma özeti (W22).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitZipWriteResult {
    pub volume_count: u32,
    pub total_bytes: u64,
}

/// Birleştirilmiş geçici arşiv (W21); `_merged` kapanınca silinir.
struct SplitZipOpen {
    path: PathBuf,
    _merged: Option<NamedTempFile>,
}

impl SplitZipOpen {
    fn resolve(archive: &Path) -> Result<Self> {
        let vols = split_zip_ordered_volumes(archive);
        if vols.len() <= 1 {
            return Ok(Self {
                path: archive.to_path_buf(),
                _merged: None,
            });
        }
        let mut tmp = NamedTempFile::new().map_err(ArchiveError::Io)?;
        for v in &vols {
            let mut f = File::open(v).map_err(ArchiveError::Io)?;
            copy(&mut f, &mut tmp).map_err(ArchiveError::Io)?;
        }
        tmp.flush().map_err(ArchiveError::Io)?;
        let mut merged = std::fs::read(tmp.path()).map_err(ArchiveError::Io)?;
        normalize_merged_split_eocd(&mut merged);
        std::fs::write(tmp.path(), &merged).map_err(ArchiveError::Io)?;
        Ok(Self {
            path: tmp.path().to_path_buf(),
            _merged: Some(tmp),
        })
    }
}

fn split_zip_part_number(name: &str, stem: &str) -> Option<u32> {
    let prefix = format!("{stem}.");
    if !name.starts_with(&prefix) {
        return None;
    }
    let ext = &name[prefix.len()..];
    let digits = ext.strip_prefix('z').or_else(|| ext.strip_prefix('Z'))?;
    if digits.len() != 2 {
        return None;
    }
    let n: u32 = digits.parse().ok()?;
    (1..=SPLIT_ZIP_MAX_VOLUMES).contains(&n).then_some(n)
}

fn split_zip_stem_from_name(file_name: &str) -> Option<String> {
    if file_name.len() > 4 && file_name.ends_with(".zip") {
        return Some(file_name[..file_name.len() - 4].to_string());
    }
    let dot = file_name.rfind('.')?;
    let stem = &file_name[..dot];
    split_zip_part_number(file_name, stem).map(|_| stem.to_string())
}

/// Diskteki ciltler, birleştirme sırasıyla (`.z01` … `.zip`).
pub fn split_zip_ordered_volumes(selected: &Path) -> Vec<PathBuf> {
    let Some(file_name) = selected.file_name().and_then(|s| s.to_str()) else {
        return selected
            .is_file()
            .then(|| vec![selected.to_path_buf()])
            .unwrap_or_default();
    };
    let Some(stem) = split_zip_stem_from_name(file_name) else {
        return vec![selected.to_path_buf()];
    };
    let parent = selected.parent().unwrap_or_else(|| Path::new("."));
    let mut parts: Vec<(u32, PathBuf)> = Vec::new();
    let mut last_zip: Option<PathBuf> = None;
    let Ok(read_dir) = std::fs::read_dir(parent) else {
        return vec![selected.to_path_buf()];
    };
    for ent in read_dir.flatten() {
        let Some(name) = ent.file_name().to_str().map(|s| s.to_string()) else {
            continue;
        };
        if name == format!("{stem}.zip") {
            last_zip = Some(ent.path());
        } else if let Some(n) = split_zip_part_number(&name, &stem) {
            parts.push((n, ent.path()));
        }
    }
    parts.sort_by_key(|(n, _)| *n);
    let mut out: Vec<PathBuf> = parts.into_iter().map(|(_, p)| p).collect();
    if let Some(z) = last_zip {
        out.push(z);
    }
    if out.is_empty() && selected.is_file() {
        out.push(selected.to_path_buf());
    }
    out
}

fn eocd_scan_last(bytes: &[u8]) -> Option<usize> {
    let max_start = bytes.len().checked_sub(22)?;
    (0..=max_start).rev().find(|&i| bytes[i..i + 4] == [0x50, 0x4b, 0x05, 0x06])
}

fn eocd_indicates_spanning(bytes: &[u8]) -> bool {
    let Some(i) = eocd_scan_last(bytes) else {
        return false;
    };
    let disk = u16::from_le_bytes([bytes[i + 4], bytes[i + 5]]);
    let cd_disk = u16::from_le_bytes([bytes[i + 6], bytes[i + 7]]);
    disk != 0 || cd_disk != 0
}

/// Birleştirilmiş arşivde EOCD disk alanlarını tek cilt olarak işaretler (zip crate uyumu).
fn normalize_merged_split_eocd(bytes: &mut [u8]) {
    let Some(i) = eocd_scan_last(bytes) else {
        return;
    };
    bytes[i + 4] = 0;
    bytes[i + 5] = 0;
    bytes[i + 6] = 0;
    bytes[i + 7] = 0;
}

/// Diskte mevcut ciltler (birleştirme sırasıyla).
pub fn split_zip_volumes_on_disk(entry: &Path) -> Vec<PathBuf> {
    split_zip_ordered_volumes(entry)
}

/// Aynı stem altındaki önceki split ciltlerini siler (`*.zNN`, hedef `*.zip`).
pub fn split_zip_remove_existing_volumes(dest_zip: &Path) -> Result<()> {
    let Some(stem) = dest_zip.file_stem().and_then(|s| s.to_str()) else {
        return Ok(());
    };
    let parent = dest_zip.parent().unwrap_or_else(|| Path::new("."));
    let Ok(read_dir) = std::fs::read_dir(parent) else {
        return Ok(());
    };
    for ent in read_dir.flatten() {
        let Some(name) = ent.file_name().to_str().map(|s| s.to_string()) else {
            continue;
        };
        if name == format!("{stem}.zip") || split_zip_part_number(&name, stem).is_some() {
            let _ = std::fs::remove_file(ent.path());
        }
    }
    Ok(())
}

/// Tam ZIP baytlarını cilt dosyalarına yazar (W21 birleştirme ile uyumlu ham bölme).
pub fn split_zip_write_volumes(
    bytes: &[u8],
    dest_zip: &Path,
    volume_size: u64,
) -> Result<SplitZipWriteResult> {
    if volume_size < SPLIT_ZIP_MIN_VOLUME_BYTES {
        return Err(ArchiveError::Zip(format!(
            "split cilt boyutu en az {} B olmalı",
            SPLIT_ZIP_MIN_VOLUME_BYTES
        )));
    }
    if dest_zip.extension().is_none_or(|e| e != "zip") {
        return Err(ArchiveError::Zip("split hedefi .zip uzantılı olmalı".into()));
    }
    let Some(stem) = dest_zip.file_stem().and_then(|s| s.to_str()) else {
        return Err(ArchiveError::Zip("split hedef adı geçersiz".into()));
    };
    let parent = dest_zip.parent().unwrap_or_else(|| Path::new("."));
    split_zip_remove_existing_volumes(dest_zip)?;
    let total = bytes.len() as u64;
    if total <= volume_size {
        std::fs::write(dest_zip, bytes).map_err(ArchiveError::Io)?;
        return Ok(SplitZipWriteResult {
            volume_count: 1,
            total_bytes: total,
        });
    }
    let chunk_size = volume_size as usize;
    let chunks: Vec<&[u8]> = bytes.chunks(chunk_size).collect();
    if chunks.len() > SPLIT_ZIP_MAX_VOLUMES as usize {
        return Err(ArchiveError::Zip(format!(
            "çok fazla cilt (>{SPLIT_ZIP_MAX_VOLUMES})"
        )));
    }
    for (i, chunk) in chunks.iter().enumerate() {
        let path = if i + 1 == chunks.len() {
            parent.join(format!("{stem}.zip"))
        } else {
            parent.join(format!("{stem}.z{:02}", i + 1))
        };
        std::fs::write(path, chunk).map_err(ArchiveError::Io)?;
    }
    Ok(SplitZipWriteResult {
        volume_count: chunks.len() as u32,
        total_bytes: total,
    })
}

/// Kaynak → ZIP oluştur → ciltlere böl (W22).
pub fn create_split_with_profile(
    source: &Path,
    dest: &Path,
    profile: CompressionProfile,
    password: Option<&str>,
    volume_size: u64,
) -> Result<SplitZipWriteResult> {
    let tmp = NamedTempFile::new().map_err(ArchiveError::Io)?;
    create_with_profile(source, tmp.path(), profile, password)?;
    let bytes = std::fs::read(tmp.path()).map_err(ArchiveError::Io)?;
    split_zip_write_volumes(&bytes, dest, volume_size)
}

/// Çok ciltli arşivi tek geçici dosyada birleştirir (W21); tek ciltte `Ok(None)`.
pub fn split_zip_materialize_merged(entry: &Path) -> Result<Option<NamedTempFile>> {
    let vols = split_zip_ordered_volumes(entry);
    if vols.len() <= 1 {
        return Ok(None);
    }
    let mut tmp = NamedTempFile::new().map_err(ArchiveError::Io)?;
    for v in &vols {
        let mut f = File::open(v).map_err(ArchiveError::Io)?;
        copy(&mut f, &mut tmp).map_err(ArchiveError::Io)?;
    }
    tmp.flush().map_err(ArchiveError::Io)?;
    let mut merged = std::fs::read(tmp.path()).map_err(ArchiveError::Io)?;
    normalize_merged_split_eocd(&mut merged);
    std::fs::write(tmp.path(), &merged).map_err(ArchiveError::Io)?;
    Ok(Some(tmp))
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
    let opened = SplitZipOpen::resolve(archive)?;
    let file = File::open(&opened.path)?;
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
    use std::process::Command;
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
        std::fs::write(dir.path().join("paket.z01"), b"vol1").unwrap();
        let vols = split_zip_volumes_on_disk(&first);
        assert_eq!(vols.len(), 2);
        assert_eq!(vols[0].file_name().unwrap(), "paket.z01");
        assert_eq!(vols[1].file_name().unwrap(), "paket.zip");
        assert!(archive_indicates_split_zip(&first).unwrap());
    }

    #[test]
    fn split_zip_merge_synthetic_two_part_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("piece.txt");
        std::fs::write(&src, b"w21-synthetic-split-merge").unwrap();
        let whole = dir.path().join("whole.zip");
        create_with_profile(&src, &whole, CompressionProfile::Fast, None).unwrap();
        let bytes = std::fs::read(&whole).unwrap();
        let mid = bytes.len() / 2;
        std::fs::write(dir.path().join("syn.z01"), &bytes[..mid]).unwrap();
        std::fs::write(dir.path().join("syn.zip"), &bytes[mid..]).unwrap();
        let vols = split_zip_volumes_on_disk(&dir.path().join("syn.zip"));
        assert_eq!(vols.len(), 2);
        test(&dir.path().join("syn.zip"), None).unwrap();
        let out = dir.path().join("out");
        extract(&dir.path().join("syn.z01"), &out, None).unwrap();
        assert_eq!(
            std::fs::read(out.join("piece.txt")).unwrap(),
            b"w21-synthetic-split-merge"
        );
    }

    #[test]
    fn split_zip_write_multivolume_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("chunk.bin"), vec![0xCDu8; 150 * 1024]).unwrap();
        std::fs::write(src.join("note.txt"), b"w22-split-write").unwrap();
        let dest = dir.path().join("out.zip");
        let info = create_split_with_profile(
            &src,
            &dest,
            CompressionProfile::Fast,
            None,
            SPLIT_ZIP_MIN_VOLUME_BYTES,
        )
        .unwrap();
        assert!(info.volume_count >= 2, "volume_count={}", info.volume_count);
        assert!(archive_indicates_split_zip(&dest).unwrap());
        test(&dest, None).unwrap();
        let extract_dir = dir.path().join("extracted");
        extract(&dest, &extract_dir, None).unwrap();
        assert_eq!(
            std::fs::read(extract_dir.join("note.txt")).unwrap(),
            b"w22-split-write"
        );
        assert_eq!(
            std::fs::read(extract_dir.join("chunk.bin")).unwrap().len(),
            150 * 1024
        );
    }

    #[test]
    fn split_zip_write_rejects_small_volume_size() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("x.txt"), b"x").unwrap();
        let err = create_split_with_profile(
            &dir.path().join("x.txt"),
            &dir.path().join("t.zip"),
            CompressionProfile::Fast,
            None,
            1024,
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("en az"));
    }

    #[test]
    fn split_zip_infozip_volume_order_and_byte_len() {
        if Command::new("zip").arg("-h").output().is_err() {
            return;
        }
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("payload.bin"), vec![0u8; 200 * 1024]).unwrap();
        let status = Command::new("zip")
            .current_dir(dir.path())
            .args(["-0", "-s", "64k", "split.zip", "payload.bin"])
            .status()
            .unwrap();
        if !status.success() {
            return;
        }
        let vols = split_zip_volumes_on_disk(&dir.path().join("split.zip"));
        assert!(vols.len() >= 2);
        assert_eq!(vols.last().unwrap().file_name().unwrap(), "split.zip");
        let sum: u64 = vols.iter().map(|p| std::fs::metadata(p).unwrap().len()).sum();
        let merged = split_zip_materialize_merged(&dir.path().join("split.zip"))
            .unwrap()
            .unwrap();
        assert_eq!(std::fs::metadata(merged.path()).unwrap().len(), sum);
    }
}
