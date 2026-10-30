use std::path::Path;

use crate::{ArchiveError, Result};

/// RAR / RAR5 **okuma** — Faz 2 (bkz. docs/RAR_READING.md).
pub fn list(_archive: &Path) -> Result<Vec<String>> {
    Err(ArchiveError::NotYet(
        "RAR listeleme — Faz 2 (UnRAR entegrasyonu)",
    ))
}

pub fn extract(_archive: &Path, _dest: &Path) -> Result<()> {
    Err(ArchiveError::NotYet(
        "RAR açma — Faz 2 (UnRAR entegrasyonu)",
    ))
}

pub fn test(_archive: &Path) -> Result<()> {
    Err(ArchiveError::NotYet("RAR test — Faz 2"))
}
