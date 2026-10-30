use std::path::Path;

use crate::{ArchiveError, Result};

const MAGIC: &[u8; 4] = b"WLRT";

/// `.wlrt` v0 — Faz 1 iskelet (spec: docs/WINLRTRAR_PLAN.md).
pub fn create(_source: &Path, _dest: &Path) -> Result<()> {
    Err(ArchiveError::NotYet(".wlrt oluşturma (Faz 1 sprint 2)"))
}

pub fn extract(_archive: &Path, _dest: &Path) -> Result<()> {
    Err(ArchiveError::NotYet(".wlrt açma (Faz 1 sprint 2)"))
}

pub fn list(_archive: &Path) -> Result<Vec<String>> {
    Err(ArchiveError::NotYet(".wlrt listeleme (Faz 1 sprint 2)"))
}

pub fn test(_archive: &Path) -> Result<()> {
    Err(ArchiveError::NotYet(".wlrt test (Faz 1 sprint 2)"))
}

#[allow(dead_code)]
pub fn magic() -> &'static [u8; 4] {
    MAGIC
}
