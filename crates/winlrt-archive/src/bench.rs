use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::Serialize;

use crate::profile::CompressionProfile;
use crate::{wlrt, zip_io, Result};

#[derive(Debug, Serialize)]
pub struct BenchReport {
    pub fixture: String,
    pub profile: String,
    pub zip_create_ms: u128,
    pub wlrt_create_ms: u128,
    pub zip_extract_ms: u128,
    pub wlrt_extract_ms: u128,
    pub zip_bytes: u64,
    pub wlrt_bytes: u64,
    pub raw_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seven_zip_create_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seven_zip_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seven_zip_extract_ms: Option<u128>,
}

fn include_7zip_bench() -> bool {
    std::env::var("WINLRT_BENCH_INCLUDE_7Z")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn run(fixture: &Path, profile: CompressionProfile) -> Result<BenchReport> {
    let fixture = fixture.canonicalize()?;
    let work = std::env::temp_dir().join(format!(
        "winlrt-bench-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work)?;

    let zip_path = work.join("out.zip");
    let wlrt_path = work.join("out.wlrt");
    let zip_out = work.join("zip_extract");
    let wlrt_out = work.join("wlrt_extract");

    let raw_bytes = dir_size(&fixture)?;

    let t0 = Instant::now();
    zip_io::create_with_profile(&fixture, &zip_path, profile, None)?;
    let zip_create_ms = t0.elapsed().as_millis();

    let t1 = Instant::now();
    wlrt::create(&fixture, &wlrt_path, profile, None)?;
    let wlrt_create_ms = t1.elapsed().as_millis();

    let zip_bytes = std::fs::metadata(&zip_path)?.len();
    let wlrt_bytes = std::fs::metadata(&wlrt_path)?.len();

    let t2 = Instant::now();
    std::fs::create_dir_all(&zip_out)?;
    zip_io::extract(&zip_path, &zip_out, None)?;
    let zip_extract_ms = t2.elapsed().as_millis();

    let t3 = Instant::now();
    std::fs::create_dir_all(&wlrt_out)?;
    wlrt::extract(&wlrt_path, &wlrt_out, None)?;
    let wlrt_extract_ms = t3.elapsed().as_millis();

    let mut seven_zip_create_ms = None;
    let mut seven_zip_bytes = None;
    let mut seven_zip_extract_ms = None;
    if include_7zip_bench() {
        if let Ok(m) = bench_7zip_zip(&fixture, &work, profile) {
            seven_zip_create_ms = Some(m.create_ms);
            seven_zip_bytes = Some(m.bytes);
            seven_zip_extract_ms = Some(m.extract_ms);
        }
    }

    let _ = std::fs::remove_dir_all(&work);

    Ok(BenchReport {
        fixture: fixture.display().to_string(),
        profile: format!("{profile:?}"),
        zip_create_ms,
        wlrt_create_ms,
        zip_extract_ms,
        wlrt_extract_ms,
        zip_bytes,
        wlrt_bytes,
        raw_bytes,
        seven_zip_create_ms,
        seven_zip_bytes,
        seven_zip_extract_ms,
    })
}

struct SevenZipMetrics {
    create_ms: u128,
    extract_ms: u128,
    bytes: u64,
}

#[cfg(windows)]
fn seven_zip_exe() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from(r"C:\Program Files\7-Zip\7z.exe"),
        PathBuf::from(r"C:\Program Files (x86)\7-Zip\7z.exe"),
    ];
    candidates.into_iter().find(|p| p.is_file()).or_else(|| {
        which_7z_via_path()
    })
}

#[cfg(windows)]
fn which_7z_via_path() -> Option<PathBuf> {
    let output = Command::new("where").arg("7z").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let line = String::from_utf8_lossy(&output.stdout);
    let first = line.lines().next()?.trim();
    if first.is_empty() {
        return None;
    }
    let p = PathBuf::from(first);
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

#[cfg(not(windows))]
fn seven_zip_exe() -> Option<PathBuf> {
    Command::new("7z")
        .arg("--help")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|_| PathBuf::from("7z"))
}

fn seven_zip_level(profile: CompressionProfile) -> &'static str {
    match profile {
        CompressionProfile::Fast => "1",
        CompressionProfile::Balanced => "5",
        CompressionProfile::Maximum | CompressionProfile::EmailAttachment => "9",
    }
}

