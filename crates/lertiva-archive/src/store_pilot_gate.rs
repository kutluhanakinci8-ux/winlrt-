//! Mağaza + mobil pilot hazırlık kapısı (Post-v3 W32) — W27 + W26 + W31 + vitrin marka.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::brand::PRODUCT_ZIP;
use crate::lerta_integration::lerta_integration_status_from_env;
use crate::mobile_reader;
use crate::store_gate;
use crate::{ArchiveError, Result};

pub const STORE_PILOT_GATE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StorePilotGateItem {
    pub id: String,
    pub pass: bool,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StorePilotGateReport {
    pub v: u32,
    pub product: String,
    pub all_pass: bool,
    pub store_gate_pass: bool,
    pub lerta_staging_pass: bool,
    pub mobile_reader_ready: bool,
    pub vitrin_brand_pass: bool,
    pub items: Vec<StorePilotGateItem>,
}

fn item(id: &str, pass: bool, note: impl Into<String>) -> StorePilotGateItem {
    StorePilotGateItem {
        id: id.into(),
        pass,
        note: note.into(),
    }
}

fn file_contains(path: &Path, needle: &str) -> bool {
    std::fs::read_to_string(path)
        .ok()
        .is_some_and(|s| s.contains(needle))
}

fn vitrin_brand_checks(workspace: &Path) -> (bool, Vec<StorePilotGateItem>) {
    let mut items = Vec::new();
    let index = workspace.join("apps/web/public/index.html");
    let idx_ok = index.is_file()
        && file_contains(&index, PRODUCT_ZIP)
        && file_contains(&index, "Lertiva")
        && !file_contains(&index, "winlrtZIP");
    items.push(item(
        "vitrin_index_product",
        idx_ok,
        "Ana vitrin Lertiva Zip (winlrtZIP yok)",
    ));

    let samples = [
        "lertiva-zip-store-gate.json",
        "lertiva-zip-lerta-integration-v1.json",
        "lertiva-zip-p0-gate.json",
    ];
    for name in samples {
        let p = workspace.join("apps/web/public/benchmark/samples").join(name);
        items.push(item(
            &format!("vitrin_sample_{name}"),
            p.is_file(),
            "Benchmark örneği mevcut",
        ));
    }

    let bench_idx = workspace.join("apps/web/public/benchmark/index.html");
    let bench_ok = bench_idx.is_file() && file_contains(&bench_idx, "lertiva-zip-store-gate.json");
    items.push(item(
        "vitrin_benchmark_index",
        bench_ok,
        "Benchmark sayfası lertiva-zip store gate referansı",
    ));

    let pass = items.iter().all(|i| i.pass);
    (pass, items)
}

/// W27 mağaza + W26 mobil + W31 Lerta staging + vitrin marka (repo dosyaları).
pub fn evaluate_store_pilot_gate(workspace: &Path) -> Result<StorePilotGateReport> {
    let store = store_gate::evaluate_store_gate(workspace)?;
    let store_gate_pass = store.all_pass;

    let lerta = lerta_integration_status_from_env();
    let lerta_staging_pass = lerta.staging_gate_pass && lerta.metadata_only;

    let caps = mobile_reader::capabilities();
    let mobile_reader_ready = !caps.create_archive && caps.lerta_share_intent_ready;

    let (vitrin_brand_pass, mut vitrin_items) = vitrin_brand_checks(workspace);

    let mut items = vec![
        item(
            "store_gate_w27",
            store_gate_pass,
            "Mağaza ön koşul checklist (W27)",
        ),
        item(
            "lerta_staging_w31",
            lerta_staging_pass,
            "Lerta birleşik staging kapısı (W31)",
        ),
        item(
            "mobile_reader_w26",
            mobile_reader_ready,
            "Mobil okuyucu salt-okuma + Lerta intent (W26)",
        ),
        item(
            "vitrin_brand",
            vitrin_brand_pass,
            "Vitrin Lertiva dosya adları ve metin",
        ),
    ];
    items.append(&mut vitrin_items);

    let all_pass = store_gate_pass && lerta_staging_pass && mobile_reader_ready && vitrin_brand_pass;
    if !all_pass {
        // Non-fatal for JSON export; CLI may exit non-zero separately.
    }

    Ok(StorePilotGateReport {
        v: STORE_PILOT_GATE_VERSION,
        product: PRODUCT_ZIP.into(),
        all_pass,
        store_gate_pass,
        lerta_staging_pass,
        mobile_reader_ready,
        vitrin_brand_pass,
        items,
    })
}

pub fn evaluate_store_pilot_gate_or_err(workspace: &Path) -> Result<StorePilotGateReport> {
    let report = evaluate_store_pilot_gate(workspace)?;
    if !report.all_pass {
        return Err(ArchiveError::Zip(
            "magaza/mobil pilot kapisi: eksik maddeler var".into(),
        ));
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store_gate::default_workspace_root;

    #[test]
    fn store_pilot_gate_repo_smoke() {
        std::env::remove_var("LERTA_API_BEARER");
        std::env::set_var("WINLRT_LERTA_API_DRY_RUN", "1");
        let root = default_workspace_root();
        let report = evaluate_store_pilot_gate(&root).expect("pilot gate");
        assert!(report.store_gate_pass, "{:?}", report.items);
        assert!(report.lerta_staging_pass);
        assert!(report.mobile_reader_ready);
        assert!(report.vitrin_brand_pass);
        assert!(report.all_pass);
        std::env::remove_var("WINLRT_LERTA_API_DRY_RUN");
    }
}
