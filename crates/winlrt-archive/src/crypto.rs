use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::Argon2;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;

use crate::password_policy::{self, PasswordPolicy};
use crate::{ArchiveError, Result};

const PBKDF2_ITER: u32 = 100_000;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

/// `.wlrt` v1 şifreli başlık — PBKDF2-SHA256
pub const WLRT_KDF_PBKDF2_V1: u16 = 1;
/// `.wlrt` v2 şifreli başlık — Argon2id (yeni oluşturma)
pub const WLRT_KDF_ARGON2ID_V2: u16 = 2;

pub fn random_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

fn derive_key_pbkdf2(password: &str, salt: &[u8; SALT_LEN]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITER, &mut key);
    key
}

fn derive_key_argon2id(password: &str, salt: &[u8; SALT_LEN]) -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| ArchiveError::Wlrt(format!("argon2: {e}")))?;
    Ok(key)
}

fn derive_key(wlrt_version: u16, password: &str, salt: &[u8; SALT_LEN]) -> Result<[u8; 32]> {
    match wlrt_version {
        WLRT_KDF_PBKDF2_V1 => Ok(derive_key_pbkdf2(password, salt)),
        WLRT_KDF_ARGON2ID_V2 => derive_key_argon2id(password, salt),
        _ => Err(ArchiveError::Wlrt(format!(
            "desteklenmeyen şifreli sürüm {wlrt_version}"
        ))),
    }
}

/// nonce (12) + ciphertext + tag (16)
pub fn encrypt_blob(
    wlrt_version: u16,
    password: &str,
    salt: &[u8; SALT_LEN],
    plaintext: &[u8],
) -> Result<Vec<u8>> {
    let key = derive_key(wlrt_version, password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| ArchiveError::Wlrt(format!("aes: {e}")))?;
    let mut nonce = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .map_err(|e| ArchiveError::Wlrt(format!("şifreleme: {e}")))?;
    let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn decrypt_blob(
    wlrt_version: u16,
    password: &str,
    salt: &[u8; SALT_LEN],
    blob: &[u8],
) -> Result<Vec<u8>> {
    if blob.len() <= NONCE_LEN {
        return Err(ArchiveError::Wlrt("şifreli veri çok kısa".into()));
    }
    let key = derive_key(wlrt_version, password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| ArchiveError::Wlrt(format!("aes: {e}")))?;
    let (nonce_bytes, ct) = blob.split_at(NONCE_LEN);
    cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ct)
        .map_err(|_| ArchiveError::Wlrt("parola hatalı veya arşiv bozuk".into()))
}

pub fn evaluate_password(password: &str, for_create: bool) -> PasswordFeedback {
    evaluate_password_with_policy(password, for_create, password_policy::active_policy())
}

pub fn evaluate_password_with_policy(
    password: &str,
    for_create: bool,
    policy: PasswordPolicy,
) -> PasswordFeedback {
    let len = password.chars().count();
    let mut score = 0u8;
    if len >= 8 {
        score += 1;
    }
    if len >= 12 {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_uppercase())
        && password.chars().any(|c| c.is_ascii_lowercase())
    {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }
    if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        score += 1;
    }
    let strength = match score {
        0..=1 => "zayıf",
        2..=3 => "orta",
        _ => "güçlü",
    };
    let violations = if for_create {
        policy.violations(password)
    } else {
        Vec::new()
    };
    let ok_for_create = !for_create || violations.is_empty();
    let hint = if for_create && !violations.is_empty() {
        policy_hint(&policy, &violations)
    } else if for_create {
        "ZIP: AES-256 (WinZip AE-2). .wlrt v2: AES-256-GCM + Argon2id."
    } else {
        "Parolalı RAR / 7z / ZIP / .wlrt için girin."
    };
    PasswordFeedback {
        length: len as u32,
        strength,
        ok_for_create,
        hint,
        policy_id: policy.id,
        policy_label: policy.label,
        violations: violations.into_iter().map(|v| v.to_string()).collect(),
    }
}

fn policy_hint(policy: &PasswordPolicy, violations: &[&str]) -> &'static str {
    let _ = (policy, violations);
    if violations.contains(&"min_length") {
        return "Parola politikası: minimum uzunluk karşılanmıyor.";
    }
    if violations.contains(&"upper") {
        return "Parola politikası: büyük harf gerekli.";
    }
    if violations.contains(&"lower") {
        return "Parola politikası: küçük harf gerekli.";
    }
    if violations.contains(&"digit") {
        return "Parola politikası: rakam gerekli.";
    }
    if violations.contains(&"symbol") {
        return "Parola politikası: özel karakter gerekli.";
    }
    "Parola politikası karşılanmıyor."
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PasswordFeedback {
    pub length: u32,
    pub strength: &'static str,
    pub ok_for_create: bool,
    pub hint: &'static str,
    pub policy_id: &'static str,
    pub policy_label: &'static str,
    pub violations: Vec<String>,
}
