use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{ArchiveError, Result};

use super::Op;

pub fn sidecar_unrar_available() -> bool {
    sidecar_candidates()
        .into_iter()
        .any(|p| p.is_file())
}

pub fn run(archive: &Path, password: Option<&str>, op: &Op<'_>) -> Result<Vec<String>> {
    let archive = path_str(archive)?;
    match op {
        Op::List => cli_list(archive, password),
        Op::Extract(dest) => {
            cli_extract(archive, path_str(dest)?, password)?;
            Ok(vec![])
        }
        Op::Test => {
            cli_test(archive, password)?;
            Ok(vec![])
        }
    }
}

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| ArchiveError::Rar("geçersiz dosya yolu".into()))
}

fn password_args(password: Option<&str>) -> Vec<String> {
    match password.filter(|p| !p.is_empty()) {
        Some(p) => vec![format!("-p{p}")],
        None => vec!["-p-".into()],
    }
}

fn run_unrar(args: &[&str]) -> Result<std::process::Output> {
    let mut last = None;
    for program in unrar_programs() {
        match Command::new(&program).args(args).output() {
            Ok(output) => return Ok(output),
            Err(e) => last = Some((program, e)),
        }
    }
    if let Some((program, e)) = last {
        return Err(ArchiveError::Rar(format!(
            "UnRAR çalıştırılamadı ({}): {e}",
            program.display()
        )));
    }
    Err(ArchiveError::Rar(
        "UnRAR bulunamadı — Windows kurulumunda libunrar veya WinRAR/unrar gerekir.".into(),
    ))
}

fn unrar_programs() -> Vec<PathBuf> {
    let mut programs = sidecar_candidates();
    programs.push(PathBuf::from("unrar"));
    programs.push(PathBuf::from("UnRAR"));
    programs.push(PathBuf::from("unrar.exe"));
    programs.push(PathBuf::from("UnRAR.exe"));
    programs
}

fn sidecar_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(dir) = std::env::var("WINLRT_UNRAR") {
        out.push(PathBuf::from(dir));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            out.push(parent.join("unrar.exe"));
            out.push(parent.join("UnRAR.exe"));
            out.push(parent.join("unrar"));
            out.push(parent.join("UnRAR"));
        }
    }
    out
}

fn map_cli_output(output: &std::process::Output, action: &str) -> Result<()> {
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stderr}\n{stdout}").to_lowercase();
    if combined.contains("wrong password") || combined.contains("incorrect password") {
        return Err(ArchiveError::Rar("Parola hatalı".into()));
    }
    if combined.contains("encrypted") && combined.contains("password") {
        return Err(ArchiveError::Rar("Parola gerekli".into()));
    }
    if combined.contains("crc failed") || combined.contains("corrupt") {
        return Err(ArchiveError::Rar(format!(
            "Arşiv bozuk veya eksik cilt ({action})"
        )));
    }
    Err(ArchiveError::Rar(format!(
        "UnRAR {action} başarısız (kod {:?})",
        output.status.code()
    )))
}

fn cli_list(archive: &str, password: Option<&str>) -> Result<Vec<String>> {
    let mut args = vec!["lb", "-idq"];
    let pw = password_args(password);
    args.extend(pw.iter().map(String::as_str));
    args.push(archive);
    let output = run_unrar(&args)?;
    if !output.status.success() {
        map_cli_output(&output, "listeleme")?;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect())
}

/// Tek girdiyi hedef dosyaya çıkarır (`unrar x … girdi`).
pub fn extract_one_file(
    archive: &Path,
    entry: &str,
    dest_file: &Path,
    password: Option<&str>,
) -> Result<()> {
    let archive = path_str(archive)?;
    let want = entry.replace('\\', "/");
    if let Some(parent) = dest_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temp = tempfile::tempdir()?;
    let temp_dest = temp.path().to_str().ok_or_else(|| {
        ArchiveError::Rar("geçici klasör yolu".into())
    })?;
    let mut args = vec!["x", "-o+", "-idq"];
    let pw = password_args(password);
    args.extend(pw.iter().map(String::as_str));
    args.push(archive);
    args.push(&want);
    args.push(temp_dest);
    args.push("-y");
    let output = run_unrar(&args)?;
    map_cli_output(&output, "tek dosya açma")?;
    let found = find_extracted_file(temp.path(), &want)?;
    std::fs::copy(&found, dest_file).map_err(|e| ArchiveError::Rar(e.to_string()))?;
    Ok(())
}

fn find_extracted_file(root: &Path, want: &str) -> Result<PathBuf> {
    let direct = root.join(want);
    if direct.is_file() {
        return Ok(direct);
    }
    let leaf = want.rsplit('/').next().unwrap_or(want);
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let p = entry.path();
        let rel = p
            .strip_prefix(root)
            .unwrap_or(p)
            .to_string_lossy()
            .replace('\\', "/");
        if rel == want || rel.ends_with(want) || p.file_name().and_then(|s| s.to_str()) == Some(leaf) {
            return Ok(p.to_path_buf());
        }
    }
    Err(ArchiveError::Rar(format!("çıkarılan dosya bulunamadı: {want}")))
}

fn cli_extract(archive: &str, dest: &str, password: Option<&str>) -> Result<()> {
    std::fs::create_dir_all(dest)?;
    let mut args = vec!["x", "-o+", "-idq"];
    let pw = password_args(password);
    args.extend(pw.iter().map(String::as_str));
    args.push(archive);
    args.push(dest);
    args.push("-y");
    let output = run_unrar(&args)?;
    map_cli_output(&output, "açma")
}

pub fn repair(archive: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
    std::fs::create_dir_all(dest)?;
    let archive = path_str(archive)?;
    let dest = path_str(dest)?;
    let mut args = vec!["r", "-idq"];
    let pw = password_args(password);
    args.extend(pw.iter().map(String::as_str));
    args.push(archive);
    args.push(dest);
    args.push("-y");
    let output = run_unrar(&args)?;
    map_cli_output(&output, "onarım")
}

fn cli_test(archive: &str, password: Option<&str>) -> Result<()> {
    let mut args = vec!["t", "-idq"];
    let pw = password_args(password);
    args.extend(pw.iter().map(String::as_str));
    args.push(archive);
    let output = run_unrar(&args)?;
    map_cli_output(&output, "test")
}
