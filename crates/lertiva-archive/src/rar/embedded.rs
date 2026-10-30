use std::path::Path;

use unrar_ng::Archive;

use crate::{ArchiveError, Result};

use super::Op;

fn map_unrar(err: unrar_ng::error::UnrarError) -> ArchiveError {
    let msg = err.to_string();
    if msg.to_lowercase().contains("password") {
        ArchiveError::Rar("Parola hatalı veya gerekli".into())
    } else {
        ArchiveError::Rar(msg)
    }
}

pub fn run(archive: &Path, password: Option<&str>, op: &Op<'_>) -> Result<Vec<String>> {
    match op {
        Op::List => list(archive, password),
        Op::Extract(dest) => {
            extract(archive, dest, password)?;
            Ok(vec![])
        }
        Op::Test => {
            let temp = tempfile::tempdir()?;
            extract(archive, temp.path(), password)?;
            Ok(vec![])
        }
    }
}

fn archive_for<'a>(path: &'a Path, password: Option<&'a str>) -> Archive<'a> {
    if let Some(pw) = password.filter(|p| !p.is_empty()) {
        Archive::with_password(path, pw.as_bytes())
    } else {
        Archive::new(path)
    }
}

fn list(archive: &Path, password: Option<&str>) -> Result<Vec<String>> {
    let mut open = archive_for(archive, password)
        .open_for_listing()
        .map_err(map_unrar)?;
    let mut names = Vec::new();
    while let Some(entry) = open.read_header().map_err(map_unrar)? {
        names.push(
            entry
                .entry()
                .filename
                .to_string_lossy()
                .replace('\\', "/"),
        );
        open = entry.skip().map_err(map_unrar)?;
    }
    Ok(names)
}

fn extract(archive: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
    std::fs::create_dir_all(dest)?;
    archive_for(archive, password)
        .open_for_processing()
        .map_err(map_unrar)?
        .extract_all(dest)
        .map_err(map_unrar)
}
