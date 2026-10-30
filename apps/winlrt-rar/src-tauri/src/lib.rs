use std::path::PathBuf;

use serde::Serialize;
use winlrt_archive::{rar, wlrt, zip_io};

#[derive(Serialize)]
pub struct ListResult {
    pub entries: Vec<String>,
    pub format: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct EngineInfo {
    pub product: &'static str,
    pub phase: &'static str,
    pub motor: &'static str,
}

#[tauri::command]
fn engine_info() -> EngineInfo {
    EngineInfo {
        product: "winlrtRAR",
        phase: "Faz 2 beta iskelet",
        motor: "winlrt-archive 0.1.0",
    }
}

#[tauri::command]
fn list_archive(path: String, format: String) -> Result<ListResult, String> {
    let archive = PathBuf::from(&path);
    if !archive.is_file() {
        return Err("Dosya bulunamadı".into());
    }
    let entries = match format.as_str() {
        "zip" => zip_io::list(&archive).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::list(&archive).map_err(|e| e.to_string())?,
        "rar" => rar::list(&archive).map_err(|e| e.to_string())?,
        _ => return Err("Desteklenmeyen format (zip | wlrt | rar)".into()),
    };
    Ok(ListResult {
        entries,
        format,
        path,
    })
}

#[tauri::command]
fn test_archive(path: String, format: String) -> Result<(), String> {
    let archive = PathBuf::from(&path);
    match format.as_str() {
        "zip" => zip_io::test(&archive).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::test(&archive).map_err(|e| e.to_string())?,
        "rar" => rar::test(&archive).map_err(|e| e.to_string())?,
        _ => return Err("Desteklenmeyen format".into()),
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            engine_info,
            list_archive,
            test_archive
        ])
        .run(tauri::generate_context!())
        .expect("winlrtRAR başlatılamadı");
}
