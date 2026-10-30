use std::path::{Path, PathBuf};

use lertiva_archive::profile::CompressionProfile;
use lertiva_archive::zip_io;

use crate::ActionResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupMode {
    Normal,
    OpenArchive(String),
    ExtractHere(String),
    CompressFolder(String),
    CompressFolderQuick(String),
    CompressSelection(Vec<String>),
}

pub fn is_zip_path(path: &str) -> bool {
    path.to_lowercase().ends_with(".zip")
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
                if is_zip_path(next) && Path::new(next).is_file() {
                    return StartupMode::ExtractHere(next.clone());
                }
            }
            if let Some(p) = args
                .iter()
                .find(|a| is_zip_path(a) && Path::new(a.as_str()).is_file())
            {
                return StartupMode::ExtractHere(p.clone());
            }
        }
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
        if is_zip_path(arg) && Path::new(arg).is_file() {
            return StartupMode::OpenArchive(arg.clone());
        }
    }
    StartupMode::Normal
}

pub fn suggested_archive_path(folder_path: &str) -> Result<String, String> {
    let folder = PathBuf::from(folder_path);
    if !folder.is_dir() {
        return Err("Klasör bulunamadı".into());
    }
    let name = folder
        .file_name()
        .ok_or_else(|| "Geçersiz klasör adı".to_string())?;
    let parent = folder.parent().unwrap_or(Path::new("."));
    Ok(parent
        .join(format!("{}.zip", name.to_string_lossy()))
        .to_string_lossy()
        .into_owned())
}

pub fn suggested_selection_archive_path(paths: &[String]) -> Result<String, String> {
    lertiva_archive::naming::suggested_selection_archive_path(paths, "zip")
}

pub fn extract_here(archive_path: &str) -> Result<ActionResult, String> {
    let archive = PathBuf::from(archive_path);
    if !archive.is_file() {
        return Err("ZIP bulunamadı".into());
    }
    let dest = archive
        .parent()
        .ok_or_else(|| "Hedef klasör belirlenemedi".to_string())?
        .to_path_buf();
    zip_io::extract(&archive, &dest, None).map_err(|e| e.to_string())?;
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
    let dest = PathBuf::from(suggested_archive_path(folder_path)?);
    zip_io::create_with_profile(
        &folder,
        &dest,
        CompressionProfile::EmailAttachment,
        None,
    )
    .map_err(|e| e.to_string())?;
    Ok(ActionResult {
        ok: true,
        message: format!("E-posta profili ZIP: {}", dest.display()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn suggested_zip_path_sibling() {
        let base = std::env::temp_dir().join(format!("winlrt-zip-cli-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let sub = base.join("Projem");
        fs::create_dir(&sub).unwrap();
        let s = suggested_archive_path(sub.to_str().unwrap()).unwrap();
        assert!(s.ends_with("Projem.zip"));
        let _ = fs::remove_dir_all(&base);
    }
}
