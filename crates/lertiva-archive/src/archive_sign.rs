//! Kurumsal imzalı ZIP — Ed25519 yan dosya (W23).

use std::path::{Path, PathBuf};

use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{ArchiveError, Result};

pub const MANIFEST_VERSION: u32 = 1;
pub const SIGNATURE_DOMAIN: &[u8] = b"Lertiva Zip-archive-signature-v1\0";

/// Yan dosya: `rapor.zip` → `rapor.winlrtsig.json`
pub const SIGNATURE_SUFFIX: &str = ".winlrtsig.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchiveSignatureManifest {
    pub v: u32,
    pub product: String,
    pub archive_sha256: String,
    pub public_key_hex: String,
    pub signature_hex: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignArchiveResult {
    pub sidecar_written: bool,
}

pub fn sidecar_path_for_archive(archive: &Path) -> PathBuf {
    let parent = archive.parent().unwrap_or_else(|| Path::new("."));
    let stem = archive
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("archive");
    parent.join(format!("{stem}{SIGNATURE_SUFFIX}"))
}

pub fn generate_signing_key() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}

pub fn signing_key_from_seed(seed: [u8; 32]) -> SigningKey {
    SigningKey::from_bytes(&seed)
}

pub fn verifying_key_from_manifest(manifest: &ArchiveSignatureManifest) -> Result<VerifyingKey> {
    let pk = hex_to_32(&manifest.public_key_hex, "public_key_hex")?;
    VerifyingKey::from_bytes(&pk).map_err(|e| ArchiveError::Zip(format!("ed25519 anahtar: {e}")))
}

fn hex_to_32(hex: &str, field: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(hex.trim())
        .map_err(|e| ArchiveError::Zip(format!("{field} hex: {e}")))?;
    bytes
        .try_into()
        .map_err(|_| ArchiveError::Zip(format!("{field} 32 bayt olmali")))
}

fn hex_to_64(hex: &str, field: &str) -> Result<[u8; 64]> {
    let bytes = hex::decode(hex.trim())
        .map_err(|e| ArchiveError::Zip(format!("{field} hex: {e}")))?;
    bytes
        .try_into()
        .map_err(|_| ArchiveError::Zip(format!("{field} 64 bayt olmali")))
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex::encode(digest)
}

fn sign_message(archive_sha256: &[u8; 32]) -> Vec<u8> {
    let mut msg = Vec::with_capacity(SIGNATURE_DOMAIN.len() + 32);
    msg.extend_from_slice(SIGNATURE_DOMAIN);
    msg.extend_from_slice(archive_sha256);
    msg
}

pub fn sign_archive_bytes(
    archive_bytes: &[u8],
    signing_key: &SigningKey,
) -> ArchiveSignatureManifest {
    let digest = Sha256::digest(archive_bytes);
    let digest_arr: [u8; 32] = digest.into();
    let signature = signing_key.sign(&sign_message(&digest_arr));
    let verifying = signing_key.verifying_key();
    ArchiveSignatureManifest {
        v: MANIFEST_VERSION,
        product: "Lertiva Zip".into(),
        archive_sha256: hex::encode(digest_arr),
        public_key_hex: hex::encode(verifying.to_bytes()),
        signature_hex: hex::encode(signature.to_bytes()),
    }
}

pub fn write_manifest(path: &Path, manifest: &ArchiveSignatureManifest) -> Result<()> {
    let json = serde_json::to_string_pretty(manifest)
        .map_err(|e| ArchiveError::Zip(format!("manifest json: {e}")))?;
    std::fs::write(path, json).map_err(ArchiveError::Io)?;
    Ok(())
}

pub fn read_manifest(path: &Path) -> Result<ArchiveSignatureManifest> {
    let raw = std::fs::read_to_string(path).map_err(ArchiveError::Io)?;
    let manifest: ArchiveSignatureManifest =
        serde_json::from_str(&raw).map_err(|e| ArchiveError::Zip(format!("manifest: {e}")))?;
    if manifest.v != MANIFEST_VERSION {
        return Err(ArchiveError::Zip(format!(
            "desteklenmeyen imza surumu {}",
            manifest.v
        )));
    }
    Ok(manifest)
}

pub fn verify_manifest_for_bytes(
    archive_bytes: &[u8],
    manifest: &ArchiveSignatureManifest,
) -> Result<()> {
    let expected = sha256_hex(archive_bytes);
    if !manifest.archive_sha256.eq_ignore_ascii_case(&expected) {
        return Err(ArchiveError::Zip(
            "imza manifesti arsiv ozeti ile uyusmuyor".into(),
        ));
    }
    let digest = hex_to_32(&manifest.archive_sha256, "archive_sha256")?;
    let sig = hex_to_64(&manifest.signature_hex, "signature_hex")?;
    let vk = verifying_key_from_manifest(manifest)?;
    let sig = ed25519_dalek::Signature::from_bytes(&sig);
    vk.verify(&sign_message(&digest), &sig)
        .map_err(|_| ArchiveError::Zip("arsiv imzasi gecersiz".into()))?;
    Ok(())
}

pub fn verify_archive_bytes_with_sidecar(
    archive_bytes: &[u8],
    sidecar: &Path,
) -> Result<()> {
    let manifest = read_manifest(sidecar)?;
    verify_manifest_for_bytes(archive_bytes, &manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn sign_and_verify_roundtrip() {
        let bytes = b"signed-zip-payload";
        let key = signing_key_from_seed([7u8; 32]);
        let manifest = sign_archive_bytes(bytes, &key);
        verify_manifest_for_bytes(bytes, &manifest).unwrap();
    }

    #[test]
    fn tamper_detected() {
        let bytes = b"original";
        let key = signing_key_from_seed([9u8; 32]);
        let manifest = sign_archive_bytes(bytes, &key);
        let err = verify_manifest_for_bytes(b"tampered", &manifest).unwrap_err();
        assert!(err.to_string().contains("uyusmuyor"));
    }

    #[test]
    fn sidecar_path_stem() {
        let p = Path::new("/tmp/pkg.zip");
        assert_eq!(
            sidecar_path_for_archive(p),
            Path::new("/tmp/pkg.winlrtsig.json")
        );
    }

    #[test]
    fn write_read_manifest_file() {
        let dir = tempdir().unwrap();
        let sidecar = dir.path().join("a.winlrtsig.json");
        let key = generate_signing_key();
        let manifest = sign_archive_bytes(b"abc", &key);
        write_manifest(&sidecar, &manifest).unwrap();
        let read = read_manifest(&sidecar).unwrap();
        assert_eq!(read, manifest);
    }
}
