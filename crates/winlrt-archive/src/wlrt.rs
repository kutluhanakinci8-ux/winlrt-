use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::WalkDir;
use zstd::stream::{read::Decoder, write::Encoder};

use crate::crypto::{self, random_salt};
use crate::profile::CompressionProfile;
use crate::{ArchiveError, Result};

const MAGIC: &[u8; 4] = b"WLRT";
const FOOTER: &[u8; 4] = b"TRLW";
const VERSION: u16 = 0;
const METHOD_ZSTD: u8 = 1;
const METHOD_ZSTD_ENC: u8 = 2;

fn encrypted_wlrt_version() -> u16 {
    crypto::WLRT_KDF_ARGON2ID_V2
}

fn wlrt_version_has_salt(version: u16) -> bool {
    version == crypto::WLRT_KDF_PBKDF2_V1 || version == crypto::WLRT_KDF_ARGON2ID_V2
}

struct WlrtHeader {
    file_count: u32,
    version: u16,
    salt: Option<[u8; 16]>,
}

struct EntryMeta {
    path: String,
    raw_size: u64,
    comp_size: u64,
    method: u8,
    level: u8,
    payload: Vec<u8>,
}

pub fn create(
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
    let encrypt = password.is_some_and(|p| !p.is_empty());
    if encrypt {
        let fb = crypto::evaluate_password(password.unwrap(), true);
        if !fb.ok_for_create {
            return Err(ArchiveError::Wlrt("parola politikası karşılanmıyor".into()));
        }
        if crate::enterprise::modern_crypto_gate_active()
            && encrypted_wlrt_version() != crypto::WLRT_KDF_ARGON2ID_V2
        {
            return Err(ArchiveError::Wlrt(
                "kurumsal kapı: .wlrt v2 (Argon2id) gerekli".into(),
            ));
        }
    }
    let level = profile.zstd_level();
    let files = collect_files(source)?;
    let salt = if encrypt {
        Some(random_salt())
    } else {
        None
    };
    let enc_version = encrypt.then_some(encrypted_wlrt_version());
    let entries: Vec<EntryMeta> = files
        .par_iter()
        .map(|(path, rel)| {
            compress_file(
                path,
                rel,
                level,
                password,
                salt.as_ref(),
                enc_version,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let mut out = File::create(dest)?;
    write_header(
        &mut out,
        entries.len() as u32,
        if encrypt {
            encrypted_wlrt_version()
        } else {
            VERSION
        },
        salt.as_ref(),
    )?;
    for e in &entries {
        write_entry(&mut out, e)?;
    }
    out.write_all(FOOTER)?;
    Ok(())
}

/// Çoklu seçimde arşive girecek `(disk yolu, arşiv içi ad)` listesi.
pub fn selection_file_list(sources: &[PathBuf]) -> Result<Vec<(PathBuf, String)>> {
    collect_selection_files(sources)
}

/// Explorer çoklu seçim: dosya ve/veya klasörleri tek `.wlrt` içinde birleştirir.
pub fn create_from_paths(
    sources: &[PathBuf],
    dest: &Path,
    profile: CompressionProfile,
    password: Option<&str>,
) -> Result<()> {
    if sources.is_empty() {
        return Err(ArchiveError::Wlrt("sıkıştırılacak öğe yok".into()));
    }
    let encrypt = password.is_some_and(|p| !p.is_empty());
    if encrypt {
        let fb = crypto::evaluate_password(password.unwrap(), true);
        if !fb.ok_for_create {
            return Err(ArchiveError::Wlrt("parola politikası karşılanmıyor".into()));
        }
        if crate::enterprise::modern_crypto_gate_active()
            && encrypted_wlrt_version() != crypto::WLRT_KDF_ARGON2ID_V2
        {
            return Err(ArchiveError::Wlrt(
                "kurumsal kapı: .wlrt v2 (Argon2id) gerekli".into(),
            ));
        }
    }
    let level = profile.zstd_level();
    let files = collect_selection_files(sources)?;
    let salt = if encrypt {
        Some(random_salt())
    } else {
        None
    };
    let enc_version = encrypt.then_some(encrypted_wlrt_version());
    let entries: Vec<EntryMeta> = files
        .par_iter()
        .map(|(path, rel)| {
            compress_file(
                path,
                rel,
                level,
                password,
                salt.as_ref(),
                enc_version,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let mut out = File::create(dest)?;
    write_header(
        &mut out,
        entries.len() as u32,
        if encrypt {
            encrypted_wlrt_version()
        } else {
            VERSION
        },
        salt.as_ref(),
    )?;
    for e in &entries {
        write_entry(&mut out, e)?;
    }
    out.write_all(FOOTER)?;
    Ok(())
}

pub fn extract(archive: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
    let mut file = File::open(archive)?;
    let header = read_header(&mut file)?;
    if header.salt.is_some() && password.is_none_or(|p| p.is_empty()) {
        return Err(ArchiveError::Wlrt("parola gerekli (şifreli .wlrt)".into()));
    }
    let mut entries = Vec::with_capacity(header.file_count as usize);
    for _ in 0..header.file_count {
        entries.push(read_entry(
            &mut file,
            header.version,
            header.salt.as_ref(),
            password,
        )?);
    }
    read_footer(&mut file)?;

    let parallel = header.salt.is_none() && entries.len() > 1;
    if parallel {
        entries.par_iter().try_for_each(|entry| write_decoded_entry(dest, entry))?;
    } else {
        for entry in entries {
            write_decoded_entry(dest, &entry)?;
        }
    }
    Ok(())
}

fn write_decoded_entry(dest: &Path, entry: &ParsedEntry) -> Result<()> {
    let out_path = dest.join(sanitize_path(&entry.path));
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut dec = Decoder::new(&entry.payload[..])
        .map_err(|e| ArchiveError::Wlrt(format!("zstd: {e}")))?;
    let mut out = File::create(&out_path)?;
    std::io::copy(&mut dec, &mut out)?;
    Ok(())
}

pub fn list(archive: &Path) -> Result<Vec<String>> {
    let mut file = File::open(archive)?;
    let header = read_header(&mut file)?;
    let mut names = Vec::with_capacity(header.file_count as usize);
    for _ in 0..header.file_count {
        let (path, comp_size, _method) = read_entry_meta(&mut file)?;
        names.push(path);
        skip_bytes(&mut file, comp_size)?;
    }
    read_footer(&mut file)?;
    Ok(names)
}

pub fn test(archive: &Path, password: Option<&str>) -> Result<()> {
    let mut file = File::open(archive)?;
    let header = read_header(&mut file)?;
    if header.salt.is_some() && password.is_none_or(|p| p.is_empty()) {
        return Err(ArchiveError::Wlrt("parola gerekli (şifreli .wlrt)".into()));
    }
    for _ in 0..header.file_count {
        let entry = read_entry(
            &mut file,
            header.version,
            header.salt.as_ref(),
            password,
        )?;
        let mut dec = Decoder::new(&entry.payload[..])
            .map_err(|e| ArchiveError::Wlrt(format!("zstd: {e}")))?;
        let mut sink = std::io::sink();
        let n = std::io::copy(&mut dec, &mut sink)?;
        if n != entry.raw_size {
            return Err(ArchiveError::Wlrt(format!(
                "boyut uyuşmazlığı: {} (beklenen {})",
                n, entry.raw_size
            )));
        }
    }
    read_footer(&mut file)?;
    Ok(())
}

pub fn magic() -> &'static [u8; 4] {
    MAGIC
}

fn collect_files(source: &Path) -> Result<Vec<(PathBuf, String)>> {
    let source = source.canonicalize()?;
    let mut out = Vec::new();
    if source.is_file() {
        let name = source
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "file".into());
        out.push((source, name));
        return Ok(out);
    }
    for entry in WalkDir::new(&source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let rel = path
                .strip_prefix(&source)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            out.push((path.to_path_buf(), rel));
        }
    }
    Ok(out)
}

fn collect_selection_files(sources: &[PathBuf]) -> Result<Vec<(PathBuf, String)>> {
    let mut out = Vec::new();
    for source in sources {
        if !source.exists() {
            return Err(ArchiveError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                source.display().to_string(),
            )));
        }
        let canonical = source.canonicalize()?;
        if canonical.is_file() {
            let name = canonical
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "file".into());
            let rel = uniquify_name(&out, &name);
            out.push((canonical, rel));
        } else {
            for (path, rel) in collect_files(&canonical)? {
                let entry = uniquify_name(&out, &rel);
                out.push((path, entry));
            }
        }
    }
    if out.is_empty() {
        return Err(ArchiveError::Wlrt("seçimde dosya yok".into()));
    }
    Ok(out)
}

