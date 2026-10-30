//! Kurumsal politika — ortam + Windows kayıt defteri (GPO hedefi, Faz 4 beta).

use std::path::PathBuf;

/// Kayıt yolu: `HKLM` / `HKCU` → `Software\winlrt\winlrtRAR`
pub const REG_KEY_PATH: &str = r"Software\winlrt\winlrtRAR";

#[cfg(windows)]
fn registry_value(name: &str) -> Option<String> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;
    for hive in [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        let root = RegKey::predef(hive);
        if let Ok(key) = root.open_subkey(REG_KEY_PATH) {
            if let Ok(v) = key.get_value::<String, _>(name) {
                if !v.trim().is_empty() {
                    return Some(v);
                }
            }
        }
    }
    None
}

#[cfg(windows)]
fn registry_dword(name: &str) -> Option<u32> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;
    for hive in [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        let root = RegKey::predef(hive);
        if let Ok(key) = root.open_subkey(REG_KEY_PATH) {
            if let Ok(v) = key.get_value::<u32, _>(name) {
                return Some(v);
            }
        }
    }
    None
}

#[cfg(not(windows))]
fn registry_dword(_name: &str) -> Option<u32> {
    None
}

#[cfg(not(windows))]
fn registry_value(_name: &str) -> Option<String> {
    None
}

/// `PasswordPolicy` = `corporate` | `consumer` (HKCU önce HKLM).
pub fn registry_password_policy_id() -> Option<String> {
    registry_value("PasswordPolicy")
}

/// Denetim kopyası — UNC veya yerel yol (`AuditMirrorPath`).
pub fn registry_audit_mirror_path() -> Option<PathBuf> {
    registry_value("AuditMirrorPath").map(PathBuf::from)
}

pub fn audit_mirror_path() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("WINLRT_AUDIT_MIRROR") {
        if !p.trim().is_empty() {
            return Some(PathBuf::from(p));
        }
    }
    registry_audit_mirror_path()
}

/// Merkezi HTTP/HTTPS toplayıcı (beta) — `AuditForwardUrl` / `WINLRT_AUDIT_FORWARD_URL`.
pub fn registry_audit_forward_url() -> Option<String> {
    registry_value("AuditForwardUrl")
}

pub fn audit_forward_url() -> Option<String> {
    if let Ok(p) = std::env::var("WINLRT_AUDIT_FORWARD_URL") {
        if !p.trim().is_empty() {
            return Some(p);
        }
    }
    registry_audit_forward_url()
}

/// `RequireModernCrypto` = `1` — kurumsal şifreli arşivde Argon2id + ZIP AES-256 zorunlu (Faz 3 kapı).
pub fn registry_require_modern_crypto() -> bool {
    if let Some(v) = registry_dword("RequireModernCrypto") {
        return v != 0;
    }
    registry_value("RequireModernCrypto")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn require_modern_crypto_enabled() -> bool {
    if std::env::var("WINLRT_REQUIRE_MODERN_CRYPTO")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return true;
    }
    registry_require_modern_crypto()
}

/// GPO + kurumsal politika: şifreli oluşturmada modern KDF/şifreleme ve kurumsal parola kuralları.
pub fn modern_crypto_gate_active() -> bool {
    require_modern_crypto_enabled()
        && crate::password_policy::active_policy().id == "corporate"
}

pub fn policy_id_from_env() -> Option<String> {
    std::env::var("WINLRT_PASSWORD_POLICY")
        .ok()
        .filter(|v| !v.trim().is_empty())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySource {
    Environment,
    Registry,
    Default,
}

pub fn resolve_policy_source() -> (PolicySource, Option<String>) {
    if let Some(id) = policy_id_from_env() {
        return (PolicySource::Environment, Some(id));
    }
    if let Some(id) = registry_password_policy_id() {
        return (PolicySource::Registry, Some(id));
    }
    (PolicySource::Default, None)
}
