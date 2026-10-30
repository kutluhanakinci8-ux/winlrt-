use std::path::{Path, PathBuf};

use winlrt_archive::profile::CompressionProfile;
use winlrt_archive::{rar, sevenz, wlrt, zip_io};

use crate::ActionResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupMode {
    Normal,
    OpenArchive(String),
    ExtractHere(String),
    CompressFolder(String),
    CompressFolderQuick(String),
    CompressSelection(Vec<String>),
    CompressSelectionQuick(Vec<String>),
}

pub fn is_archive_path(path: &str) -> bool {
    let lower = path.to_lowercase();
    ["zip", "wlrt", "rar", "7z"]
        .iter()
        .any(|ext| lower.ends_with(&format!(".{ext}")))
}

pub fn format_from_path(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    match ext.as_str() {
        "zip" => Some("zip"),
        "wlrt" => Some("wlrt"),
        "rar" => Some("rar"),
        "7z" => Some("7z"),
        _ => None,
    }
}

fn flag_with_directory(args: &[String], flag: &str) -> Option<String> {
    for (i, arg) in args.iter().enumerate() {
        if arg == flag {
            if let Some(next) = args.get(i + 1) {
                if Path::new(next).is_dir() {
                    return Some(next.clone());
                }
            }
            return args
                .iter()
                .find(|a| Path::new(a.as_str()).is_dir())
                .cloned();
        }
    }
    None
}

fn collect_paths_after_flag(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut after = false;
    for arg in args {
        if arg == flag {
            after = true;
            continue;
        }
        if !after {
            continue;
        }
        if arg.starts_with('-') {
            continue;
        }
        let path = Path::new(arg);
        if path.is_file() || path.is_dir() {
            out.push(arg.clone());
        }
    }
    out
}

pub fn parse_startup_mode() -> StartupMode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--extract-here" {
            if let Some(next) = args.get(i + 1) {
                if is_archive_path(next) && Path::new(next).is_file() {
                    return StartupMode::ExtractHere(next.clone());
                }
            }
            if let Some(p) = args.iter().find(|a| is_archive_path(a) && Path::new(a.as_str()).is_file()) {
                return StartupMode::ExtractHere(p.clone());
            }
        }
    }
    let quick_sel = collect_paths_after_flag(&args, "--compress-selection-quick");
    if !quick_sel.is_empty() {
        return StartupMode::CompressSelectionQuick(quick_sel);
    }
    let sel = collect_paths_after_flag(&args, "--compress-selection");
    if !sel.is_empty() {
        return StartupMode::CompressSelection(sel);
    }
    if let Some(dir) = flag_with_directory(&args, "--compress-folder-quick") {
        return StartupMode::CompressFolderQuick(dir);
    }
    if let Some(dir) = flag_with_directory(&args, "--compress-folder") {
        return StartupMode::CompressFolder(dir);
    }
    for arg in &args {
        if is_archive_path(arg) && Path::new(arg).is_file() {
            return StartupMode::OpenArchive(arg.clone());
        }
    }
    StartupMode::Normal
}

pub fn suggested_archive_path(folder_path: &str, ext: &str) -> Result<String, String> {
    let folder = PathBuf::from(folder_path);
    if !folder.is_dir() {
        return Err("Klasör bulunamadı".into());
    }
    let name = folder
        .file_name()
        .ok_or_else(|| "Geçersiz klasör adı".to_string())?;
    let parent = folder.parent().unwrap_or(Path::new("."));
    Ok(parent
        .join(format!("{}.{}", name.to_string_lossy(), ext))
        .to_string_lossy()
        .into_owned())
}

pub fn suggested_selection_archive_path(paths: &[String], ext: &str) -> Result<String, String> {
    if paths.is_empty() {
        return Err("Seçim boş".into());
    }
    let first = PathBuf::from(&paths[0]);
    let parent = first
        .parent()
        .ok_or_else(|| "Hedef klasör belirlenemedi".to_string())?;
    if paths.len() == 1 {
        if first.is_file() {
            let stem = first
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "arsiv".into());
            return Ok(parent
                .join(format!("{stem}.{ext}"))
                .to_string_lossy()
                .into_owned());
        }
        if first.is_dir() {
            return suggested_archive_path(&paths[0], ext);
        }
    }
    Ok(parent
        .join(format!("winlrt-secim.{ext}"))
        .to_string_lossy()
        .into_owned())
}

