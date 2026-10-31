//! Birleşik kurtarma raporları (ZIP kısmi + RAR onarım).

pub use crate::zip_io::{PartialExtractFailure, PartialExtractReport};

use std::path::Path;

use crate::{rar, zip_io, ArchiveError, Result};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecoverySummary {
    pub format: String,
    pub extracted_count: usize,
    pub failed_count: usize,
    pub message: String,
}

/// ZIP kısmi çıkarma veya RAR onarım — CLI / Arc ortak giriş.
pub fn recover_partial_zip(
    archive: &Path,
    dest: &Path,
    password: Option<&str>,
) -> Result<PartialExtractReport> {
    zip_io::extract_partial(archive, dest, password)
}

pub fn recover_repair_rar(
    archive: &Path,
    dest: &Path,
    password: Option<&str>,
) -> Result<RecoverySummary> {
    rar::repair(archive, dest, password)?;
    Ok(RecoverySummary {
        format: "rar".into(),
        extracted_count: 0,
        failed_count: 0,
        message: format!("RAR onarım tamamlandı → {}", dest.display()),
    })
}

pub fn extract_one_any(
    format: &str,
    archive: &Path,
    entry: &str,
    dest_file: &Path,
    password: Option<&str>,
) -> Result<()> {
    match format {
        "zip" => zip_io::extract_one(archive, entry, dest_file, password),
        "wlrt" => crate::wlrt::extract_one(archive, entry, dest_file, password),
        "rar" => rar::extract_one(archive, entry, dest_file, password),
        "7z" => crate::sevenz::extract_one(archive, entry, dest_file, password),
        _ => Err(ArchiveError::Zip(format!("desteklenmeyen format: {format}"))),
    }
}
