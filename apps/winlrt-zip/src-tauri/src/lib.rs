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
use winlrt_archive::lerta_api;
use winlrt_archive::lerta_integration;
use winlrt_archive::lerta_mail;
use winlrt_archive::lerta_message;
use winlrt_archive::mobile_reader;
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
fn lerta_prepare_mail_attachment(
    source: String,
    dest: String,
    tier: String,
    password: Option<String>,
) -> Result<lerta_mail::LertaAttachmentPrepareResult, String> {
    let tier = parse_lerta_tier(&tier)?;
    lerta_mail::prepare_lerta_mail_attachment(
        Path::new(&source),
        Path::new(&dest),
        tier,
        password.as_deref(),
    )
    .map_err(map_err)
}

#[tauri::command]
fn lerta_validate_mail_attachment(
    zip: String,
    tier: String,
) -> Result<ActionResult, String> {
    let tier = parse_lerta_tier(&tier)?;
    lerta_mail::validate_lerta_mail_attachment(Path::new(&zip), tier).map_err(map_err)?;
    Ok(ActionResult {
        ok: true,
        message: "Lerta posta eki OK".into(),
    })
}

fn parse_lerta_tier(s: &str) -> Result<lerta_mail::LertaAttachmentTier, String> {
    match s {
        "standard10_mb" | "standard" => Ok(lerta_mail::LertaAttachmentTier::Standard10Mb),
        "extended25_mb" | "extended" => Ok(lerta_mail::LertaAttachmentTier::Extended25Mb),
        _ => Err(format!("bilinmeyen Lerta tier: {s}")),
    }
}

#[tauri::command]
fn lerta_api_status() -> lerta_api::LertaApiConfig {
    lerta_api::lerta_api_config_from_env()
}

#[tauri::command]
fn lerta_integration_status() -> lerta_integration::LertaIntegrationStatus {
    lerta_integration::lerta_integration_status_from_env()
}

#[tauri::command]
fn lerta_submit_mail_attachment(
    source: String,
    dest: String,
    tier: String,
    password: Option<String>,
) -> Result<lerta_api::LertaApiSubmitResult, String> {
    let tier = parse_lerta_tier(&tier)?;
    let prep = lerta_mail::prepare_lerta_mail_attachment(
        Path::new(&source),
        Path::new(&dest),
        tier,
        password.as_deref(),
    )
    .map_err(map_err)?;
    let hook = lerta_mail::webhook_payload_from_prepare(&prep);
    lerta_api::submit_mail_attachment_metadata(&hook).map_err(map_err)
}

fn parse_lerta_message_format(s: &str) -> Result<lerta_message::LertaMessageFormat, String> {
    match s {
        "wlrt_v0" | "wlrt" => Ok(lerta_message::LertaMessageFormat::WlrtV0),
        "aes_zip" | "aes" => Ok(lerta_message::LertaMessageFormat::AesZip),
        _ => Err(format!("bilinmeyen Lerta mesaj formati: {s}")),
    }
}

#[tauri::command]
fn lerta_prepare_message_attachment(
    source: String,
    dest: String,
    format: String,
    password: Option<String>,
) -> Result<lerta_message::LertaMessagePrepareResult, String> {
    let format = parse_lerta_message_format(&format)?;
    lerta_message::prepare_lerta_message_attachment(
        Path::new(&source),
        Path::new(&dest),
        format,
        password.as_deref(),
    )
    .map_err(map_err)
}

#[tauri::command]
fn lerta_validate_message_attachment(
    attachment: String,
    format: String,
) -> Result<ActionResult, String> {
    let format = parse_lerta_message_format(&format)?;
    lerta_message::validate_lerta_message_attachment(Path::new(&attachment), format)
        .map_err(map_err)?;
    Ok(ActionResult {
        ok: true,
        message: "Lerta mesaj eki OK".into(),
    })
}

#[tauri::command]
fn lerta_submit_message_attachment(
    source: String,
    dest: String,
    format: String,
    password: Option<String>,
) -> Result<lerta_api::LertaApiSubmitResult, String> {
    let format = parse_lerta_message_format(&format)?;
    let prep = lerta_message::prepare_lerta_message_attachment(
        Path::new(&source),
        Path::new(&dest),
        format,
        password.as_deref(),
    )
    .map_err(map_err)?;
    let hook = lerta_message::webhook_payload_from_prepare(&prep);
    lerta_api::submit_message_attachment_metadata(&hook).map_err(map_err)
}

#[tauri::command]
fn mobile_capabilities() -> mobile_reader::MobileCapabilities {
    mobile_reader::capabilities()
}

#[tauri::command]
fn mobile_inspect_zip(
    archive: String,
    password: Option<String>,
) -> Result<mobile_reader::MobileInspectReport, String> {
    mobile_reader::inspect(Path::new(&archive), password.as_deref()).map_err(map_err)
}

#[tauri::command]
fn mobile_lerta_share_intent(
    archive: String,
    download_url: Option<String>,
) -> Result<mobile_reader::LertaShareIntentPayload, String> {
    mobile_reader::lerta_share_intent_from_zip(
        Path::new(&archive),
        download_url.as_deref(),
    )
    .map_err(map_err)
}

#[tauri::command]
fn enterprise_policy_info() -> EnterprisePolicyInfo {
    use winlrt_archive::enterprise::{self, PolicySource};
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
            audit_log_path_hint,
            enterprise_policy_info,
            mobile_capabilities,
            mobile_inspect_zip,
            mobile_lerta_share_intent,
            lerta_prepare_mail_attachment,
            lerta_validate_mail_attachment,
            lerta_api_status,
            lerta_integration_status,
            lerta_submit_mail_attachment,
            lerta_prepare_message_attachment,
            lerta_validate_message_attachment,
            lerta_submit_message_attachment
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
