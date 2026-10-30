//! Lerta posta eki otomasyonu (Faz 8 smoke, W28) — W7 e-posta profili ile hizalı.

use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::audit;
use crate::profile::{CompressionProfile, EMAIL_ATTACHMENT_MAX_ZIP_BYTES};
use crate::zip_io;
use crate::{ArchiveError, Result};

pub const LERTA_MAIL_CONTRACT_VERSION: &str = "1";
pub const LERTA_EXTENDED_MAX_ZIP_BYTES: u64 = 25 * 1024 * 1024;
pub const LERTA_MAIL_MIME: &str = "application/zip";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LertaAttachmentTier {
    Standard10Mb,
    Extended25Mb,
}

impl LertaAttachmentTier {
    pub fn id(self) -> &'static str {
        match self {
            Self::Standard10Mb => "standard10_mb",
            Self::Extended25Mb => "extended25_mb",
        }
    }

    pub fn max_bytes(self) -> u64 {
        match self {
            Self::Standard10Mb => EMAIL_ATTACHMENT_MAX_ZIP_BYTES,
            Self::Extended25Mb => LERTA_EXTENDED_MAX_ZIP_BYTES,
        }
    }

    pub fn compression_profile(self) -> CompressionProfile {
        match self {
            Self::Standard10Mb => CompressionProfile::EmailAttachment,
            Self::Extended25Mb => CompressionProfile::Balanced,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaAttachmentPrepareResult {
    pub contract_version: String,
    pub product: String,
    pub tier: LertaAttachmentTier,
    pub zip_path: String,
    pub filename: String,
    pub size_bytes: u64,
    pub sha256_hex: String,
    pub mime: String,
    pub within_limit: bool,
    pub suggested_attachment_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaMailWebhookPayload {
    pub contract_version: String,
    pub product: String,
    pub tier: LertaAttachmentTier,
    pub filename: String,
    pub size_bytes: u64,
    pub sha256_hex: String,
    pub mime: String,
    pub within_limit: bool,
    pub kvkk_local_only: bool,
    pub content_stays_on_device: bool,
}

pub fn prepare_lerta_mail_attachment(
    source: &Path,
    dest_zip: &Path,
    tier: LertaAttachmentTier,
    password: Option<&str>,
) -> Result<LertaAttachmentPrepareResult> {
    let profile = tier.compression_profile();
    zip_io::create_with_profile(source, dest_zip, profile, password)?;
    let meta = std::fs::metadata(dest_zip).map_err(ArchiveError::Io)?;
    let size_bytes = meta.len();
    let within_limit = size_bytes <= tier.max_bytes();
    if !within_limit {
        audit::event_lerta_mail("zip", false, "size_over_limit");
        return Err(ArchiveError::Zip(format!(
            "Lerta posta eki limiti asildi: {size_bytes} B > {} B",
            tier.max_bytes()
        )));
    }
    let bytes = std::fs::read(dest_zip).map_err(ArchiveError::Io)?;
    let sha256_hex = hex::encode(Sha256::digest(&bytes));
    let filename = dest_zip
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("ek.zip")
        .to_string();
    let suggested_attachment_name = if filename.ends_with(".zip") {
        filename.clone()
    } else {
        format!("{filename}.zip")
    };
    audit::event_lerta_mail("zip", true, &format!("prepare:{}", tier.id()));
    Ok(LertaAttachmentPrepareResult {
        contract_version: LERTA_MAIL_CONTRACT_VERSION.into(),
        product: "Lertiva Zip".into(),
        tier,
        zip_path: dest_zip.display().to_string(),
        filename,
        size_bytes,
        sha256_hex,
        mime: LERTA_MAIL_MIME.into(),
        within_limit,
        suggested_attachment_name,
    })
}

pub fn validate_lerta_mail_attachment(zip: &Path, tier: LertaAttachmentTier) -> Result<()> {
    zip_io::test(zip, None)?;
    let size = std::fs::metadata(zip).map_err(ArchiveError::Io)?.len();
    if size > tier.max_bytes() {
        audit::event_lerta_mail("zip", false, "validate_size");
        return Err(ArchiveError::Zip(format!(
            "ek boyutu limit ustunde: {size} B"
        )));
    }
    audit::event_lerta_mail("zip", true, "validate_ok");
    Ok(())
}

pub fn webhook_payload_from_prepare(prep: &LertaAttachmentPrepareResult) -> LertaMailWebhookPayload {
    LertaMailWebhookPayload {
        contract_version: prep.contract_version.clone(),
        product: prep.product.clone(),
        tier: prep.tier,
        filename: prep.filename.clone(),
        size_bytes: prep.size_bytes,
        sha256_hex: prep.sha256_hex.clone(),
        mime: prep.mime.clone(),
        within_limit: prep.within_limit,
        kvkk_local_only: true,
        content_stays_on_device: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn lerta_standard_tier_small_file() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("mail.txt");
        std::fs::write(&src, b"lerta-posta-eki").unwrap();
        let zip = dir.path().join("ek.zip");
        let prep = prepare_lerta_mail_attachment(
            &src,
            &zip,
            LertaAttachmentTier::Standard10Mb,
            None,
        )
        .unwrap();
        assert!(prep.within_limit);
        validate_lerta_mail_attachment(&zip, LertaAttachmentTier::Standard10Mb).unwrap();
        let hook = webhook_payload_from_prepare(&prep);
        assert!(hook.kvkk_local_only);
        assert!(hook.content_stays_on_device);
    }
}
