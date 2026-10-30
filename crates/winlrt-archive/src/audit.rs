use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;

/// KVKK / kurumsal denetim — **parola ve anahtar metni asla loglanmaz**.
#[derive(Debug, Serialize)]
pub struct AuditRecord {
    pub ts_unix: u64,
    pub event: &'static str,
    pub channel: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

pub fn log(record: AuditRecord) {
    if std::env::var("WINLRT_AUDIT_DISABLE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return;
    }
    let Ok(line) = serde_json::to_string(&record) else {
        return;
    };
    let path = audit_log_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{line}");
    }
    mirror_audit_line(&line);
    forward_audit_line(&line);
}

fn forward_audit_line(line: &str) {
    let Some(url) = crate::enterprise::audit_forward_url() else {
        return;
    };
    let body = line.to_string();
    std::thread::spawn(move || {
        let _ = ureq::post(&url)
            .set("Content-Type", "application/x-ndjson")
            .send_string(&body);
    });
}

fn mirror_audit_line(line: &str) {
    let Some(mirror) = crate::enterprise::audit_mirror_path() else {
        return;
    };
    if let Some(parent) = mirror.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&mirror)
    {
        let _ = writeln!(f, "{line}");
    }
}

pub fn audit_log_path() -> PathBuf {
    if let Ok(p) = std::env::var("WINLRT_AUDIT_LOG") {
        if !p.trim().is_empty() {
            return PathBuf::from(p);
        }
    }
    #[cfg(windows)]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata)
                .join("winlrtRAR")
                .join("audit.jsonl");
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".local/share/winlrtRAR/audit.jsonl");
    }
    std::env::temp_dir().join("winlrtRAR-audit.jsonl")
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn event_create(channel: &str, ok: bool, dest: &str, format: &str, policy_id: Option<&str>) {
    log(AuditRecord {
        ts_unix: now_unix(),
        event: "archive_create",
        channel: channel.into(),
        ok,
        path_hint: Some(basename_hint(dest)),
        format: Some(format.into()),
        policy_id: policy_id.map(String::from),
        detail: None,
    });
}

pub fn event_extract(channel: &str, ok: bool, archive: &str, format: &str) {
    log(AuditRecord {
        ts_unix: now_unix(),
        event: "archive_extract",
        channel: channel.into(),
        ok,
        path_hint: Some(basename_hint(archive)),
        format: Some(format.into()),
        policy_id: None,
        detail: None,
    });
}

pub fn event_secure_prompt(channel: &str, outcome: &str) {
    log(AuditRecord {
        ts_unix: now_unix(),
        event: "secure_password_prompt",
        channel: channel.into(),
        ok: outcome == "ok" || outcome == "accepted",
        path_hint: None,
        format: None,
        policy_id: None,
        detail: Some(outcome.into()),
    });
}

pub fn event_policy_reject(channel: &str, policy_id: &str) {
    log(AuditRecord {
        ts_unix: now_unix(),
        event: "password_policy_reject",
        channel: channel.into(),
        ok: false,
        path_hint: None,
        format: None,
        policy_id: Some(policy_id.into()),
        detail: None,
    });
}

fn basename_hint(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("?")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_line_serializes_without_password_fields() {
        let json = serde_json::to_string(&AuditRecord {
            ts_unix: 1,
            event: "archive_create",
            channel: "test".into(),
            ok: true,
            path_hint: Some("demo.wlrt".into()),
            format: Some("wlrt".into()),
            policy_id: None,
            detail: None,
        })
        .unwrap();
        assert!(!json.contains("password"));
        assert!(json.contains("archive_create"));
    }
}
