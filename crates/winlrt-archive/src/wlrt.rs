use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::WalkDir;
use zstd::stream::{read::Decoder, write::Encoder};

use crate::profile::CompressionProfile;
use crate::{ArchiveError, Result};

const MAGIC: &[u8; 4] = b"WLRT";
const FOOTER: &[u8; 4] = b"TRLW";
const VERSION: u16 = 0;
const METHOD_ZSTD: u8 = 1;

struct EntryMeta {
    path: String,
    raw_size: u64,
    comp_size: u64,
    level: u8,
    payload: Vec<u8>,
}

pub fn create(source: &Path, dest: &Path, profile: CompressionProfile) -> Result<()> {
    if !source.exists() {
        return Err(ArchiveError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            source.display().to_string(),
        )));
    }
    let level = profile.zstd_level();
    let files = collect_files(source)?;
    let entries: Vec<EntryMeta> = files
        .par_iter()
        .map(|(path, rel)| compress_file(path, rel, level))
        .collect::<Result<Vec<_>>>()?;

    let mut out = File::create(dest)?;
    write_header(&mut out, entries.len() as u32)?;
    for e in &entries {
        write_entry(&mut out, e)?;
    }
    out.write_all(FOOTER)?;
    Ok(())
}

pub fn extract(archive: &Path, dest: &Path) -> Result<()> {
    let mut file = File::open(archive)?;
    let count = read_header(&mut file)?;
    for _ in 0..count {
        let entry = read_entry(&mut file)?;
        let out_path = dest.join(sanitize_path(&entry.path));
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut dec = Decoder::new(&entry.payload[..])
            .map_err(|e| ArchiveError::Wlrt(format!("zstd: {e}")))?;
        let mut out = File::create(&out_path)?;
        std::io::copy(&mut dec, &mut out)?;
    }
    read_footer(&mut file)?;
    Ok(())
}

pub fn list(archive: &Path) -> Result<Vec<String>> {
    let mut file = File::open(archive)?;
    let count = read_header(&mut file)?;
    let mut names = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let (path, comp_size) = read_entry_meta(&mut file)?;
        names.push(path);
        skip_bytes(&mut file, comp_size)?;
    }
    read_footer(&mut file)?;
    Ok(names)
}

pub fn test(archive: &Path) -> Result<()> {
    let mut file = File::open(archive)?;
    let count = read_header(&mut file)?;
    for _ in 0..count {
        let entry = read_entry(&mut file)?;
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

fn compress_file(path: &Path, rel: &str, level: i32) -> Result<EntryMeta> {
    let mut raw = Vec::new();
    File::open(path)?.read_to_end(&mut raw)?;
    let raw_size = raw.len() as u64;
    let mut enc = Encoder::new(Vec::new(), level)
        .map_err(|e| ArchiveError::Wlrt(format!("zstd enc: {e}")))?;
    enc.write_all(&raw)
        .map_err(|e| ArchiveError::Wlrt(format!("zstd enc: {e}")))?;
    let payload = enc
        .finish()
        .map_err(|e| ArchiveError::Wlrt(format!("zstd enc: {e}")))?;
    Ok(EntryMeta {
        path: rel.to_string(),
        raw_size,
        comp_size: payload.len() as u64,
        level: level.clamp(0, 255) as u8,
        payload,
    })
}

fn write_header(w: &mut File, file_count: u32) -> Result<()> {
    w.write_all(MAGIC)?;
    w.write_all(&VERSION.to_le_bytes())?;
    w.write_all(&file_count.to_le_bytes())?;
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
    w.write_all(&[METHOD_ZSTD, e.level])?;
    w.write_all(&e.payload)?;
    Ok(())
}

fn read_header(r: &mut File) -> Result<u32> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(ArchiveError::Wlrt("geçersiz magic".into()));
    }
    let mut ver = [0u8; 2];
    r.read_exact(&mut ver)?;
    if u16::from_le_bytes(ver) != VERSION {
        return Err(ArchiveError::Wlrt("desteklenmeyen sürüm".into()));
    }
    let mut count = [0u8; 4];
    r.read_exact(&mut count)?;
    Ok(u32::from_le_bytes(count))
}

struct ParsedEntry {
    path: String,
    raw_size: u64,
    payload: Vec<u8>,
}

fn read_entry_meta(r: &mut File) -> Result<(String, u64)> {
    let path = read_path(r)?;
    let mut raw_size = [0u8; 8];
    r.read_exact(&mut raw_size)?;
    let _raw = u64::from_le_bytes(raw_size);
    let mut comp_size = [0u8; 8];
    r.read_exact(&mut comp_size)?;
    let comp = u64::from_le_bytes(comp_size);
    let mut method_level = [0u8; 2];
    r.read_exact(&mut method_level)?;
    if method_level[0] != METHOD_ZSTD {
        return Err(ArchiveError::Wlrt(format!(
            "desteklenmeyen method {}",
            method_level[0]
        )));
    }
    Ok((path, comp))
}

fn read_entry(r: &mut File) -> Result<ParsedEntry> {
    let path = read_path(r)?;
    let mut raw_size = [0u8; 8];
    r.read_exact(&mut raw_size)?;
    let raw_size = u64::from_le_bytes(raw_size);
    let mut comp_size = [0u8; 8];
    r.read_exact(&mut comp_size)?;
    let comp_size = u64::from_le_bytes(comp_size);
    let mut method_level = [0u8; 2];
    r.read_exact(&mut method_level)?;
    if method_level[0] != METHOD_ZSTD {
        return Err(ArchiveError::Wlrt(format!(
            "desteklenmeyen method {}",
            method_level[0]
        )));
    }
    let mut payload = vec![0u8; comp_size as usize];
    r.read_exact(&mut payload)?;
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
        create(&src, &archive, CompressionProfile::Fast).unwrap();
        test(&archive).unwrap();
        let names = list(&archive).unwrap();
        assert_eq!(names, vec!["a.txt".to_string()]);
        std::fs::create_dir_all(&out).unwrap();
        extract(&archive, &out).unwrap();
        let got = std::fs::read(out.join("a.txt")).unwrap();
        assert_eq!(got, b"winlrt wlrt test");
    }
}
