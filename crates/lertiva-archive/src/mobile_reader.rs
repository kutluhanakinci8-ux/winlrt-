//! Mobil okuyucu sözleşmesi (Faz 6 smoke, W26) — salt okuma + paylaşım hazırlığı.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::archive_sign;
use crate::secure_share;
use crate::zip_io;
use crate::{ArchiveError, Result};

pub const MOBILE_READER_CONTRACT_VERSION: &str = "1";
pub const MOBILE_MAX_ENTRY_PREVIEW: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MobileCapabilities {
    pub contract_version: String,
    pub product: String,
    pub create_archive: bool,
    pub extract_archive: bool,
    pub list_entries: bool,
    pub verify_share_pack: bool,
    pub lerta_share_intent_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MobileInspectReport {
    pub archive_path: String,
    pub size_bytes: u64,
    pub entry_count: usize,
    pub entries_preview: Vec<String>,
    pub entries_truncated: bool,
    pub split_zip: bool,
    pub zip64: bool,
    pub signed_sidecar: bool,
    pub secure_share_manifest: bool,
    pub encryption_detected: bool,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaShareIntentPayload {
    pub contract_version: String,
    pub pack_id: String,
    pub archive_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    pub kvkk_local_only: bool,
}

pub fn capabilities() -> MobileCapabilities {
    MobileCapabilities {
        contract_version: MOBILE_READER_CONTRACT_VERSION.into(),
        product: "Lertiva Zip".into(),
        create_archive: false,
        extract_archive: true,
        list_entries: true,
        verify_share_pack: true,
        lerta_share_intent_ready: true,
    }
}

pub fn inspect(archive: &Path, password: Option<&str>) -> Result<MobileInspectReport> {
    let size_bytes = std::fs::metadata(archive).map_err(ArchiveError::Io)?.len();
    let split_zip = zip_io::archive_indicates_split_zip(archive).unwrap_or(false);
    let zip64 = zip_io::archive_indicates_zip64(archive).unwrap_or(false);
    let signed_sidecar = archive_sign::sidecar_path_for_archive(archive).is_file();
    let secure_share_manifest = secure_share::manifest_path_for_zip(archive).is_file();
    let encryption_detected = match zip_io::list(archive, password) {
        Ok(entries) => {
            let entry_count = entries.len();
            let truncated = entry_count > MOBILE_MAX_ENTRY_PREVIEW;
            let preview: Vec<String> = entries
                .into_iter()
                .take(MOBILE_MAX_ENTRY_PREVIEW)
                .collect();
            return Ok(MobileInspectReport {
                archive_path: archive.display().to_string(),
                size_bytes,
                entry_count,
                entries_preview: preview,
                entries_truncated: truncated,
                split_zip,
                zip64,
                signed_sidecar,
                secure_share_manifest,
                encryption_detected: false,
                read_only: true,
            });
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            if password.is_none()
                && (msg.contains("password") || msg.contains("parola") || msg.contains("decrypt"))
            {
                true
            } else {
                return Err(e);
            }
        }
    };
    Ok(MobileInspectReport {
        archive_path: archive.display().to_string(),
        size_bytes,
        entry_count: 0,
        entries_preview: Vec::new(),
        entries_truncated: false,
        split_zip,
        zip64,
        signed_sidecar,
        secure_share_manifest,
        encryption_detected,
        read_only: true,
    })
}

pub fn lerta_share_intent_from_zip(
    archive: &Path,
    download_url: Option<&str>,
) -> Result<LertaShareIntentPayload> {
    let manifest = secure_share::read_manifest_for_zip(archive)?;
    Ok(LertaShareIntentPayload {
        contract_version: MOBILE_READER_CONTRACT_VERSION.into(),
        pack_id: manifest.pack_id,
        archive_file: manifest.archive_file,
        download_url: download_url.map(String::from),
        kvkk_local_only: manifest.kvkk_local_only,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::CompressionProfile;
    use tempfile::tempdir;

    #[test]
    fn capabilities_read_only_create_false() {
        let caps = capabilities();
        assert!(!caps.create_archive);
        assert!(caps.extract_archive);
        assert_eq!(caps.contract_version, "1");
    }

    #[test]
    fn inspect_plain_zip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("m.txt");
        std::fs::write(&src, b"mobile").unwrap();
        let zip = dir.path().join("m.zip");
        zip_io::create_with_profile(&src, &zip, CompressionProfile::Fast, None).unwrap();
        let report = inspect(&zip, None).unwrap();
        assert!(!report.encryption_detected);
        assert!(report.entries_preview.iter().any(|e| e.contains("m.txt")));
        assert!(report.read_only);
    }

    #[test]
    fn lerta_intent_from_share_pack() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("s.txt");
        std::fs::write(&src, b"share").unwrap();
        let zip = dir.path().join("pack.zip");
        secure_share::create_secure_share_pack(&src, &zip, "Mobile-Sh12", None, None).unwrap();
        let intent = lerta_share_intent_from_zip(&zip, Some("https://cdn.example/x")).unwrap();
        assert!(intent.kvkk_local_only);
        assert_eq!(intent.download_url.as_deref(), Some("https://cdn.example/x"));
    }
}