fn uniquify_name(existing: &[(PathBuf, String)], rel: &str) -> String {
    if !existing.iter().any(|(_, r)| r == rel) {
        return rel.to_string();
    }
    let path = Path::new(rel);
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| rel.into());
    let ext = path.extension().map(|e| format!(".{}", e.to_string_lossy()));
    let mut n = 2u32;
    loop {
        let candidate = if let Some(ref e) = ext {
            format!("{stem}-{n}{e}")
        } else {
            format!("{stem}-{n}")
        };
        if !existing.iter().any(|(_, r)| r == &candidate) {
            return candidate;
        }
        n += 1;
    }
}

fn compress_file(
    path: &Path,
    rel: &str,
    level: i32,
    password: Option<&str>,
    salt: Option<&[u8; 16]>,
    wlrt_version: Option<u16>,
) -> Result<EntryMeta> {
    let mut raw = Vec::new();
    File::open(path)?.read_to_end(&mut raw)?;
    let raw_size = raw.len() as u64;
    let mut enc = Encoder::new(Vec::new(), level)
        .map_err(|e| ArchiveError::Wlrt(format!("zstd enc: {e}")))?;
    enc.write_all(&raw)
        .map_err(|e| ArchiveError::Wlrt(format!("zstd enc: {e}")))?;
    let compressed = enc
        .finish()
        .map_err(|e| ArchiveError::Wlrt(format!("zstd enc: {e}")))?;
    let (method, payload) = if let (Some(pw), Some(s), Some(ver)) = (
        password.filter(|p| !p.is_empty()),
        salt,
        wlrt_version,
    ) {
        let enc_payload = crypto::encrypt_blob(ver, pw, s, &compressed)?;
        (METHOD_ZSTD_ENC, enc_payload)
    } else {
        (METHOD_ZSTD, compressed)
    };
    Ok(EntryMeta {
        path: rel.to_string(),
        raw_size,
        comp_size: payload.len() as u64,
        method,
        level: level.clamp(0, 255) as u8,
        payload,
    })
}

