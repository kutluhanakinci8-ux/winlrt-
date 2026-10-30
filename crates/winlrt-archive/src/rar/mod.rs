mod cli;
mod crate_backend;

#[cfg(feature = "unrar-embedded")]
mod embedded;

use std::path::Path;

use serde::Serialize;

use crate::Result;

#[derive(Debug, Clone, Serialize)]
pub struct BackendInfo {
    pub order: Vec<&'static str>,
    pub embedded_libunrar: bool,
    pub sidecar_unrar: bool,
}

pub fn backend_info() -> BackendInfo {
    BackendInfo {
        order: dispatch_order(),
        embedded_libunrar: cfg!(feature = "unrar-embedded"),
        sidecar_unrar: cli::sidecar_unrar_available(),
    }
}

fn dispatch_order() -> Vec<&'static str> {
    let mut order = Vec::new();
    if cfg!(feature = "unrar-embedded") {
        order.push("libunrar (gömülü)");
    }
    order.push("UnRAR CLI (sistem / yan dosya)");
    order.push("rar crate (RAR5)");
    order
}

/// RAR / RAR4 / RAR5 okuma — gömülü UnRAR, CLI, `rar` crate sırası.
pub fn list(archive: &Path, password: Option<&str>) -> Result<Vec<String>> {
    try_backends(archive, password, BackendOp::List)
}

pub fn extract(archive: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
    try_backends(archive, password, BackendOp::Extract(dest)).map(|_| ())
}

pub fn test(archive: &Path, password: Option<&str>) -> Result<()> {
    try_backends(archive, password, BackendOp::Test).map(|_| ())
}

pub(crate) enum BackendOp<'a> {
    List,
    Extract(&'a Path),
    Test,
}

fn try_backends(archive: &Path, password: Option<&str>, op: BackendOp<'_>) -> Result<Vec<String>> {
    let mut errors: Vec<String> = Vec::new();

    #[cfg(feature = "unrar-embedded")]
    {
        match run_embedded(archive, password, &op) {
            Ok(v) => return Ok(v),
            Err(e) => errors.push(format!("libunrar: {e}")),
        }
    }

    match cli::run(archive, password, &op) {
        Ok(v) => return Ok(v),
        Err(e) => errors.push(format!("CLI: {e}")),
    }

    match crate_backend::run(archive, password, &op) {
        Ok(v) => return Ok(v),
        Err(e) => errors.push(format!("rar crate: {e}")),
    }

    Err(crate::ArchiveError::Rar(format!(
        "RAR açılamadı. {}",
        tr_summary(&errors)
    )))
}

#[cfg(feature = "unrar-embedded")]
fn run_embedded(archive: &Path, password: Option<&str>, op: &BackendOp<'_>) -> Result<Vec<String>> {
    embedded::run(archive, password, op)
}

fn tr_summary(errors: &[String]) -> String {
    if errors.is_empty() {
        return "Bilinmeyen hata".into();
    }
    if errors.iter().any(|e| e.contains("Parola") || e.contains("password")) {
        return "Parola hatalı veya gerekli.".into();
    }
    errors.join(" · ")
}

pub(crate) use BackendOp as Op;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../benchmarks/fixtures/rar/rar5-text.rar")
    }

    #[test]
    fn rar5_fixture_extract_list() {
        let archive = fixture();
        if !archive.is_file() {
            return;
        }
        let names = list(&archive, Some("test")).expect("list");
        assert!(names.iter().any(|n| n.contains("text.txt")));
        let dir = tempfile::tempdir().unwrap();
        extract(&archive, dir.path(), Some("test")).unwrap();
        assert!(dir.path().join("text.txt").is_file());
    }
}
