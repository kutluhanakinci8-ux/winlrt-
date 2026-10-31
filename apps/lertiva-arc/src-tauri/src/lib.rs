mod cli;
mod password_dialog;

use std::sync::atomic::{AtomicBool, Ordering};

static NATIVE_SECURE_HANDLED: AtomicBool = AtomicBool::new(false);

use std::path::Path;

use serde::Serialize;
use tauri::{AppHandle, Emitter, RunEvent};
#[cfg(target_os = "windows")]
use tauri_plugin_opener::OpenerExt;
use lertiva_archive::profile::CompressionProfile;
use lertiva_archive::crypto::{self, PasswordFeedback};
use lertiva_archive::password_policy::{self, PasswordPolicy};
use lertiva_archive::{audit, rar, sevenz, volumes, wlrt, zip_io};

use cli::{
    compress_folder_quick, compress_selection_quick, extract_here, is_archive_path,
    parse_startup_mode, suggested_archive_path, suggested_selection_archive_path, StartupMode,
};

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
    pub rar: lertiva_archive::rar::BackendInfo,
}

#[derive(Serialize)]
pub struct ActionResult {
    pub ok: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct PlatformInfo {
    pub os: String,
    pub explorer_context_menu: &'static str,
    pub native_password_dialog: bool,
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

#[derive(Clone, Serialize)]
pub struct CompressFolderHint {
    pub folder: String,
    pub suggested_output: String,
    pub secure_create: bool,
}

#[derive(Clone, Serialize)]
pub struct CompressSelectionHint {
    pub sources: Vec<String>,
    pub suggested_output: String,
    pub secure_create: bool,
}

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
    if NATIVE_SECURE_HANDLED.load(Ordering::SeqCst) {
        return;
    }
    let _ = app.emit("secure-create-wizard", wizard);
}

fn run_secure_wlrt_create(
    folder: Option<&str>,
    sources: &[String],
    dest: &str,
    password: &str,
    channel: &str,
) -> Result<ActionResult, String> {
    let policy = password_policy::policy_for_secure_create();
    if !policy.passes(password) {
        audit::event_policy_reject(channel, policy.id);
        return Err("Parola politikası karşılanmıyor.".into());
    }
    let dest_path = std::path::PathBuf::from(dest);
    let profile = CompressionProfile::Balanced;
    let result = if !sources.is_empty() {
        let paths: Vec<std::path::PathBuf> =
            sources.iter().map(std::path::PathBuf::from).collect();
        wlrt::create_from_paths(&paths, &dest_path, profile, Some(password))
    } else if let Some(folder) = folder {
        wlrt::create(
            std::path::Path::new(folder),
            &dest_path,
            profile,
            Some(password),
        )
    } else {
        return Err("Sıkıştırma kaynağı yok.".into());
    };
    match result {
        Ok(()) => {
            audit::event_create(channel, true, dest, "wlrt", Some(policy.id));
            Ok(ActionResult {
                ok: true,
                message: format!("Şifreli arşiv oluşturuldu: {}", dest_path.display()),
            })
        }
        Err(e) => {
            audit::event_create(channel, false, dest, "wlrt", Some(policy.id));
            Err(e.to_string())
        }
    }
}

fn try_native_headless_secure(app: &AppHandle) -> bool {
    if !cli_secure_create() {
        return false;
    }
    let (folder, sources, dest) = match parse_startup_mode() {
        StartupMode::CompressFolder(folder) => {
            let dest = match suggested_archive_path(&folder, "wlrt") {
                Ok(d) => d,
                Err(_) => return false,
            };
            (Some(folder), Vec::<String>::new(), dest)
        }
        StartupMode::CompressSelection(sources) => {
            let dest = match suggested_selection_archive_path(&sources, "wlrt") {
                Ok(d) => d,
                Err(_) => return false,
            };
            (None, sources, dest)
        }
        _ => return false,
    };

    let title = "Lertiva Arc — arşiv parolası";
    let message = "Parolayı girin (kurumsal politika geçerli):";
    match password_dialog::prompt_archive_password(title, message) {
        password_dialog::NativePromptOutcome::Accepted(password) => {
            audit::event_secure_prompt("explorer_native", "accepted");
            match run_secure_wlrt_create(
                folder.as_deref(),
                &sources,
                &dest,
                &password,
                "explorer_native",
            ) {
                Ok(res) => {
                    NATIVE_SECURE_HANDLED.store(true, Ordering::SeqCst);
                    let _ = app.emit("cli-action-done", res.message);
                    true
                }
                Err(err) => {
                    let _ = app.emit("cli-action-error", err);
                    NATIVE_SECURE_HANDLED.store(true, Ordering::SeqCst);
                    true
                }
            }
        }
        password_dialog::NativePromptOutcome::Cancelled => {
            audit::event_secure_prompt("explorer_native", "cancelled");
            false
        }
        password_dialog::NativePromptOutcome::Unavailable => false,
    }
}

fn emit_startup_events(app: &AppHandle) {
    match parse_startup_mode() {
        StartupMode::OpenArchive(path) => {
            let _ = app.emit("open-archive", path);
        }
        StartupMode::CompressFolder(folder) => {
            if let Ok(suggested) = suggested_archive_path(&folder, "wlrt") {
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
            if let Ok(suggested) = suggested_selection_archive_path(&sources, "wlrt") {
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
        | StartupMode::CompressSelectionQuick(_) => {}
        StartupMode::Normal => {
            for arg in std::env::args().skip(1) {
                if is_archive_path(&arg) && Path::new(&arg).is_file() {
                    let _ = app.emit("open-archive", arg);
                }
            }
        }
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
            let suggested_output = suggested_archive_path(&folder, "wlrt").ok()?;
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
            let suggested_output = suggested_selection_archive_path(&sources, "wlrt").ok()?;
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
fn engine_info() -> EngineInfo {
    EngineInfo {
        product: "Lertiva Arc",
        phase: "Faz 3 beta (geçiş)",
        motor: "lertiva-archive 0.1.0 (ZIP · wlrt · RAR4/5 · 7z)",
        rar: rar::backend_info(),
    }
}

#[tauri::command]
fn platform_info() -> PlatformInfo {
    PlatformInfo {
        os: std::env::consts::OS.into(),
        explorer_context_menu: if cfg!(target_os = "windows") {
            "NSIS: aç/buraya aç · sıkıştır/şifreli · çoklu seçim · Win32 parola"
        } else {
            "Yalnızca Windows kurulumunda"
        },
        native_password_dialog: cfg!(target_os = "windows"),
    }
}

#[tauri::command]
fn open_windows_default_apps(app: AppHandle) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        app.opener()
            .open_url("ms-settings:defaultapps", None::<&str>)
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = app;
        Err("Varsayılan uygulamalar yalnızca Windows'ta açılır.".into())
    }
}

#[tauri::command]
fn inspect_archive(path: String, format: String) -> Result<volumes::VolumeInfo, String> {
    let archive = std::path::PathBuf::from(&path);
    match format.as_str() {
        "rar" => Ok(rar::inspect(&archive)),
        _ => Ok(volumes::VolumeInfo {
            multipart: false,
            open_path: path.clone(),
            selected_path: path,
            format: "other",
            hint: "Çok cilt bilgisi şu an RAR için.".into(),
            can_repair: false,
        }),
    }
}

#[tauri::command]
fn repair_archive(
    path: String,
    dest: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    let archive = std::path::PathBuf::from(&path);
    let dest = std::path::PathBuf::from(&dest);
    rar::repair(&archive, &dest, password.as_deref()).map_err(|e| e.to_string())?;
    Ok(ActionResult {
        ok: true,
        message: format!("Onarım tamamlandı: {}", dest.display()),
    })
}

#[tauri::command]
fn list_archive(
    path: String,
    format: String,
    password: Option<String>,
) -> Result<ListResult, String> {
    let archive = std::path::PathBuf::from(&path);
    if !archive.is_file() {
        return Err("Dosya bulunamadı".into());
    }
    let pw = password.as_deref();
    let entries = match format.as_str() {
        "zip" => zip_io::list(&archive, pw).map_err(|e| e.to_string())?,
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
    let archive = std::path::PathBuf::from(&path);
    let pw = password.as_deref();
    match format.as_str() {
        "zip" => zip_io::test(&archive, pw).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::test(&archive, pw).map_err(|e| e.to_string())?,
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
    let archive = std::path::PathBuf::from(&path);
    let dest = std::path::PathBuf::from(&dest);
    let pw = password.as_deref();
    let extract_result = match format.as_str() {
        "zip" => zip_io::extract(&archive, &dest, pw).map_err(|e| e.to_string()),
        "wlrt" => wlrt::extract(&archive, &dest, pw).map_err(|e| e.to_string()),
        "rar" => rar::extract(&archive, &dest, pw).map_err(|e| e.to_string()),
        "7z" => sevenz::extract(&archive, &dest, pw).map_err(|e| e.to_string()),
        _ => return Err("Desteklenmeyen format".into()),
    };
    let archive_str = path.clone();
    match extract_result {
        Ok(()) => {
            audit::event_extract("ui", true, &archive_str, &format);
            Ok(ActionResult {
                ok: true,
                message: format!("Açıldı: {}", dest.display()),
            })
        }
        Err(e) => {
            audit::event_extract("ui", false, &archive_str, &format);
            Err(e)
        }
    }
}

#[tauri::command]
fn extract_one_entry(
    path: String,
    entry: String,
    dest_file: String,
    format: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    let archive = std::path::PathBuf::from(&path);
    let dest = std::path::PathBuf::from(&dest_file);
    let pw = password.as_deref();
    match format.as_str() {
        "wlrt" => wlrt::extract_one(&archive, &entry, &dest, pw).map_err(|e| e.to_string())?,
        _ => return Err("Tek dosya çıkarma şimdilik yalnızca .wlrt (G9 akışlı açma)".into()),
    }
    audit::event_extract("ui-one", true, &path, &format);
    Ok(ActionResult {
        ok: true,
        message: format!("Tek dosya çıkarıldı: {}", dest.display()),
    })
}

#[tauri::command]
fn evaluate_password(password: String, for_create: bool) -> PasswordFeedback {
    crypto::evaluate_password(&password, for_create)
}

#[tauri::command]
fn evaluate_create_password(password: String, secure_create: bool) -> PasswordFeedback {
    let policy = if secure_create {
        password_policy::policy_for_secure_create()
    } else {
        password_policy::active_policy()
    };
    let fb = crypto::evaluate_password_with_policy(&password, true, policy);
    if secure_create && !fb.ok_for_create {
        audit::event_policy_reject("ui_wizard", policy.id);
    }
    fb
}

#[tauri::command]
fn prompt_native_password(title: String, message: String) -> Option<String> {
    match password_dialog::prompt_archive_password(&title, &message) {
        password_dialog::NativePromptOutcome::Accepted(p) => {
            audit::event_secure_prompt("ui_native_retry", "accepted");
            Some(p)
        }
        password_dialog::NativePromptOutcome::Cancelled => {
            audit::event_secure_prompt("ui_native_retry", "cancelled");
            None
        }
        password_dialog::NativePromptOutcome::Unavailable => None,
    }
}

#[tauri::command]
fn audit_log_path() -> String {
    audit::audit_log_path().to_string_lossy().into_owned()
}

#[derive(Serialize)]
pub struct EnterprisePolicyInfo {
    pub source: String,
    pub policy_id: Option<String>,
    pub active_policy_id: String,
    pub audit_mirror: Option<String>,
    pub audit_forward_url: Option<String>,
    pub require_modern_crypto: bool,
}

#[tauri::command]
fn enterprise_policy_info() -> EnterprisePolicyInfo {
    use lertiva_archive::enterprise::{self, PolicySource};
    let (source, id) = enterprise::resolve_policy_source();
    let source = match source {
        PolicySource::Environment => "environment",
        PolicySource::Registry => "registry",
        PolicySource::Default => "default",
    };
    let active = password_policy::active_policy();
    EnterprisePolicyInfo {
        source: source.into(),
        policy_id: id,
        active_policy_id: active.id.into(),
        audit_mirror: enterprise::audit_mirror_path().map(|p| p.to_string_lossy().into_owned()),
        audit_forward_url: enterprise::audit_forward_url(),
        require_modern_crypto: enterprise::require_modern_crypto_enabled(),
    }
}

#[tauri::command]
fn password_policy_info(secure_create: bool) -> PasswordPolicyView {
    let policy = if secure_create {
        password_policy::policy_for_secure_create()
    } else {
        password_policy::active_policy()
    };
    policy.into()
}

#[tauri::command]
fn startup_secure_wizard() -> Option<SecureCreateWizard> {
    if !cli_secure_create() {
        return None;
    }
    match parse_startup_mode() {
        StartupMode::CompressFolder(folder) => {
            let suggested = suggested_archive_path(&folder, "wlrt").ok()?;
            Some(secure_wizard_payload(Some(folder), vec![], suggested))
        }
        StartupMode::CompressSelection(sources) => {
            let suggested = suggested_selection_archive_path(&sources, "wlrt").ok()?;
            Some(secure_wizard_payload(None, sources, suggested))
        }
        _ => None,
    }
}

#[tauri::command]
fn create_archive(
    source: String,
    dest: String,
    format: String,
    profile: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    let source = std::path::PathBuf::from(&source);
    let dest = std::path::PathBuf::from(&dest);
    let profile = parse_profile(&profile)?;
    let pw = password.as_deref();
    let create_result = match format.as_str() {
        "zip" => zip_io::create_with_profile(&source, &dest, profile, pw).map_err(|e| e.to_string()),
        "wlrt" => wlrt::create(&source, &dest, profile, pw).map_err(|e| e.to_string()),
        "rar" => return Err("RAR yazma desteklenmiyor".into()),
        "7z" => return Err("7z yazma henüz desteklenmiyor".into()),
        _ => return Err("Desteklenmeyen format".into()),
    };
    match create_result {
        Ok(()) => {
            audit::event_create(
                "ui",
                true,
                &dest.to_string_lossy(),
                &format,
                None,
            );
            Ok(ActionResult {
                ok: true,
                message: format!("Oluşturuldu: {}", dest.display()),
            })
        }
        Err(e) => {
            audit::event_create(
                "ui",
                false,
                &dest.to_string_lossy(),
                &format,
                None,
            );
            Err(e)
        }
    }
}

#[tauri::command]
fn create_archive_from_paths(
    sources: Vec<String>,
    dest: String,
    format: String,
    profile: String,
    password: Option<String>,
) -> Result<ActionResult, String> {
    if sources.is_empty() {
        return Err("Seçim boş".into());
    }
    let paths: Vec<std::path::PathBuf> = sources.iter().map(std::path::PathBuf::from).collect();
    let dest = std::path::PathBuf::from(&dest);
    let profile = parse_profile(&profile)?;
    let pw = password.as_deref();
    let create_result = match format.as_str() {
        "zip" => zip_io::create_from_paths(&paths, &dest, profile, pw).map_err(|e| e.to_string()),
        "wlrt" => wlrt::create_from_paths(&paths, &dest, profile, pw).map_err(|e| e.to_string()),
        "rar" => return Err("RAR yazma desteklenmiyor".into()),
        "7z" => return Err("7z yazma henüz desteklenmiyor".into()),
        _ => return Err("Desteklenmeyen format".into()),
    };
    match create_result {
        Ok(()) => {
            audit::event_create(
                "ui",
                true,
                &dest.to_string_lossy(),
                &format,
                None,
            );
            Ok(ActionResult {
                ok: true,
                message: format!("Oluşturuldu ({} öğe): {}", sources.len(), dest.display()),
            })
        }
        Err(e) => {
            audit::event_create(
                "ui",
                false,
                &dest.to_string_lossy(),
                &format,
                None,
            );
            Err(e)
        }
    }
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
                StartupMode::CompressSelectionQuick(paths) => {
                    match compress_selection_quick(&paths) {
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
            platform_info,
            open_windows_default_apps,
            startup_archives,
            startup_compress_folder,
            startup_compress_selection,
            evaluate_password,
            evaluate_create_password,
            password_policy_info,
            prompt_native_password,
            audit_log_path,
            enterprise_policy_info,
            startup_secure_wizard,
            inspect_archive,
            repair_archive,
            list_archive,
            test_archive,
            extract_archive,
            extract_one_entry,
            create_archive,
            create_archive_from_paths
        ])
        .build(tauri::generate_context!())
        .expect("Lertiva Arc build hatası")
        .run(|app, event| {
            if let RunEvent::Ready = event {
                let _ = try_native_headless_secure(app);
                emit_startup_events(app);
            }
        });
}
