//! Lerta mesaj eki (Faz 8, W30) — `.wlrt` veya AES-ZIP profili.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::audit;
use crate::profile::CompressionProfile;
use crate::{wlrt, zip_io, ArchiveError, Result};

pub const LERTA_MESSAGE_CONTRACT_VERSION: &str = "1";
pub const LERTA_MESSAGE_MAX_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LertaMessageFormat {
    WlrtV0,
    AesZip,
}

impl LertaMessageFormat {
    pub fn id(self) -> &'static str {
        match self {
            Self::WlrtV0 => "wlrt_v0",
            Self::AesZip => "aes_zip",
        }
    }

    pub fn mime(self) -> &'static str {
        match self {
            Self::WlrtV0 => "application/vnd.winlrt.wlrt",
            Self::AesZip => "application/zip",
        }
    }

    pub fn default_extension(self) -> &'static str {
        match self {
            Self::WlrtV0 => "wlrt",
            Self::AesZip => "zip",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaMessagePrepareResult {
    pub contract_version: String,
    pub product: String,
    pub format: LertaMessageFormat,
    pub attachment_path: String,
    pub filename: String,
    pub size_bytes: u64,
    pub sha256_hex: String,
    pub mime: String,
    pub within_limit: bool,
    pub kvkk_local_only: bool,
    pub content_stays_on_device: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaMessageWebhookPayload {
    pub contract_version: String,
    pub product: String,
    pub channel: String,
    pub format: LertaMessageFormat,
    pub filename: String,
    pub size_bytes: u64,
    pub sha256_hex: String,
    pub mime: String,
    pub within_limit: bool,
    pub kvkk_local_only: bool,
    pub content_stays_on_device: bool,
}

pub fn prepare_lerta_message_attachment(
    source: &Path,
    dest: &Path,
    format: LertaMessageFormat,
    password: Option<&str>,
) -> Result<LertaMessagePrepareResult> {
    match format {
        LertaMessageFormat::WlrtV0 => {
            if password.is_some() {
                wlrt::create(source, dest, CompressionProfile::Balanced, password)?;
            } else {
                wlrt::create(source, dest, CompressionProfile::Balanced, None)?;
            }
        }
        LertaMessageFormat::AesZip => {
            let pw = password.ok_or_else(|| {
                ArchiveError::Zip("AES-ZIP mesaj eki icin parola gerekli".into())
            })?;
            zip_io::create_with_profile(source, dest, CompressionProfile::Balanced, Some(pw))?;
        }
    }
    let meta = std::fs::metadata(dest).map_err(ArchiveError::Io)?;
    let size_bytes = meta.len();
    if size_bytes > LERTA_MESSAGE_MAX_BYTES {
        audit::event_lerta_message("zip", false, "size_over_limit");
        return Err(ArchiveError::Zip(format!(
            "Lerta mesaj eki limiti asildi: {size_bytes} B"
        )));
    }
    let bytes = std::fs::read(dest).map_err(ArchiveError::Io)?;
    let sha256_hex = hex::encode(Sha256::digest(&bytes));
    let filename = dest
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("ek")
        .to_string();
    audit::event_lerta_message("zip", true, &format!("prepare:{}", format.id()));
    Ok(LertaMessagePrepareResult {
        contract_version: LERTA_MESSAGE_CONTRACT_VERSION.into(),
        product: "Lertiva Zip".into(),
        format,
        attachment_path: dest.display().to_string(),
        filename,
        size_bytes,
        sha256_hex,
        mime: format.mime().into(),
        within_limit: true,
        kvkk_local_only: true,
        content_stays_on_device: true,
    })
}

pub fn validate_lerta_message_attachment(path: &Path, format: LertaMessageFormat) -> Result<()> {
    match format {
        LertaMessageFormat::WlrtV0 => wlrt::test(path, None)?,
        LertaMessageFormat::AesZip => zip_io::test(path, None)?,
    }
    let size = std::fs::metadata(path).map_err(ArchiveError::Io)?.len();
    if size > LERTA_MESSAGE_MAX_BYTES {
        audit::event_lerta_message("zip", false, "validate_size");
        return Err(ArchiveError::Zip(format!("mesaj eki limit ustunde: {size} B")));
    }
    audit::event_lerta_message("zip", true, "validate_ok");
    Ok(())
}

pub fn webhook_payload_from_prepare(prep: &LertaMessagePrepareResult) -> LertaMessageWebhookPayload {
    LertaMessageWebhookPayload {
        contract_version: prep.contract_version.clone(),
        product: prep.product.clone(),
        channel: "lerta_message".into(),
        format: prep.format,
        filename: prep.filename.clone(),
        size_bytes: prep.size_bytes,
        sha256_hex: prep.sha256_hex.clone(),
        mime: prep.mime.clone(),
        within_limit: prep.within_limit,
        kvkk_local_only: prep.kvkk_local_only,
        content_stays_on_device: prep.content_stays_on_device,
    }
}

pub fn suggest_dest_path(source: &Path, format: LertaMessageFormat) -> PathBuf {
    let stem = source
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("mesaj");
    PathBuf::from(format!("{stem}.{}", format.default_extension()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn lerta_message_wlrt_roundtrip() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("msg.txt");
        std::fs::write(&src, b"lerta-mesaj-w30").unwrap();
        let dest = dir.path().join("mesaj.wlrt");
        let prep = prepare_lerta_message_attachment(
            &src,
            &dest,
            LertaMessageFormat::WlrtV0,
            None,
        )
        .unwrap();
        assert_eq!(prep.format, LertaMessageFormat::WlrtV0);
        validate_lerta_message_attachment(&dest, LertaMessageFormat::WlrtV0).unwrap();
        let hook = webhook_payload_from_prepare(&prep);
        assert!(hook.kvkk_local_only);
    }
}
