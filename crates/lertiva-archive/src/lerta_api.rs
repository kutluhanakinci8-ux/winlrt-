//! Lerta üretim API bağlantısı (Faz 8, W29) — yalnızca metadata; ZIP içeriği cihazda kalır.

use serde::{Deserialize, Serialize};

use crate::audit;
use crate::lerta_mail::LertaMailWebhookPayload;
use crate::lerta_message::LertaMessageWebhookPayload;
use crate::{ArchiveError, Result};

pub const LERTA_API_CONTRACT_VERSION: &str = "1";
pub const LERTA_PRODUCTION_HOST: &str = "lerta.com.tr";
pub const LERTA_DEFAULT_API_BASE: &str = "https://u88.lerta.com.tr/api/v1";
pub const LERTA_MAIL_REGISTER_PATH: &str = "/integrations/winlrt-zip/mail-attachment";
pub const LERTA_MESSAGE_REGISTER_PATH: &str = "/integrations/winlrt-zip/message-attachment";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaApiConfig {
    pub contract_version: String,
    pub product: String,
    pub api_base: String,
    pub register_url: String,
    pub bearer_configured: bool,
    pub dry_run: bool,
    pub production_host: String,
    pub metadata_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaApiSubmitResult {
    pub ok: bool,
    pub dry_run: bool,
    pub http_status: Option<u16>,
    pub register_url: String,
    pub detail: String,
}

fn trim_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn normalize_base(base: &str) -> String {
    base.trim_end_matches('/').to_string()
}

pub fn lerta_api_config_from_env() -> LertaApiConfig {
    let api_base = trim_env("LERTA_API_BASE").unwrap_or_else(|| LERTA_DEFAULT_API_BASE.into());
    let base = normalize_base(&api_base);
    let register_url = format!("{base}{LERTA_MAIL_REGISTER_PATH}");
    let bearer_configured = trim_env("LERTA_API_BEARER").is_some();
    let dry_run = lerta_api_dry_run_forced() || !bearer_configured;
    LertaApiConfig {
        contract_version: LERTA_API_CONTRACT_VERSION.into(),
        product: "Lertiva Zip".into(),
        api_base: base,
        register_url,
        bearer_configured,
        dry_run,
        production_host: LERTA_PRODUCTION_HOST.into(),
        metadata_only: true,
    }
}

pub fn lerta_api_dry_run_forced() -> bool {
    if trim_env("WINLRT_LERTA_API_DRY_RUN")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return true;
    }
    trim_env("CI").is_some()
}