fn write_header(
    w: &mut File,
    file_count: u32,
    version: u16,
    salt: Option<&[u8; 16]>,
) -> Result<()> {
    w.write_all(MAGIC)?;
    w.write_all(&version.to_le_bytes())?;
    w.write_all(&file_count.to_le_bytes())?;
    if wlrt_version_has_salt(version) {
        let s = salt.ok_or_else(|| ArchiveError::Wlrt("salt eksik".into()))?;
        w.write_all(s)?;
    }
    Ok(())
}

fn write_entry(w: &mut File, e: &EntryMeta) -> Result<()> {
    let path_bytes = e.path.as_bytes();
    if path_bytes.len() > u16::MAX as usize {
        return Err(ArchiveError::Wlrt("yol çok uzun".into()));
    }
    w.write_all(&(path_bytes.len() as u16).to_le_bytes())?;
    w.write_all(path_bytes)?;
    w.write_all(&e.raw_size.to_le_bytes())?;
    w.write_all(&e.comp_size.to_le_bytes())?;
    w.write_all(&[e.method, e.level])?;
    w.write_all(&e.payload)?;
    Ok(())
}

fn read_header(r: &mut File) -> Result<WlrtHeader> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(ArchiveError::Wlrt("geçersiz magic".into()));
    }
    let mut ver = [0u8; 2];
    r.read_exact(&mut ver)?;
    let version = u16::from_le_bytes(ver);
    if version != VERSION && !wlrt_version_has_salt(version) {
        return Err(ArchiveError::Wlrt("desteklenmeyen sürüm".into()));
    }
    let mut count = [0u8; 4];
    r.read_exact(&mut count)?;
    let file_count = u32::from_le_bytes(count);
    let salt = if wlrt_version_has_salt(version) {
        let mut s = [0u8; 16];
        r.read_exact(&mut s)?;
        Some(s)
    } else {
        None
    };
    Ok(WlrtHeader {
        file_count,
        version,
        salt,
    })
}

struct ParsedEntry {
    path: String,
    raw_size: u64,
    payload: Vec<u8>,
}

