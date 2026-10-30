use std::path::{Path, PathBuf};

use serde::Serialize;
use tauri::{AppHandle, Emitter, RunEvent};
use winlrt_archive::profile::CompressionProfile;
use winlrt_archive::{rar, sevenz, wlrt, zip_io};

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

#[derive(Serialize)]
pub struct ActionResult {
    pub ok: bool,
    pub message: String,
}

fn parse_profile(profile: &str) -> Result<CompressionProfile, String> {
    match profile.to_lowercase().as_str() {
        "fast" | "hizli" => Ok(CompressionProfile::Fast),
        "balanced" | "dengeli" => Ok(CompressionProfile::Balanced),
        "maximum" | "maksimum" => Ok(CompressionProfile::Maximum),
        "email" | "eposta" => Ok(CompressionProfile::EmailAttachment),
        _ => Err("Profil: fast | balanced | maximum | email".into()),
    }
}

fn is_archive_path(path: &str) -> bool {
    let lower = path.to_lowercase();
    ["zip", "wlrt", "rar", "7z"]
        .iter()
        .any(|ext| lower.ends_with(&format!(".{ext}")))
}

fn emit_startup_archives(app: &AppHandle) {
    for arg in std::env::args().skip(1) {
        if is_archive_path(&arg) && Path::new(&arg).is_file() {
            let _ = app.emit("open-archive", arg);
        }
    }
}

#[tauri::command]
fn startup_archives() -> Vec<String> {
    std::env::args()
        .skip(1)
        .filter(|a| is_archive_path(a) && Path::new(a).is_file())
        .collect()
}

#[tauri::command]
fn engine_info() -> EngineInfo {
    EngineInfo {
        product: "winlrtRAR",
        phase: "Faz 2 beta",
        motor: "winlrt-archive 0.1.0 (ZIP · wlrt · RAR5 · 7z)",
    }
}

#[tauri::command]
fn list_archive(
    path: String,
    format: String,
    password: Option<String>,
) -> Result<ListResult, String> {
    let archive = PathBuf::from(&path);
    if !archive.is_file() {
        return Err("Dosya bulunamadı".into());
    }
    let pw = password.as_deref();
    let entries = match format.as_str() {
        "zip" => zip_io::list(&archive).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::list(&archive).map_err(|e| e.to_string())?,
        "rar" => rar::list(&archive, pw).map_err(|e| e.to_string())?,
        "7z" => sevenz::list(&archive, pw).map_err(|e| e.to_string())?,
        _ => return Err("Desteklenmeyen format (zip | wlrt | rar | 7z)".into()),
    };
    Ok(ListResult {
        entries,
        format,
        path,
    })
}

#[tauri::command]
fn test_archive(
    path: String,
    format: String,
    password: Option<String>,
) -> Result<(), String> {
    let archive = PathBuf::from(&path);
    let pw = password.as_deref();
    match format.as_str() {
        "zip" => zip_io::test(&archive).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::test(&archive).map_err(|e| e.to_string())?,
        "rar" => rar::test(&archive, pw).map_err(|e| e.to_string())?,
        "7z" => sevenz::test(&archive, pw).map_err(|e| e.to_string())?,
        _ => return Err("Desteklenmeyen format".into()),
    }
    Ok(())
}

#[tauri::command]
fn extract_archive(
    path: String,
    dest: String,
    format: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    let archive = PathBuf::from(&path);
    let dest = PathBuf::from(&dest);
    let pw = password.as_deref();
    match format.as_str() {
        "zip" => zip_io::extract(&archive, &dest).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::extract(&archive, &dest).map_err(|e| e.to_string())?,
        "rar" => rar::extract(&archive, &dest, pw).map_err(|e| e.to_string())?,
        "7z" => sevenz::extract(&archive, &dest, pw).map_err(|e| e.to_string())?,
        _ => return Err("Desteklenmeyen format".into()),
    }
    Ok(ActionResult {
        ok: true,
        message: format!("Açıldı: {}", dest.display()),
    })
}

#[tauri::command]
fn create_archive(
    source: String,
    dest: String,
    format: String,
    profile: String,
) -> Result<ActionResult, String> {
    let source = PathBuf::from(&source);
    let dest = PathBuf::from(&dest);
    let profile = parse_profile(&profile)?;
    match format.as_str() {
        "zip" => zip_io::create_with_profile(&source, &dest, profile).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::create(&source, &dest, profile).map_err(|e| e.to_string())?,
        "rar" => return Err("RAR yazma desteklenmiyor".into()),
        "7z" => return Err("7z yazma henüz desteklenmiyor".into()),
        _ => return Err("Desteklenmeyen format".into()),
    }
    Ok(ActionResult {
        ok: true,
        message: format!("Oluşturuldu: {}", dest.display()),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            engine_info,
            startup_archives,
            list_archive,
            test_archive,
            extract_archive,
            create_archive
        ])
        .build(tauri::generate_context!())
        .expect("winlrtRAR build hatası")
        .run(|app, event| {
            if let RunEvent::Ready = event {
                emit_startup_archives(app);
            }
        });
}