/// W28 webhook gövdesini Lerta API'ye bildirir (içerik yüklenmez).
pub fn submit_mail_attachment_metadata(
    payload: &LertaMailWebhookPayload,
) -> Result<LertaApiSubmitResult> {
    let cfg = lerta_api_config_from_env();
    if cfg.dry_run {
        audit::event_lerta_api("zip", true, "dry_run_submit");
        return Ok(LertaApiSubmitResult {
            ok: true,
            dry_run: true,
            http_status: None,
            register_url: cfg.register_url,
            detail: "metadata_only_dry_run".into(),
        });
    }
    let bearer = trim_env("LERTA_API_BEARER").ok_or_else(|| {
        ArchiveError::Zip("LERTA_API_BEARER gerekli (canli gonderim)".into())
    })?;
    let body = serde_json::to_string(payload).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let resp = ureq::post(&cfg.register_url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {bearer}"))
        .send_string(&body)
        .map_err(|e| ArchiveError::Zip(format!("Lerta API: {e}")))?;
    let status = resp.status();
    let ok = (200..300).contains(&status);
    audit::event_lerta_api(
        "zip",
        ok,
        &format!("submit_http_{status}"),
    );
    Ok(LertaApiSubmitResult {
        ok,
        dry_run: false,
        http_status: Some(status),
        register_url: cfg.register_url,
        detail: if ok {
            "submitted".into()
        } else {
            format!("http_{status}")
        },
    })
}

pub fn message_register_url_from_config(cfg: &LertaApiConfig) -> String {
    format!(
        "{}{LERTA_MESSAGE_REGISTER_PATH}",
        cfg.api_base.trim_end_matches('/')
    )
}

/// W30 mesaj eki metadata (içerik yüklenmez).
pub fn submit_message_attachment_metadata(
    payload: &LertaMessageWebhookPayload,
) -> Result<LertaApiSubmitResult> {
    let cfg = lerta_api_config_from_env();
    let register_url = message_register_url_from_config(&cfg);
    if cfg.dry_run {
        audit::event_lerta_api("zip", true, "dry_run_message_submit");
        return Ok(LertaApiSubmitResult {
            ok: true,
            dry_run: true,
            http_status: None,
            register_url,
            detail: "message_metadata_dry_run".into(),
        });
    }
    let bearer = trim_env("LERTA_API_BEARER").ok_or_else(|| {
        ArchiveError::Zip("LERTA_API_BEARER gerekli (canli gonderim)".into())
    })?;
    let body = serde_json::to_string(payload).map_err(|e| ArchiveError::Zip(e.to_string()))?;
    let resp = ureq::post(&register_url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {bearer}"))
        .send_string(&body)
        .map_err(|e| ArchiveError::Zip(format!("Lerta API: {e}")))?;
    let status = resp.status();
    let ok = (200..300).contains(&status);
    audit::event_lerta_api(
        "zip",
        ok,
        &format!("message_submit_http_{status}"),
    );
    Ok(LertaApiSubmitResult {
        ok,
        dry_run: false,
        http_status: Some(status),
        register_url,
        detail: if ok { "submitted".into() } else { format!("http_{status}") },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lerta_mail::{LertaAttachmentPrepareResult, LertaAttachmentTier, webhook_payload_from_prepare};
    use crate::lerta_message::{LertaMessageFormat, LertaMessagePrepareResult, webhook_payload_from_prepare as message_webhook};

    fn sample_payload() -> LertaMailWebhookPayload {
        let prep = LertaAttachmentPrepareResult {
            contract_version: "1".into(),
            product: "Lertiva Zip".into(),
            tier: LertaAttachmentTier::Standard10Mb,
            zip_path: "/tmp/ek.zip".into(),
            filename: "ek.zip".into(),
            size_bytes: 42,
            sha256_hex: "abc".into(),
            mime: "application/zip".into(),
            within_limit: true,
            suggested_attachment_name: "ek.zip".into(),
        };
        webhook_payload_from_prepare(&prep)
    }

    #[test]
    fn config_defaults_to_production_base() {
        let cfg = lerta_api_config_from_env();
        assert!(cfg.register_url.contains(LERTA_MAIL_REGISTER_PATH));
        assert!(cfg.metadata_only);
    }

    #[test]
    fn submit_dry_run_without_bearer() {
        std::env::remove_var("LERTA_API_BEARER");
        std::env::set_var("WINLRT_LERTA_API_DRY_RUN", "1");
        let r = submit_mail_attachment_metadata(&sample_payload()).unwrap();
        assert!(r.dry_run);
        assert!(r.ok);
        std::env::remove_var("WINLRT_LERTA_API_DRY_RUN");
    }

    fn sample_message_payload() -> LertaMessageWebhookPayload {
        let prep = LertaMessagePrepareResult {
            contract_version: "1".into(),
            product: "Lertiva Zip".into(),
            format: LertaMessageFormat::WlrtV0,
            attachment_path: "/tmp/m.wlrt".into(),
            filename: "m.wlrt".into(),
            size_bytes: 64,
            sha256_hex: "def".into(),
            mime: "application/vnd.winlrt.wlrt".into(),
            within_limit: true,
            kvkk_local_only: true,
            content_stays_on_device: true,
        };
        message_webhook(&prep)
    }

    #[test]
    fn message_submit_dry_run_without_bearer() {
        std::env::remove_var("LERTA_API_BEARER");
        std::env::set_var("WINLRT_LERTA_API_DRY_RUN", "1");
        let r = submit_message_attachment_metadata(&sample_message_payload()).unwrap();
        assert!(r.dry_run);
        assert!(r.ok);
        assert!(r.register_url.contains("message-attachment"));
        std::env::remove_var("WINLRT_LERTA_API_DRY_RUN");
    }
}