fn read_entry_meta(r: &mut File) -> Result<(String, u64, u8)> {
    let path = read_path(r)?;
    let mut raw_size = [0u8; 8];
    r.read_exact(&mut raw_size)?;
    let _raw = u64::from_le_bytes(raw_size);
    let mut comp_size = [0u8; 8];
    r.read_exact(&mut comp_size)?;
    let comp = u64::from_le_bytes(comp_size);
    let mut method_level = [0u8; 2];
    r.read_exact(&mut method_level)?;
    if method_level[0] != METHOD_ZSTD && method_level[0] != METHOD_ZSTD_ENC {
        return Err(ArchiveError::Wlrt(format!(
            "desteklenmeyen method {}",
            method_level[0]
        )));
    }
    Ok((path, comp, method_level[0]))
}

fn read_entry(
    r: &mut File,
    wlrt_version: u16,
    salt: Option<&[u8; 16]>,
    password: Option<&str>,
) -> Result<ParsedEntry> {
    let path = read_path(r)?;
    let mut raw_size = [0u8; 8];
    r.read_exact(&mut raw_size)?;
    let raw_size = u64::from_le_bytes(raw_size);
    let mut comp_size = [0u8; 8];
    r.read_exact(&mut comp_size)?;
    let comp_size = u64::from_le_bytes(comp_size);
    let mut method_level = [0u8; 2];
    r.read_exact(&mut method_level)?;
    let method = method_level[0];
    if method != METHOD_ZSTD && method != METHOD_ZSTD_ENC {
        return Err(ArchiveError::Wlrt(format!(
            "desteklenmeyen method {method}"
        )));
    }
    let mut payload = vec![0u8; comp_size as usize];
    r.read_exact(&mut payload)?;
    let payload = if method == METHOD_ZSTD_ENC {
        let s = salt.ok_or_else(|| ArchiveError::Wlrt("salt eksik".into()))?;
        let pw = password.filter(|p| !p.is_empty()).ok_or_else(|| {
            ArchiveError::Wlrt("parola gerekli (şifreli girdi)".into())
        })?;
        crypto::decrypt_blob(wlrt_version, pw, s, &payload)?
    } else {
        payload
    };
    Ok(ParsedEntry {
        path,
        raw_size,
        payload,
    })
}

fn read_path(r: &mut File) -> Result<String> {
    let mut len = [0u8; 2];
    r.read_exact(&mut len)?;
    let len = u16::from_le_bytes(len) as usize;
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|e| ArchiveError::Wlrt(format!("yol utf-8: {e}")))
}

fn skip_bytes(r: &mut File, n: u64) -> Result<()> {
    let mut left = n;
    let mut buf = [0u8; 8192];
    while left > 0 {
        let chunk = std::cmp::min(left, buf.len() as u64) as usize;
        r.read_exact(&mut buf[..chunk])?;
        left -= chunk as u64;
    }
    Ok(())
}

fn read_footer(r: &mut File) -> Result<()> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if &magic != FOOTER {
        return Err(ArchiveError::Wlrt("geçersiz footer".into()));
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn wlrt_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("a.txt"), b"winlrt wlrt test").unwrap();
        let archive = dir.path().join("t.wlrt");
        let out = dir.path().join("out");
        create(&src, &archive, CompressionProfile::Fast, None).unwrap();
        test(&archive, None).unwrap();
        let names = list(&archive).unwrap();
        assert_eq!(names, vec!["a.txt".to_string()]);
        std::fs::create_dir_all(&out).unwrap();
        extract(&archive, &out, None).unwrap();
        let got = std::fs::read(out.join("a.txt")).unwrap();
        assert_eq!(got, b"winlrt wlrt test");
    }

    #[test]
    fn wlrt_encrypted_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("secret.txt"), b"encrypted wlrt").unwrap();
        let archive = dir.path().join("enc.wlrt");
        create(
            &src,
            &archive,
            CompressionProfile::Balanced,
            Some("test-password-12"),
        )
        .unwrap();
        list(&archive).unwrap();
        test(&archive, Some("test-password-12")).unwrap();
        let out = dir.path().join("out");
        std::fs::create_dir_all(&out).unwrap();
        extract(&archive, &out, Some("test-password-12")).unwrap();
        assert_eq!(
            std::fs::read(out.join("secret.txt")).unwrap(),
            b"encrypted wlrt"
        );
    }
}
