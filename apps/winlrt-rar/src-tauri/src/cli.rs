use std::path::{Path, PathBuf};

use winlrt_archive::{rar, sevenz, wlrt, zip_io};

use crate::ActionResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupMode {
    Normal,
    OpenArchive(String),
    ExtractHere(String),
}

pub fn is_archive_path(path: &str) -> bool {
    let lower = path.to_lowercase();
    ["zip", "wlrt", "rar", "7z"]
        .iter()
        .any(|ext| lower.ends_with(&format!(".{ext}")))
}

pub fn format_from_path(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    match ext.as_str() {
        "zip" => Some("zip"),
        "wlrt" => Some("wlrt"),
        "rar" => Some("rar"),
        "7z" => Some("7z"),
        _ => None,
    }
}

pub fn parse_startup_mode() -> StartupMode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--extract-here" {
            if let Some(next) = args.get(i + 1) {
                if is_archive_path(next) && Path::new(next).is_file() {
                    return StartupMode::ExtractHere(next.clone());
                }
            }
            if let Some(p) = args.iter().find(|a| is_archive_path(a) && Path::new(a.as_str()).is_file()) {
                return StartupMode::ExtractHere(p.clone());
            }
        }
    }
    for arg in &args {
        if is_archive_path(arg) && Path::new(arg).is_file() {
            return StartupMode::OpenArchive(arg.clone());
        }
    }
    StartupMode::Normal
}

pub fn extract_here(archive_path: &str) -> Result<ActionResult, String> {
    let archive = PathBuf::from(archive_path);
    if !archive.is_file() {
        return Err("Arşiv bulunamadı".into());
    }
    let dest = archive
        .parent()
        .ok_or_else(|| "Hedef klasör belirlenemedi".to_string())?
        .to_path_buf();
    let format =
        format_from_path(&archive).ok_or_else(|| "Desteklenmeyen uzantı".to_string())?;
    match format {
        "zip" => zip_io::extract(&archive, &dest).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::extract(&archive, &dest).map_err(|e| e.to_string())?,
        "rar" => rar::extract(&archive, &dest, None).map_err(|e| e.to_string())?,
        "7z" => sevenz::extract(&archive, &dest, None).map_err(|e| e.to_string())?,
        _ => return Err("Desteklenmeyen format".into()),
    }
    Ok(ActionResult {
        ok: true,
        message: format!(
            "Buraya açıldı: {} → {}",
            archive.file_name().unwrap_or_default().to_string_lossy(),
            dest.display()
        ),
    })
}
