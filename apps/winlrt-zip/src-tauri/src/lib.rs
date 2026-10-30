mod cli;

use std::path::{Path, PathBuf};

use cli::{
    compress_folder_quick, extract_here, parse_startup_mode, suggested_archive_path,
    suggested_selection_archive_path, StartupMode,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, RunEvent};
use winlrt_archive::profile::CompressionProfile;
use winlrt_archive::zip_io;

#[derive(Serialize)]
pub struct EngineInfo {
    pub product: &'static str,
    pub version: &'static str,
    pub motor: &'static str,
    pub explorer_context_menu: &'static str,
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

#[derive(Serialize, Clone)]
pub struct CompressFolderHint {
    pub folder: String,
    pub suggested_output: String,
}

#[derive(Serialize, Clone)]
pub struct CompressSelectionHint {
    pub sources: Vec<String>,
    pub suggested_output: String,
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

fn emit_startup_events(app: &AppHandle) {
    match parse_startup_mode() {
        StartupMode::OpenArchive(path) => {
            let _ = app.emit("open-archive", path);
        }
        StartupMode::CompressFolder(folder) => {
            if let Ok(suggested) = suggested_archive_path(&folder) {
                let _ = app.emit(
                    "compress-folder",
                    CompressFolderHint {
                        folder: folder.clone(),
                        suggested_output: suggested,
                    },
                );
            }
        }
        StartupMode::CompressSelection(sources) => {
            if let Ok(suggested) = suggested_selection_archive_path(&sources) {
                let _ = app.emit(
                    "compress-selection",
                    CompressSelectionHint {
                        sources: sources.clone(),
                        suggested_output: suggested,
                    },
                );
            }
        }
        StartupMode::ExtractHere(_)
        | StartupMode::CompressFolderQuick(_)
        | StartupMode::Normal => {}
    }
}

#[tauri::command]
fn engine_info() -> EngineInfo {
    EngineInfo {
        product: "winlrtZIP",
        version: "0.1.0",
        motor: "winlrt-archive (zip_io)",
        explorer_context_menu: if cfg!(target_os = "windows") {
            "NSIS: ZIP ac/buraya ac · klasor/secim sikistir (ZIP)"
        } else {
            "Explorer yalnizca Windows NSIS kurulumunda"
        },
    }
}

#[tauri::command]
fn startup_archives() -> Vec<String> {
    match parse_startup_mode() {
        StartupMode::OpenArchive(path) => vec![path],
        _ => vec![],
    }
}

#[tauri::command]
fn startup_compress_folder() -> Option<CompressFolderHint> {
    match parse_startup_mode() {
        StartupMode::CompressFolder(folder) => {
            let suggested_output = suggested_archive_path(&folder).ok()?;
            Some(CompressFolderHint {
                folder,
                suggested_output,
            })
        }
        _ => None,
    }
}

#[tauri::command]
fn startup_compress_selection() -> Option<CompressSelectionHint> {
    match parse_startup_mode() {
        StartupMode::CompressSelection(sources) => {
            let suggested_output = suggested_selection_archive_path(&sources).ok()?;
            Some(CompressSelectionHint {
                sources,
                suggested_output,
            })
        }
        _ => None,
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

#[tauri::command]
fn create_zip_from_paths(
    sources: Vec<String>,
    dest: String,
    profile: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    let profile = parse_profile(&profile)?;
    let paths: Vec<PathBuf> = sources.iter().map(PathBuf::from).collect();
    zip_io::create_from_paths(&paths, Path::new(&dest), profile, password.as_deref())
        .map_err(map_err)?;
    Ok(ActionResult {
        ok: true,
        message: format!("Oluşturuldu → {dest} ({})", sources.len()),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            match parse_startup_mode() {
                StartupMode::ExtractHere(path) => match extract_here(&path) {
                    Ok(res) => {
                        let _ = handle.emit("cli-action-done", res.message);
                    }
                    Err(err) => {
                        let _ = handle.emit("cli-action-error", err);
                    }
                },
                StartupMode::CompressFolderQuick(folder) => {
                    match compress_folder_quick(&folder) {
                        Ok(res) => {
                            let _ = handle.emit("cli-action-done", res.message);
                        }
                        Err(err) => {
                            let _ = handle.emit("cli-action-error", err);
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            engine_info,
            startup_archives,
            startup_compress_folder,
            startup_compress_selection,
            list_zip,
            test_zip,
            extract_zip,
            create_zip,
            create_zip_from_paths
        ])
        .build(tauri::generate_context!())
        .expect("winlrtZIP build hatası")
        .run(|app, event| {
            if let RunEvent::Ready = event {
                emit_startup_events(app);
            }
        });
}
