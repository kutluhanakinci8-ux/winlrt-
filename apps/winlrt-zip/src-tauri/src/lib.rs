use std::path::Path;

use serde::Serialize;
use winlrt_archive::profile::CompressionProfile;
use winlrt_archive::zip_io;

#[derive(Serialize)]
pub struct EngineInfo {
    pub product: &'static str,
    pub version: &'static str,
    pub motor: &'static str,
}

#[derive(Serialize)]
pub struct ListResult {
    pub path: String,
    pub entries: Vec<String>,
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

fn map_err(e: winlrt_archive::ArchiveError) -> String {
    e.to_string()
}

#[tauri::command]
fn engine_info() -> EngineInfo {
    EngineInfo {
        product: "winlrtZIP",
        version: "0.1.0",
        motor: "winlrt-archive (zip_io)",
    }
}

#[tauri::command]
fn list_zip(archive: String, password: Option<String>) -> Result<ListResult, String> {
    let pw = password.as_deref();
    let entries = zip_io::list(Path::new(&archive), pw).map_err(map_err)?;
    Ok(ListResult {
        path: archive,
        entries,
    })
}

#[tauri::command]
fn test_zip(archive: String, password: Option<String>) -> Result<ActionResult, String> {
    zip_io::test(Path::new(&archive), password.as_deref()).map_err(map_err)?;
    Ok(ActionResult {
        ok: true,
        message: "ZIP test OK".into(),
    })
}

#[tauri::command]
fn extract_zip(
    archive: String,
    dest: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    zip_io::extract(
        Path::new(&archive),
        Path::new(&dest),
        password.as_deref(),
    )
    .map_err(map_err)?;
    Ok(ActionResult {
        ok: true,
        message: format!("Çıkarıldı → {dest}"),
    })
}

#[tauri::command]
fn create_zip(
    source: String,
    dest: String,
    profile: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    let profile = parse_profile(&profile)?;
    zip_io::create_with_profile(
        Path::new(&source),
        Path::new(&dest),
        profile,
        password.as_deref(),
    )
    .map_err(map_err)?;
    Ok(ActionResult {
        ok: true,
        message: format!("Oluşturuldu → {dest}"),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            engine_info,
            list_zip,
            test_zip,
            extract_zip,
            create_zip
        ])
        .run(tauri::generate_context!())
        .expect("winlrtZIP build hatası");
}
