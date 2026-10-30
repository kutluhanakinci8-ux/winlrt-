//! Lerta birleşik entegrasyon durumu (Faz 8, W31) — posta + mesaj kanalları, canlı API kapısı.

use serde::{Deserialize, Serialize};

use crate::lerta_api::{
    lerta_api_config_from_env, lerta_api_dry_run_forced, message_register_url_from_config,
    LERTA_MAIL_REGISTER_PATH, LERTA_MESSAGE_REGISTER_PATH,
};
use crate::lerta_mail::{LERTA_EXTENDED_MAX_ZIP_BYTES, LERTA_MAIL_CONTRACT_VERSION};
use crate::lerta_message::{LERTA_MESSAGE_CONTRACT_VERSION, LERTA_MESSAGE_MAX_BYTES};

pub const LERTA_INTEGRATION_CONTRACT_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaIntegrationChannel {
    pub id: String,
    pub madde: String,
    pub webhook_contract: String,
    pub register_path: String,
    pub register_url: String,
    pub max_bytes: u64,
    pub mime_types: Vec<String>,
    pub content_stays_on_device: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LertaIntegrationStatus {
    pub contract_version: String,
    pub product: String,
    pub api_base: String,
    pub production_host: String,
    pub bearer_configured: bool,
    pub dry_run: bool,
    pub metadata_only: bool,
    /// Canlı HTTP gönderimi (Bearer + dry-run kapalı).
    pub live_submit_allowed: bool,
    /// IT onayı öncesi repo/CI güvenli modu (dry-run veya CI).
    pub staging_safe_mode: bool,
    /// Her iki kanal URL’si tanımlı ve metadata-only.
    pub staging_gate_pass: bool,
    pub channels: Vec<LertaIntegrationChannel>,
}

pub fn lerta_integration_status_from_env() -> LertaIntegrationStatus {
    let cfg = lerta_api_config_from_env();
    let message_url = message_register_url_from_config(&cfg);
    let staging_safe = cfg.dry_run || lerta_api_dry_run_forced();
    let live = cfg.bearer_configured && !cfg.dry_run;
    let channels = vec![
        LertaIntegrationChannel {
            id: "mail".into(),
            madde: "W28".into(),
            webhook_contract: LERTA_MAIL_CONTRACT_VERSION.into(),
            register_path: LERTA_MAIL_REGISTER_PATH.into(),
            register_url: cfg.register_url.clone(),
            max_bytes: LERTA_EXTENDED_MAX_ZIP_BYTES,
            mime_types: vec!["application/zip".into()],
            content_stays_on_device: true,
        },
        LertaIntegrationChannel {
            id: "message".into(),
            madde: "W30".into(),
            webhook_contract: LERTA_MESSAGE_CONTRACT_VERSION.into(),
            register_path: LERTA_MESSAGE_REGISTER_PATH.into(),
            register_url: message_url,
            max_bytes: LERTA_MESSAGE_MAX_BYTES,
            mime_types: vec![
                "application/vnd.winlrt.wlrt".into(),
                "application/zip".into(),
            ],
            content_stays_on_device: true,
        },
    ];
    let staging_gate_pass = cfg.metadata_only
        && channels.iter().all(|c| c.register_url.contains("lertiva-zip"));
    LertaIntegrationStatus {
        contract_version: LERTA_INTEGRATION_CONTRACT_VERSION.into(),
        product: cfg.product,
        api_base: cfg.api_base,
        production_host: cfg.production_host,
        bearer_configured: cfg.bearer_configured,
        dry_run: cfg.dry_run,
        metadata_only: cfg.metadata_only,
        live_submit_allowed: live,
        staging_safe_mode: staging_safe,
        staging_gate_pass,
        channels,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_status_lists_both_channels() {
        std::env::remove_var("LERTA_API_BEARER");
        std::env::set_var("WINLRT_LERTA_API_DRY_RUN", "1");
        let st = lerta_integration_status_from_env();
        assert_eq!(st.channels.len(), 2);
        assert!(st.staging_gate_pass);
        assert!(!st.live_submit_allowed);
        assert!(st.staging_safe_mode);
        assert!(st.channels[0].register_url.contains("mail-attachment"));
        assert!(st.channels[1].register_url.contains("message-attachment"));
        std::env::remove_var("WINLRT_LERTA_API_DRY_RUN");
    }
}