fn bench_7zip_zip(fixture: &Path, work: &Path, profile: CompressionProfile) -> Result<SevenZipMetrics> {
    let exe = seven_zip_exe().ok_or_else(|| {
        crate::ArchiveError::NotYet("7-Zip bulunamadı (WINLRT_BENCH_INCLUDE_7Z=1)")
    })?;
    let archive = work.join("7z-out.zip");
    let extract_dir = work.join("7z-extract");
    let level = seven_zip_level(profile);
    let t0 = Instant::now();
    let mut src = fixture.display().to_string();
    if fixture.is_dir() {
        src.push(std::path::MAIN_SEPARATOR);
    }
    let status = Command::new(&exe)
        .args([
            "a",
            "-tzip",
            "-mx",
            level,
            "-y",
            &archive.to_string_lossy(),
            &src,
        ])
        .status()
        .map_err(|e| crate::ArchiveError::Io(e))?;
    if !status.success() {
        return Err(crate::ArchiveError::Zip("7-Zip oluşturma başarısız".into()));
    }
    let create_ms = t0.elapsed().as_millis();
    let bytes = std::fs::metadata(&archive)?.len();
    std::fs::create_dir_all(&extract_dir)?;
    let t1 = Instant::now();
    let status = Command::new(&exe)
        .args([
            "x",
            &archive.to_string_lossy(),
            &format!("-o{}", extract_dir.display()),
            "-y",
        ])
        .status()
        .map_err(|e| crate::ArchiveError::Io(e))?;
    if !status.success() {
        return Err(crate::ArchiveError::Zip("7-Zip açma başarısız".into()));
    }
    Ok(SevenZipMetrics {
        create_ms,
        extract_ms: t1.elapsed().as_millis(),
        bytes,
    })
}

fn dir_size(path: &Path) -> Result<u64> {
    let mut total = 0u64;
    if path.is_file() {
        return Ok(std::fs::metadata(path)?.len());
    }
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            total += entry.metadata().map_err(|e| {
                crate::ArchiveError::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
            })?.len();
        }
    }
    Ok(total)
}

pub fn default_fixture(repo_root: &Path) -> PathBuf {
    repo_root.join("benchmarks/fixtures/text")
}

pub fn fixtures_root(repo_root: &Path) -> PathBuf {
    repo_root.join("benchmarks/fixtures")
}

/// `benchmarks/fixtures/*` altındaki her klasör için benchmark.
pub fn run_all(repo_root: &Path, profile: CompressionProfile) -> Result<Vec<BenchReport>> {
    let root = fixtures_root(repo_root);
    if !root.is_dir() {
        return Err(crate::ArchiveError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            root.display().to_string(),
        )));
    }
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&root)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    dirs.into_iter().map(|p| run(&p, profile)).collect()
}

#[derive(Debug, Serialize)]
pub struct BenchSuite {
    pub profile: String,
    pub platform: String,
    pub generated_at: String,
    pub reports: Vec<BenchReport>,
}

pub fn suite_metadata() -> (String, String) {
    let platform = format!(
        "{} / {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let generated_at = chrono_lite_timestamp();
    (platform, generated_at)
}

fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs} UTC")
}

pub fn suite_to_markdown(suite: &BenchSuite) -> String {
    let mut out = String::new();
    out.push_str("# winlrt motor benchmark\n\n");
    out.push_str(&format!("- Profil: `{}`\n", suite.profile));
    out.push_str(&format!("- Platform: `{}`\n", suite.platform));
    out.push_str(&format!("- Zaman: `{}`\n\n", suite.generated_at));
    out.push_str("| Fixture | Ham (B) | ZIP (B) | .wlrt (B) | wlrt vs ZIP | ZIP oluştur (ms) | .wlrt oluştur (ms) |");
    if suite
        .reports
        .first()
        .and_then(|r| r.seven_zip_bytes)
        .is_some()
    {
        out.push_str(" 7-Zip (B) | 7-Zip oluştur (ms) |");
    }
    out.push('\n');
    out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |");
    if suite
        .reports
        .first()
        .and_then(|r| r.seven_zip_bytes)
        .is_some()
    {
        out.push_str(" ---: | ---: |");
    }
    out.push('\n');
    for r in &suite.reports {
        let ratio = if r.zip_bytes > 0 {
            (1.0 - r.wlrt_bytes as f64 / r.zip_bytes as f64) * 100.0
        } else {
            0.0
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {:.1}% | {} | {} |",
            short_name(&r.fixture),
            r.raw_bytes,
            r.zip_bytes,
            r.wlrt_bytes,
            ratio,
            r.zip_create_ms,
            r.wlrt_create_ms,
        ));
        if let (Some(b), Some(c)) = (r.seven_zip_bytes, r.seven_zip_create_ms) {
            out.push_str(&format!(" {} | {} |", b, c));
        }
        out.push('\n');
    }
    out.push_str("\n_Otomatik rapor — winlrt-archive bench --all_\n");
    out
}

fn short_name(fixture: &str) -> String {
    Path::new(fixture)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(fixture)
        .to_string()
}
