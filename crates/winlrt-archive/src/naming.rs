use std::path::{Path, PathBuf};

/// WinRAR benzeri çoklu seçim arşiv adı (aynı klasördeki dosyalar).
pub fn suggested_selection_archive_path(paths: &[String], ext: &str) -> Result<String, String> {
    if paths.is_empty() {
        return Err("Seçim boş".into());
    }
    let parsed: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let first = &parsed[0];
    let parent = first
        .parent()
        .ok_or_else(|| "Hedef klasör belirlenemedi".to_string())?;

    if paths.len() == 1 {
        return suggested_single_path(first, parent, ext);
    }

    if same_parent(&parsed)? {
        return archive_name_in_parent(&parsed[0], parent, ext);
    }

    // Karma üst klasör (WinRAR): hedef = ilk seçimin klasörü, ad = ilk öğe kuralı.
    let first_c = parsed[0].canonicalize().map_err(|e| e.to_string())?;
    let dest_parent = first_c
        .parent()
        .ok_or_else(|| "Hedef klasör belirlenemedi".to_string())?;
    if parsed.iter().all(|p| p.is_file()) {
        let stem = first_c
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "arsiv".into());
        return Ok(dest_parent
            .join(format!("{stem}.{ext}"))
            .to_string_lossy()
            .into_owned());
    }
    archive_name_in_parent(&parsed[0], dest_parent, ext)
}

fn archive_name_in_parent(first: &Path, parent: &Path, ext: &str) -> Result<String, String> {
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
    suggested_single_path(first, parent, ext)
}

fn suggested_single_path(path: &Path, parent: &Path, ext: &str) -> Result<String, String> {
    if path.is_file() {
        let stem = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "arsiv".into());
        return Ok(parent
            .join(format!("{stem}.{ext}"))
            .to_string_lossy()
            .into_owned());
    }
    if path.is_dir() {
        return suggested_folder_path(path, ext);
    }
    Err("Geçersiz seçim".into())
}

fn suggested_folder_path(folder: &Path, ext: &str) -> Result<String, String> {
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

fn same_parent(paths: &[PathBuf]) -> Result<bool, String> {
    let mut canon_parents = Vec::new();
    for p in paths {
        if !p.exists() {
            return Err(format!("Bulunamadı: {}", p.display()));
        }
        let c = p.canonicalize().map_err(|e| e.to_string())?;
        let parent = c
            .parent()
            .ok_or_else(|| "Üst klasör yok".to_string())?
            .to_path_buf();
        canon_parents.push(parent);
    }
    Ok(canon_parents.windows(2).all(|w| w[0] == w[1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn mixed_folder_files_use_first_stem_in_first_folder() {
        let base = std::env::temp_dir().join(format!("winlrt-mix-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let dir_a = base.join("a");
        let dir_b = base.join("b");
        fs::create_dir_all(&dir_a).unwrap();
        fs::create_dir_all(&dir_b).unwrap();
        let a = dir_a.join("Rapor.pdf");
        let b = dir_b.join("Ek.xlsx");
        fs::write(&a, b"x").unwrap();
        fs::write(&b, b"y").unwrap();
        let s = suggested_selection_archive_path(
            &[a.to_string_lossy().into(), b.to_string_lossy().into()],
            "wlrt",
        )
        .unwrap();
        assert!(s.ends_with("Rapor.wlrt"));
        assert!(s.contains("a"));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn multi_select_uses_first_file_stem_winrar_style() {
        let base = std::env::temp_dir().join(format!("winlrt-name-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let a = base.join("Rapor.pdf");
        let b = base.join("Ek.xlsx");
        fs::write(&a, b"x").unwrap();
        fs::write(&b, b"y").unwrap();
        let s = suggested_selection_archive_path(
            &[a.to_string_lossy().into(), b.to_string_lossy().into()],
            "wlrt",
        )
        .unwrap();
        assert!(s.ends_with("Rapor.wlrt"));
        let _ = fs::remove_dir_all(&base);
    }
}
