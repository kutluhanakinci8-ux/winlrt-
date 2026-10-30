//! Güvenli paylaşım paketi — yerel / self-hosted (KVKK, W25). Bulut yok.

use std::path::{Path, PathBuf};

use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::archive_sign;
use crate::audit;
use crate::profile::CompressionProfile;
use crate::zip_io;
use crate::{ArchiveError, Result};

pub const MANIFEST_VERSION: u32 = 1;
pub const MANIFEST_SUFFIX: &str = ".winlrtshare.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecureShareManifest {
    pub v: u32,
    pub product: String,
    /// Kişisel veri taşımaz; paket yalnızca kullanıcının seçtiği kanalla gider.
    pub kvkk_local_only: bool,
    pub created_at_unix: u64,
    pub pack_id: String,
    pub archive_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retention_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_hosted_base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SecureSharePackResult {
    pub zip_path: PathBuf,
    pub manifest_path: PathBuf,
    pub pack_id: String,
}

pub fn manifest_path_for_zip(zip: &Path) -> PathBuf {
    let parent = zip.parent().unwrap_or_else(|| Path::new("."));
    let stem = zip
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("share");
    parent.join(format!("{stem}{MANIFEST_SUFFIX}"))
}

fn new_pack_id() -> String {
    let mut buf = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut buf);
    hex::encode(buf)
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// AES ZIP + KVKK manifest (+ isteğe Ed25519 imza).
pub fn create_secure_share_pack(
    source: &Path,
    dest_zip: &Path,
    password: &str,
    self_hosted_base_url: Option<&str>,
    max_retention_days: Option<u32>,
) -> Result<SecureSharePackResult> {
    if password.trim().len() < 8 {
        return Err(ArchiveError::Zip(
            "guvenli paylasim icin parola en az 8 karakter".into(),
        ));
    }
    zip_io::create_with_profile(
        source,
        dest_zip,
        CompressionProfile::EmailAttachment,
        Some(password),
    )?;
    let pack_id = new_pack_id();
    let archive_name = dest_zip
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("share.zip")
        .to_string();
    let mut signature_file = None;
    if let Ok(seed_hex) = std::env::var("WINLRT_SIGNING_SEED_HEX") {
        if !seed_hex.trim().is_empty() {
            let seed = parse_seed_hex(&seed_hex)?;
            let key = archive_sign::signing_key_from_seed(seed);
            let sidecar = zip_io::sign_zip_archive(dest_zip, &key)?;
            signature_file = sidecar
                .file_name()
                .and_then(|s| s.to_str())
                .map(String::from);
        }
    }
    let manifest = SecureShareManifest {
        v: MANIFEST_VERSION,
        product: "Lertiva Zip".into(),
        kvkk_local_only: true,
        created_at_unix: now_unix(),
        pack_id: pack_id.clone(),
        archive_file: archive_name,
        signature_file,
        max_retention_days,
        self_hosted_base_url: self_hosted_base_url.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
    };
    let manifest_path = manifest_path_for_zip(dest_zip);
    let json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| ArchiveError::Zip(format!("manifest json: {e}")))?;
    std::fs::write(&manifest_path, json).map_err(ArchiveError::Io)?;
    audit::event_secure_share_pack("zip", true, &pack_id);
    Ok(SecureSharePackResult {
        zip_path: dest_zip.to_path_buf(),
        manifest_path,
        pack_id,
    })
}

pub fn read_manifest_for_zip(zip: &Path) -> Result<SecureShareManifest> {
    let path = manifest_path_for_zip(zip);
    let raw = std::fs::read_to_string(&path).map_err(ArchiveError::Io)?;
    let manifest: SecureShareManifest =
        serde_json::from_str(&raw).map_err(|e| ArchiveError::Zip(format!("manifest: {e}")))?;
    if manifest.v != MANIFEST_VERSION {
        return Err(ArchiveError::Zip(format!(
            "desteklenmeyen paylasim surumu {}",
            manifest.v
        )));
    }
    Ok(manifest)
}

/// Manifest + ZIP bütünlüğü (imza varsa doğrular).
pub fn verify_secure_share_pack(zip: &Path, password: Option<&str>) -> Result<SecureShareManifest> {
    let manifest = read_manifest_for_zip(zip)?;
    if !manifest.kvkk_local_only {
        return Err(ArchiveError::Zip(
            "KVKK: yalnizca kvkk_local_only paketler desteklenir".into(),
        ));
    }
    zip_io::test(zip, password)?;
    if let Some(sig_name) = &manifest.signature_file {
        let parent = zip.parent().unwrap_or_else(|| Path::new("."));
        let sidecar = parent.join(sig_name);
        if sidecar.is_file() {
            zip_io::verify_zip_archive_signature(zip)?;
        }
    }
    audit::event_secure_share_pack("zip", true, &format!("verify:{}", manifest.pack_id));
    Ok(manifest)
}

/// Self-hosted indirme URL şablonu (bulut servisi yok).
pub fn self_hosted_download_url(manifest: &SecureShareManifest, base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!("{base}/winlrt-share/{}/{}", manifest.pack_id, manifest.archive_file)
}

fn parse_seed_hex(hex: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(hex.trim())
        .map_err(|e| ArchiveError::Zip(format!("seed hex: {e}")))?;
    bytes
        .try_into()
        .map_err(|_| ArchiveError::Zip("seed 32 bayt olmali".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn manifest_path_next_to_zip() {
        let p = Path::new("/tmp/out/share.zip");
        assert_eq!(
            manifest_path_for_zip(p),
            Path::new("/tmp/out/share.winlrtshare.json")
        );
    }

    #[test]
    fn pack_roundtrip_without_cloud() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("doc.txt");
        std::fs::write(&src, b"kvkk-local-share").unwrap();
        let zip = dir.path().join("pack.zip");
        let pw = "Pilot-Share12";
        let result = create_secure_share_pack(&src, &zip, pw, None, Some(30)).unwrap();
        assert!(result.manifest_path.is_file());
        let manifest = verify_secure_share_pack(&zip, Some(pw)).unwrap();
        assert!(manifest.kvkk_local_only);
        assert_eq!(manifest.max_retention_days, Some(30));
        let url = self_hosted_download_url(&manifest, "https://files.example.com");
        assert!(url.contains(&manifest.pack_id));
        assert!(!url.contains("winzip"));
    }
}
