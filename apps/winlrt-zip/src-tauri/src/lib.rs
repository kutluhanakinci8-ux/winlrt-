mod cli;
mod migration;

use std::path::{Path, PathBuf};

use cli::{
    compress_folder_quick, extract_here, parse_startup_mode, suggested_archive_path,
    suggested_selection_archive_path, StartupMode,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, RunEvent};
use winlrt_archive::audit;
use winlrt_archive::crypto::{self, PasswordFeedback};
use winlrt_archive::enterprise;
use winlrt_archive::password_policy::{self, PasswordPolicy};
use winlrt_archive::profile::CompressionProfile;
use winlrt_archive::zip_io;

fn cli_secure_create() -> bool {
    std::env::args().any(|a| a == "--secure")
}

#[derive(Clone, Serialize)]
pub struct PasswordPolicyView {
    pub id: String,
    pub label: String,
    pub min_length: u32,
    pub require_upper: bool,
    pub require_lower: bool,
    pub require_digit: bool,
    pub require_symbol: bool,
}

impl From<PasswordPolicy> for PasswordPolicyView {
    fn from(p: PasswordPolicy) -> Self {
        Self {
            id: p.id.into(),
            label: p.label.into(),
            min_length: p.min_length,
            require_upper: p.require_upper,
            require_lower: p.require_lower,
            require_digit: p.require_digit,
            require_symbol: p.require_symbol,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SecureCreateWizard {
    pub suggested_output: String,
    pub folder: Option<String>,
    pub sources: Vec<String>,
    pub policy: PasswordPolicyView,
}

fn secure_wizard_payload(
    folder: Option<String>,
    sources: Vec<String>,
    suggested_output: String,
) -> SecureCreateWizard {
    SecureCreateWizard {
        suggested_output,
        folder,
        sources,
        policy: password_policy::policy_for_secure_create().into(),
    }
}

fn maybe_emit_secure_wizard(app: &AppHandle, wizard: SecureCreateWizard) {
    let _ = app.emit("secure-create-wizard", wizard);
}

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
    pub secure_create: bool,
}

#[derive(Serialize, Clone)]
pub struct CompressSelectionHint {
    pub sources: Vec<String>,
    pub suggested_output: String,
    pub secure_create: bool,
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
                let secure = cli_secure_create();
                let _ = app.emit(
                    "compress-folder",
                    CompressFolderHint {
                        folder: folder.clone(),
                        suggested_output: suggested.clone(),
                        secure_create: secure,
                    },
                );
                if secure {
                    maybe_emit_secure_wizard(
                        app,
                        secure_wizard_payload(Some(folder), vec![], suggested),
                    );
                }
            }
        }
        StartupMode::CompressSelection(sources) => {
            if let Ok(suggested) = suggested_selection_archive_path(&sources) {
                let secure = cli_secure_create();
                let _ = app.emit(
                    "compress-selection",
                    CompressSelectionHint {
                        sources: sources.clone(),
                        suggested_output: suggested.clone(),
                        secure_create: secure,
                    },
                );
                if secure {
                    maybe_emit_secure_wizard(
                        app,
                        secure_wizard_payload(None, sources, suggested),
                    );
                }
            }
        }
        StartupMode::ExtractHere(_)
        | StartupMode::CompressFolderQuick(_)
        | StartupMode::Normal => {}
    }
}

#[tauri::command]
fn winzip_migration_guide() -> migration::WinzipMigrationGuide {
    migration::winzip_migration_guide()
}

#[tauri::command]
fn engine_info() -> EngineInfo {
    EngineInfo {
        product: "winlrtZIP",
        version: "0.1.0",
        motor: "winlrt-archive (zip_io)",
        explorer_context_menu: if cfg!(target_os = "windows") {
            "NSIS: ZIP ac/buraya ac · sikistir · sifreli sikistir (--secure)"
        } else if cfg!(target_os = "macos") {
            "macOS: .zip cift tik / Surukle-birak · imza/notarize W14 beta CI"
        } else {
            "Kabuk entegrasyonu: Windows NSIS · macOS dmg (W14)"
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
                secure_create: cli_secure_create(),
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
                secure_create: cli_secure_create(),
            })
        }
        _ => None,
    }
}