pub fn extract_here(archive_path: &str) -> Result<ActionResult, String> {
    let archive = PathBuf::from(archive_path);
    if !archive.is_file() {
        return Err("Arşiv bulunamadı".into());
    }
    let dest = archive
        .parent()
        .ok_or_else(|| "Hedef klasör belirlenemedi".to_string())?
        .to_path_buf();
    let format =
        format_from_path(&archive).ok_or_else(|| "Desteklenmeyen uzantı".to_string())?;
    match format {
        "zip" => zip_io::extract(&archive, &dest, None).map_err(|e| e.to_string())?,
        "wlrt" => wlrt::extract(&archive, &dest, None).map_err(|e| e.to_string())?,
        "rar" => rar::extract(&archive, &dest, None).map_err(|e| e.to_string())?,
        "7z" => sevenz::extract(&archive, &dest, None).map_err(|e| e.to_string())?,
        _ => return Err("Desteklenmeyen format".into()),
    }
    Ok(ActionResult {
        ok: true,
        message: format!(
            "Buraya açıldı: {} → {}",
            archive.file_name().unwrap_or_default().to_string_lossy(),
            dest.display()
        ),
    })
}

pub fn compress_folder_quick(folder_path: &str) -> Result<ActionResult, String> {
    let folder = PathBuf::from(folder_path);
    if !folder.is_dir() {
        return Err("Klasör bulunamadı".into());
    }
    let dest = PathBuf::from(suggested_archive_path(folder_path, "wlrt")?);
    wlrt::create(&folder, &dest, CompressionProfile::Balanced, None)
        .map_err(|e| e.to_string())?;
    Ok(ActionResult {
        ok: true,
        message: format!("Hızlı .wlrt oluşturuldu: {}", dest.display()),
    })
}

pub fn compress_selection_quick(paths: &[String]) -> Result<ActionResult, String> {
    let sources: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    if sources.is_empty() {
        return Err("Seçim boş".into());
    }
    let dest = PathBuf::from(suggested_selection_archive_path(paths, "wlrt")?);
    wlrt::create_from_paths(&sources, &dest, CompressionProfile::Balanced, None)
        .map_err(|e| e.to_string())?;
    Ok(ActionResult {
        ok: true,
        message: format!(
            "Hızlı .wlrt ({} öğe): {}",
            paths.len(),
            dest.display()
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn suggested_archive_path_sibling() {
        let base = std::env::temp_dir().join(format!("winlrt-cli-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let sub = base.join("Projem");
        fs::create_dir(&sub).unwrap();
        let s = suggested_archive_path(sub.to_str().unwrap(), "wlrt").unwrap();
        assert!(s.ends_with("Projem.wlrt"));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn suggested_selection_multi_default_name() {
        let base = std::env::temp_dir().join(format!("winlrt-cli-sel-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let a = base.join("a.txt");
        let b = base.join("b.txt");
        fs::write(&a, b"1").unwrap();
        fs::write(&b, b"2").unwrap();
        let s = suggested_selection_archive_path(
            &[a.to_string_lossy().into(), b.to_string_lossy().into()],
            "wlrt",
        )
        .unwrap();
        assert!(s.ends_with("winlrt-secim.wlrt"));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn parse_compress_selection_flag() {
        let base = std::env::temp_dir().join(format!("winlrt-cli-flag-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let a = base.join("a.txt");
        let b = base.join("b.txt");
        fs::write(&a, b"1").unwrap();
        fs::write(&b, b"2").unwrap();
        let inner = vec![
            "--compress-selection".into(),
            a.to_string_lossy().into_owned(),
            b.to_string_lossy().into_owned(),
        ];
        let sel = collect_paths_after_flag(&inner, "--compress-selection");
        assert_eq!(sel.len(), 2);
        let _ = fs::remove_dir_all(&base);
    }
}
