//! Mağaza / güncelleme hattı kapısı (Faz 7 smoke, W27).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{ArchiveError, Result};

pub const STORE_GATE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoreGateItem {
    pub id: String,
    pub platform: String,
    pub pass: bool,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoreGateReport {
    pub v: u32,
    pub product: String,
    pub app_version: String,
    pub bundle_identifier: Option<String>,
    pub all_pass: bool,
    pub items: Vec<StoreGateItem>,
}

fn read_file(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn item(id: &str, platform: &str, pass: bool, note: impl Into<String>) -> StoreGateItem {
    StoreGateItem {
        id: id.into(),
        platform: platform.into(),
        pass,
        note: note.into(),
    }
}

/// Play / App Store ön kapısı — repo dosyaları (gerçek mağaza yükleme değil).
pub fn evaluate_store_gate(workspace: &Path) -> Result<StoreGateReport> {
    let tauri_conf = workspace.join("apps/lertiva-zip/src-tauri/tauri.conf.json");
    let conf = read_file(&tauri_conf).ok_or_else(|| {
        ArchiveError::Zip(format!("tauri.conf bulunamadi: {}", tauri_conf.display()))
    })?;
    let app_version = extract_json_string(&conf, "\"version\"").unwrap_or_else(|| "?".into());
    let bundle_id = extract_json_string(&conf, "\"identifier\"");
    let mut items = Vec::new();

    items.push(item(
        "bundle_identifier",
        "both",
        bundle_id.as_deref() == Some("com.lertiva.zip"),
        "Tauri identifier com.lertiva.zip",
    ));
    items.push(item(
        "short_description",
        "both",
        conf.contains("shortDescription"),
        "NSIS / mağaza kısa açıklama",
    ));
    items.push(item(
        "macos_min_os",
        "app_store",
        conf.contains("minimumSystemVersion"),
        "macOS minimumSystemVersion",
    ));
    items.push(item(
        "privacy_audit_doc",
        "both",
        workspace
            .join("docs/WINLRTRAR_AUDIT.md")
            .is_file(),
        "KVKK denetim dokümantasyonu",
    ));
    items.push(item(
        "mobile_reader_contract",
        "both",
        workspace
            .join("docs/qa/mobile-reader-v1.schema.json")
            .is_file(),
        "Mobil okuyucu sözleşmesi (W26)",
    ));
    items.push(item(
        "kvkk_secure_share",
        "both",
        workspace
            .join("docs/qa/MADDE_W25_WINLRTZIP_SECURE_SHARE.md")
            .is_file(),
        "Güvenli paylaşım / KVKK (W25)",
    ));
    items.push(item(
        "ci_windows_release",
        "both",
        workspace
            .join(".github/workflows/winlrt-zip-windows.yml")
            .is_file(),
        "Windows güncelleme hattı workflow",
    ));
    items.push(item(
        "ci_macos_release",
        "app_store",
        workspace
            .join(".github/workflows/winlrt-zip-macos.yml")
            .is_file(),
        "macOS dmg hattı (W14)",
    ));
    let win_ci = read_file(&workspace.join(".github/workflows/winlrt-zip-windows.yml"));
    items.push(item(
        "update_channel_github",
        "both",
        win_ci.as_ref().is_some_and(|s| s.contains("release") || s.contains("workflow_dispatch")),
        "GitHub release / dispatch kanalı",
    ));
    items.push(item(
        "play_data_safety_note",
        "play",
        workspace
            .join("docs/qa/WINLRTZIP_STORE_DATA_SAFETY.md")
            .is_file(),
        "Play Data safety özet şablonu",
    ));

    let all_pass = items.iter().all(|i| i.pass);
    Ok(StoreGateReport {
        v: STORE_GATE_VERSION,
        product: "Lertiva Zip".into(),
        app_version,
        bundle_identifier: bundle_id,
        all_pass,
        items,
    })
}

fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let idx = json.find(key)? + key.len();
    let rest = json[idx..].trim_start_matches([':', ' ', '\t']);
    let quote = rest.find('"')? + 1;
    let end = rest[quote..].find('"')? + quote;
    Some(rest[quote..end].to_string())
}

pub fn default_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_gate_repo_smoke() {
        let root = default_workspace_root();
        let report = evaluate_store_gate(&root).expect("store gate");
        assert_eq!(report.product, "Lertiva Zip");
        assert!(
            report.items.iter().any(|i| i.id == "bundle_identifier" && i.pass),
            "bundle id: {:?}",
            report.items
        );
        assert!(
            report.items.iter().any(|i| i.id == "ci_windows_release" && i.pass),
            "windows ci"
        );
    }
}
