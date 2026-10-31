//! İçerik hash ile yinelenen dosya grupları (G9 dedup spike — solid öncesi).

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;

use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::{ArchiveError, Result};

#[derive(Debug, Clone, serde::Serialize)]
pub struct DuplicateGroup {
    pub hash_hex: String,
    pub paths: Vec<String>,
    pub size_bytes: u64,
}

/// Aynı SHA-256 içeriğe sahip dosya gruplarını döner (yalnızca `min_size` ve üzeri).
pub fn scan_duplicate_files(root: &Path, min_size: u64) -> Result<Vec<DuplicateGroup>> {
    if !root.is_dir() {
        return Err(ArchiveError::Zip(format!(
            "dedup tarama: klasör değil {}",
            root.display()
        )));
    }
    let mut by_hash: HashMap<[u8; 32], (u64, Vec<String>)> = HashMap::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let meta = std::fs::metadata(path)?;
        if meta.len() < min_size {
            continue;
        }
        let hash = hash_file(path)?;
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        by_hash
            .entry(hash)
            .and_modify(|(_, paths)| paths.push(rel.clone()))
            .or_insert((meta.len(), vec![rel]));
    }
    let mut groups: Vec<DuplicateGroup> = by_hash
        .into_iter()
        .filter(|(_, (_, paths))| paths.len() > 1)
        .map(|(hash, (size, paths))| DuplicateGroup {
            hash_hex: hex::encode(hash),
            paths,
            size_bytes: size,
        })
        .collect();
    groups.sort_by(|a, b| b.paths.len().cmp(&a.paths.len()));
    Ok(groups)
}

/// `.wlrt` oluştururken aynı içerik hash'ini tek girdi olarak bırakır (`LERTIVA_WLRT_DEDUP=1`).
pub fn dedup_file_list(files: &[(std::path::PathBuf, String)]) -> Result<Vec<(std::path::PathBuf, String)>> {
    if std::env::var("LERTIVA_WLRT_DEDUP").ok().as_deref() != Some("1") {
        return Ok(files.to_vec());
    }
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(files.len());
    for (path, rel) in files {
        let hash = hash_file(path)?;
        if seen.insert(hash) {
            out.push((path.clone(), rel.clone()));
        }
    }
    Ok(out)
}

fn hash_file(path: &Path) -> Result<[u8; 32]> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn dedup_finds_identical_files() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("src");
        std::fs::create_dir_all(root.join("a")).unwrap();
        std::fs::write(root.join("a/x.txt"), b"same").unwrap();
        std::fs::write(root.join("a/y.txt"), b"same").unwrap();
        std::fs::write(root.join("unique.txt"), b"other").unwrap();
        let groups = scan_duplicate_files(&root, 1).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].paths.len(), 2);
    }
}
