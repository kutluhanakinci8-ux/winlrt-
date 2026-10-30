use std::path::Path;

use rar::Archive;

use crate::{ArchiveError, Result};

use super::Op;

const LIST_FALLBACK_MAX_BYTES: u64 = 50 * 1024 * 1024;

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| ArchiveError::Rar("geçersiz dosya yolu".into()))
}

fn map_rar(err: rar::error::RarError) -> ArchiveError {
    ArchiveError::Rar(err.to_string())
}

pub fn run(archive: &Path, password: Option<&str>, op: &Op<'_>) -> Result<Vec<String>> {
    match op {
        Op::List => list_via_rar_crate(archive, password.unwrap_or("")),
        Op::Extract(dest) => {
            std::fs::create_dir_all(dest)?;
            let pw = password.unwrap_or("");
            Archive::extract_all(path_str(archive)?, path_str(dest)?, pw).map_err(map_rar)?;
            Ok(vec![])
        }
        Op::Test => {
            let temp = tempfile::tempdir()?;
            let pw = password.unwrap_or("");
            Archive::extract_all(path_str(archive)?, path_str(temp.path())?, pw)
                .map_err(map_rar)?;
            Ok(vec![])
        }
    }
}

fn list_via_rar_crate(archive: &Path, password: &str) -> Result<Vec<String>> {
    let len = std::fs::metadata(archive)?.len();
    if len > LIST_FALLBACK_MAX_BYTES {
        return Err(ArchiveError::Rar(
            "Büyük RAR: RAR5 listesi için UnRAR kullanın.".into(),
        ));
    }
    let temp = tempfile::tempdir()?;
    let arch = Archive::extract_all(path_str(archive)?, path_str(temp.path())?, password)
        .map_err(map_rar)?;
    Ok(arch.files.into_iter().map(|f| f.name).collect())
}
