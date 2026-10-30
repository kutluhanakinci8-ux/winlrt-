use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VolumeInfo {
    pub multipart: bool,
    /// Motorun açması gereken yol (genelde ilk cilt).
    pub open_path: String,
    pub selected_path: String,
    pub format: &'static str,
    pub hint: String,
    pub can_repair: bool,
}

/// RAR çok ciltli isimlerinden ilk cilt yolunu türetir (dosya sistemi kontrolü ile).
pub fn resolve_rar(path: &Path) -> PathBuf {
    inspect_rar(path).resolved_path()
}

pub fn inspect_rar(path: &Path) -> VolumeInfo {
    let selected_str = path.display().to_string();
    let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
        return VolumeInfo {
            multipart: false,
            open_path: selected_str.clone(),
            selected_path: selected_str,
            format: "rar",
            hint: String::new(),
            can_repair: true,
        };
    };

    if let Some(first) = rar_first_part_name(file_name) {
        let parent = path.parent().unwrap_or(Path::new("."));
        let candidate = parent.join(&first);
        if candidate.is_file() {
            return VolumeInfo {
                multipart: true,
                open_path: candidate.display().to_string(),
                selected_path: selected_str,
                format: "rar",
                hint: format!(
                    "Çok ciltli RAR: «{}» — motor ilk cilt «{}» ile açıyor. Tüm ciltler aynı klasörde olmalı.",
                    file_name, first
                ),
                can_repair: true,
            };
        }
        return VolumeInfo {
            multipart: true,
            open_path: candidate.display().to_string(),
            selected_path: selected_str,
            format: "rar",
            hint: format!(
                "Çok ciltli RAR algılandı; ilk cilt bulunamadı: {} (aynı klasörde olmalı).",
                candidate.display()
            ),
            can_repair: true,
        };
    }

    VolumeInfo {
        multipart: false,
        open_path: selected_str.clone(),
        selected_path: selected_str,
        format: "rar",
        hint: String::new(),
        can_repair: true,
    }
}

impl VolumeInfo {
    pub fn resolved_path(&self) -> PathBuf {
        PathBuf::from(&self.open_path)
    }
}

fn rar_first_part_name(file_name: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(\.part)(\d+)(\.rar)$").ok()?;
    if let Some(caps) = re.captures(file_name) {
        let digits = caps.get(2)?.as_str();
        let width = digits.len().max(2);
        return Some(re.replace(file_name, format!(".part{:0width$}.rar", 1)).into_owned());
    }

    let re_r = Regex::new(r"(?i)(\.r)(\d+)$").ok()?;
    if let Some(caps) = re_r.captures(file_name) {
        let stem = file_name.trim_end_matches(caps.get(0)?.as_str());
        let with_rar = format!("{stem}.rar");
        return Some(with_rar);
    }

    let re_part_r = Regex::new(r"(?i)(\.part)(\d+)(\.r\d+)$").ok()?;
    if let Some(caps) = re_part_r.captures(file_name) {
        let digits = caps.get(2)?.as_str();
        let width = digits.len().max(2);
        let suffix = caps.get(3)?.as_str();
        return Some(
            re_part_r
                .replace(file_name, format!(".part{:0width$}{suffix}", 1))
                .into_owned(),
        );
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rar_part_name_to_first() {
        assert_eq!(
            rar_first_part_name("backup.part03.rar").as_deref(),
            Some("backup.part01.rar")
        );
        assert_eq!(
            rar_first_part_name("backup.part3.rar").as_deref(),
            Some("backup.part01.rar")
        );
        assert_eq!(
            rar_first_part_name("data.r05").as_deref(),
            Some("data.rar")
        );
    }
}