#[tauri::command]
fn evaluate_create_password(password: String, secure_create: bool) -> PasswordFeedback {
    let policy = if secure_create {
        password_policy::policy_for_secure_create()
    } else {
        password_policy::active_policy()
    };
    crypto::evaluate_password_with_policy(&password, true, policy)
}

#[tauri::command]
fn startup_secure_wizard() -> Option<SecureCreateWizard> {
    if !cli_secure_create() {
        return None;
    }
    match parse_startup_mode() {
        StartupMode::CompressFolder(folder) => {
            let suggested = suggested_archive_path(&folder).ok()?;
            Some(secure_wizard_payload(Some(folder), vec![], suggested))
        }
        StartupMode::CompressSelection(sources) => {
            let suggested = suggested_selection_archive_path(&sources).ok()?;
            Some(secure_wizard_payload(None, sources, suggested))
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
    match zip_io::extract(
        Path::new(&archive),
        Path::new(&dest),
        password.as_deref(),
    ) {
        Ok(()) => {
            audit::event_extract("ui", true, &archive, "zip");
            Ok(ActionResult {
                ok: true,
                message: format!("Çıkarıldı → {dest}"),
            })
        }
        Err(e) => {
            audit::event_extract("ui", false, &archive, "zip");
            Err(map_err(e))
        }
    }
}

#[tauri::command]
fn create_zip(
    source: String,
    dest: String,
    profile: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    let profile = parse_profile(&profile)?;
    let policy = password_policy::active_policy();
    match zip_io::create_with_profile(
        Path::new(&source),
        Path::new(&dest),
        profile,
        password.as_deref(),
    ) {
        Ok(()) => {
            audit::event_create("ui", true, &dest, "zip", Some(policy.id));
            Ok(ActionResult {
                ok: true,
                message: format!("Oluşturuldu → {dest}"),
            })
        }
        Err(e) => {
            audit::event_create("ui", false, &dest, "zip", Some(policy.id));
            Err(map_err(e))
        }
    }
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
    let policy = password_policy::active_policy();
    match zip_io::create_from_paths(&paths, Path::new(&dest), profile, password.as_deref()) {
        Ok(()) => {
            audit::event_create("ui", true, &dest, "zip", Some(policy.id));
            Ok(ActionResult {
                ok: true,
                message: format!("Oluşturuldu → {dest} ({})", sources.len()),
            })
        }
        Err(e) => {
            audit::event_create("ui", false, &dest, "zip", Some(policy.id));
            Err(map_err(e))
        }
    }
}

#[tauri::command]
fn audit_log_path_hint() -> String {
    audit::audit_log_path().to_string_lossy().into_owned()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    enterprise::init_registry_product("zip");
    audit::init_audit_product("zip");
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
            winzip_migration_guide,
            startup_archives,
            startup_compress_folder,
            startup_compress_selection,
            startup_secure_wizard,
            evaluate_create_password,
            list_zip,
            test_zip,
            extract_zip,
            create_zip,
            create_zip_from_paths,
            audit_log_path_hint
        ])
        .build(tauri::generate_context!())
        .expect("winlrtZIP build hatası")
        .run(|app, event| {
            if let RunEvent::Ready = event {
                emit_startup_events(app);
            }
        });
}

#[cfg(test)]
mod platform_info_tests {
    use super::engine_info;

    #[test]
    fn engine_info_platform_hint_non_empty() {
        let hint = engine_info().explorer_context_menu;
        assert!(!hint.is_empty());
        #[cfg(target_os = "macos")]
        assert!(hint.contains("macOS"));
        #[cfg(target_os = "windows")]
        assert!(hint.contains("NSIS"));
    }
}
