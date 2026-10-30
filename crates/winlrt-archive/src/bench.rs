use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winrar_create_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winrar_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winrar_extract_ms: Option<u128>,
}

fn include_7zip_bench() -> bool {
    env_flag("WINLRT_BENCH_INCLUDE_7Z")
}

fn include_winrar_bench() -> bool {
    env_flag("WINLRT_BENCH_INCLUDE_WINRAR")
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn is_bench_nested_archive(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            matches!(
                e.to_ascii_lowercase().as_str(),
                "rar" | "zip" | "7z" | "wlrt" | "gz" | "bz2" | "xz" | "cab"
            )
        })
        .unwrap_or(false)
}

/// KPI: yalnızca «sıkıştırılacak kullanıcı dosyaları»; fixture içindeki hazır arşivler hariç.
fn prepare_bench_input(fixture: &Path, work: &Path) -> Result<PathBuf> {
    let input_root = work.join("input");
    let _ = std::fs::remove_dir_all(&input_root);
    if fixture.is_file() {
        if is_bench_nested_archive(fixture) {
            return Err(crate::ArchiveError::Zip(
                "benchmark: tek dosya arşiv fixture desteklenmiyor".into(),
            ));
        }
        std::fs::create_dir_all(&input_root)?;
        let name = fixture.file_name().ok_or_else(|| {
            crate::ArchiveError::Zip("benchmark: geçersiz dosya adı".into())
        })?;
        std::fs::copy(fixture, input_root.join(name))?;
        return Ok(input_root);
    }
    copy_bench_tree(fixture, &input_root)?;
    Ok(input_root)
}

fn bench_elapsed_ms(d: Duration) -> u128 {
    let us = d.as_micros();
    if us == 0 {
        0
    } else {
        us.div_ceil(1000)
    }
}

/// Isınma + ölçüm (Faz 3 hız KPI — milisaniye çözünürlük).
fn time_extract_ms(
    archive: &Path,
    dest: &Path,
    extract: impl Fn(&Path, &Path) -> Result<()>,
) -> Result<u128> {
    let _ = std::fs::remove_dir_all(dest);
    std::fs::create_dir_all(dest)?;
    let _ = extract(archive, dest);
    let _ = std::fs::remove_dir_all(dest);
    std::fs::create_dir_all(dest)?;
    let t0 = Instant::now();
    extract(archive, dest)?;
    Ok(bench_elapsed_ms(t0.elapsed()))
}

fn copy_bench_tree(from: &Path, to: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(from)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let rel = path.strip_prefix(from).unwrap_or(path);
        if path.is_dir() {
            std::fs::create_dir_all(to.join(rel))?;
        } else if path.is_file() {
            if is_bench_nested_archive(path) {
                continue;
            }
            let dest = to.join(rel);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(path, dest)?;
        }
    }
    Ok(())
}

pub fn run(fixture: &Path, profile: CompressionProfile) -> Result<BenchReport> {
    let fixture = fixture.canonicalize()?;
    let work = std::env::temp_dir().join(format!(
        "winlrt-bench-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work)?;

    let input = prepare_bench_input(&fixture, &work)?;
    let zip_path = work.join("out.zip");
    let wlrt_path = work.join("out.wlrt");
    let zip_out = work.join("zip_extract");
    let wlrt_out = work.join("wlrt_extract");

    let raw_bytes = dir_size(&input)?;

    let t0 = Instant::now();
    zip_io::create_with_profile(&input, &zip_path, profile, None)?;
    let zip_create_ms = t0.elapsed().as_millis();

    let t1 = Instant::now();
    wlrt::create(&input, &wlrt_path, profile, None)?;
    let wlrt_create_ms = t1.elapsed().as_millis();

    let zip_bytes = std::fs::metadata(&zip_path)?.len();
    let wlrt_bytes = std::fs::metadata(&wlrt_path)?.len();

    let zip_extract_ms = time_extract_ms(&zip_path, &zip_out, |p, d| zip_io::extract(p, d, None))?;
    let wlrt_extract_ms = time_extract_ms(&wlrt_path, &wlrt_out, |p, d| wlrt::extract(p, d, None))?;

    let mut seven_zip_create_ms = None;
    let mut seven_zip_bytes = None;
    let mut seven_zip_extract_ms = None;
    if include_7zip_bench() {
        if let Ok(m) = bench_7zip_zip(&input, &work, profile) {
            seven_zip_create_ms = Some(m.create_ms);
            seven_zip_bytes = Some(m.bytes);
            seven_zip_extract_ms = Some(m.extract_ms);
        }
    }

    let mut winrar_create_ms = None;
    let mut winrar_bytes = None;
    let mut winrar_extract_ms = None;
    if include_winrar_bench() {
        if let Ok(m) = bench_winrar_zip(&input, &work, profile) {
            winrar_create_ms = Some(m.create_ms);
            winrar_bytes = Some(m.bytes);
            winrar_extract_ms = Some(m.extract_ms);
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
        winrar_create_ms,
        winrar_bytes,
        winrar_extract_ms,
    })
}

struct CompetitorZipMetrics {
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

fn winrar_zip_level(profile: CompressionProfile) -> &'static str {
    match profile {
        CompressionProfile::Fast => "1",
        CompressionProfile::Balanced => "3",
        CompressionProfile::Maximum | CompressionProfile::EmailAttachment => "5",
    }
}

fn bench_7zip_zip(fixture: &Path, work: &Path, profile: CompressionProfile) -> Result<CompetitorZipMetrics> {
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
    Ok(CompetitorZipMetrics {
        create_ms,
        extract_ms: t1.elapsed().as_millis(),
        bytes,
    })
}

#[cfg(windows)]
fn winrar_exe() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from(r"C:\Program Files\WinRAR\Rar.exe"),
        PathBuf::from(r"C:\Program Files (x86)\WinRAR\Rar.exe"),
    ];
    candidates.into_iter().find(|p| p.is_file()).or_else(|| {
        let output = Command::new("where").arg("rar").output().ok()?;
        if !output.status.success() {
            return None;
        }
        let first = String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()?
            .trim();
        if first.is_empty() {
            return None;
        }
        let p = PathBuf::from(first);
        if p.is_file() { Some(p) } else { None }
    })
}

#[cfg(not(windows))]
fn winrar_exe() -> Option<PathBuf> {
    None
}

fn bench_winrar_zip(fixture: &Path, work: &Path, profile: CompressionProfile) -> Result<CompetitorZipMetrics> {
    let exe = winrar_exe().ok_or_else(|| {
        crate::ArchiveError::NotYet("WinRAR (Rar.exe) bulunamadı (WINLRT_BENCH_INCLUDE_WINRAR=1)")
    })?;
    let archive = work.join("winrar-out.zip");
    let extract_dir = work.join("winrar-extract");
    let level = winrar_zip_level(profile);
    let mut src = fixture.display().to_string();
    if fixture.is_dir() {
        if !src.ends_with(std::path::MAIN_SEPARATOR) {
            src.push(std::path::MAIN_SEPARATOR);
        }
    }
    let t0 = Instant::now();
    let status = Command::new(&exe)
        .args([
            "a",
            "-afzip",
            "-m",
            level,
            "-r",
            "-y",
            "-ep1",
            &archive.to_string_lossy(),
            &src,
        ])
        .status()
        .map_err(crate::ArchiveError::Io)?;
    if !status.success() {
        return Err(crate::ArchiveError::Zip("WinRAR ZIP oluşturma başarısız".into()));
    }
    let create_ms = t0.elapsed().as_millis();
    let bytes = std::fs::metadata(&archive)?.len();
    std::fs::create_dir_all(&extract_dir)?;
    let t1 = Instant::now();
    let status = Command::new(&exe)
        .args(["x", "-y", &archive.to_string_lossy(), &format!("{}\\", extract_dir.display())])
        .status()
        .map_err(crate::ArchiveError::Io)?;
    if !status.success() {
        return Err(crate::ArchiveError::Zip("WinRAR açma başarısız".into()));
    }
    Ok(CompetitorZipMetrics {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi: Option<BenchKpiSummary>,
}

#[derive(Debug, Serialize)]
pub struct BenchKpiRow {
    pub fixture: String,
    pub compression_ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_ok: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BenchKpiSummary {
    /// Faz 3: tüm fixture’larda sıkıştırma kriteri
    pub compression_gate: bool,
    /// Yalnızca `Fast` profilde: referans açmaya göre ≥ %30 hız
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_gate: Option<bool>,
    pub compression_pass_count: usize,
    pub compression_total: usize,
    pub speed_pass_count: usize,
    pub speed_total: usize,
    pub rows: Vec<BenchKpiRow>,
    pub notes: Vec<String>,
}

pub fn evaluate_kpi(suite: &BenchSuite) -> BenchKpiSummary {
    let speed_kpi = suite.profile.starts_with("Fast");
    let mut rows = Vec::with_capacity(suite.reports.len());
    let mut compression_pass = 0usize;
    let mut speed_pass = 0usize;
    let mut speed_total = 0usize;

    for r in &suite.reports {
        let compression_ok = compression_row_ok(r);
        if compression_ok {
            compression_pass += 1;
        }
        let speed_ok = if speed_kpi {
            if speed_row_applicable(r) {
                speed_total += 1;
                let ok = speed_row_ok(r);
                if ok {
                    speed_pass += 1;
                }
                Some(ok)
            } else {
                None
            }
        } else {
            None
        };
        rows.push(BenchKpiRow {
            fixture: short_name(&r.fixture),
            compression_ok,
            speed_ok,
        });
    }

    let total = suite.reports.len();
    let mut notes = vec![
        "Sıkıştırma (P0): .wlrt en az %10 daha küçük veya eşit boyutta daha hızlı oluşturma (ZIP referans).".into(),
    ];
    if speed_kpi {
        notes.push(
            "Hız (P0): .wlrt açma, 7-Zip/WinRAR/ZIP referansının en hızlısından ≥ %30 daha hızlı.".into(),
        );
    } else {
        notes.push(
            "Hız KPI yalnız `--profile fast` ile ölçülür; bu koşuda değerlendirilmedi.".into(),
        );
    }
    notes.push(
        "Kurumsal güven KPI’si bench dışı (AES-GCM, imza) — bkz. WINLRTRAR_PHASE0.md.".into(),
    );

    BenchKpiSummary {
        compression_gate: total > 0 && compression_pass == total,
        speed_gate: if speed_kpi {
            Some(speed_total > 0 && speed_pass == speed_total)
        } else {
            None
        },
        compression_pass_count: compression_pass,
        compression_total: total,
        speed_pass_count: speed_pass,
        speed_total,
        rows,
        notes,
    }
}

fn compression_row_ok(r: &BenchReport) -> bool {
    if r.zip_bytes == 0 {
        return false;
    }
    let ten_pct_smaller = r.wlrt_bytes as f64 <= r.zip_bytes as f64 * 0.90;
    let equal_faster = r.wlrt_bytes <= r.zip_bytes && r.wlrt_create_ms < r.zip_create_ms;
    ten_pct_smaller || equal_faster
}

fn reference_extract_ms(r: &BenchReport) -> u128 {
    let mut best = r.zip_extract_ms;
    if let Some(ms) = r.seven_zip_extract_ms {
        best = best.min(ms);
    }
    if let Some(ms) = r.winrar_extract_ms {
        best = best.min(ms);
    }
    best
}

fn speed_row_ok(r: &BenchReport) -> bool {
    let reference = reference_extract_ms(r);
    if reference == 0 {
        return r.wlrt_extract_ms == 0;
    }
    if r.wlrt_extract_ms > reference {
        return false;
    }
    let target = reference as f64 * 0.70;
    if target < 1.0 {
        return true;
    }
    r.wlrt_extract_ms <= target.floor() as u128
}

fn speed_row_applicable(r: &BenchReport) -> bool {
    reference_extract_ms(r) > 0 || r.wlrt_extract_ms > 0
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
    let has_7z = suite
        .reports
        .first()
        .and_then(|r| r.seven_zip_bytes)
        .is_some();
    let has_winrar = suite
        .reports
        .first()
        .and_then(|r| r.winrar_bytes)
        .is_some();
    if has_7z {
        out.push_str(" 7-Zip (B) | 7-Zip oluştur (ms) |");
    }
    if has_winrar {
        out.push_str(" WinRAR ZIP (B) | WinRAR oluştur (ms) |");
    }
    out.push('\n');
    out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |");
    if has_7z {
        out.push_str(" ---: | ---: |");
    }
    if has_winrar {
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
        if has_7z {
            if let (Some(b), Some(c)) = (r.seven_zip_bytes, r.seven_zip_create_ms) {
                out.push_str(&format!(" {} | {} |", b, c));
            } else {
                out.push_str(" — | — |");
            }
        }
        if has_winrar {
            if let (Some(b), Some(c)) = (r.winrar_bytes, r.winrar_create_ms) {
                out.push_str(&format!(" {} | {} |", b, c));
            } else {
                out.push_str(" — | — |");
            }
        }
        out.push('\n');
    }

    let computed_kpi = evaluate_kpi(suite);
    let kpi = suite.kpi.as_ref().unwrap_or(&computed_kpi);
    out.push_str("\n## KPI özeti (Faz 3 kapısı)\n\n");
    out.push_str(&format!(
        "- Sıkıştırma: **{}** ({}/{} fixture)\n",
        gate_label(kpi.compression_gate),
        kpi.compression_pass_count,
        kpi.compression_total
    ));
    if let Some(speed) = kpi.speed_gate {
        out.push_str(&format!(
            "- Hız (Fast): **{}** ({}/{} fixture)\n",
            gate_label(speed),
            kpi.speed_pass_count,
            kpi.speed_total
        ));
    }
    for note in &kpi.notes {
        out.push_str(&format!("- _{note}_\n"));
    }
    out.push_str("\n_Otomatik rapor — winlrt-archive bench --all_\n");
    out
}

fn gate_label(passed: bool) -> &'static str {
    if passed { "GEÇTİ" } else { "henüz değil" }
}

fn short_name(fixture: &str) -> String {
    Path::new(fixture)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(fixture)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(zip_e: u128, wlrt_e: u128) -> BenchReport {
        BenchReport {
            fixture: "x".into(),
            profile: "Fast".into(),
            zip_create_ms: 0,
            wlrt_create_ms: 0,
            zip_extract_ms: zip_e,
            wlrt_extract_ms: wlrt_e,
            zip_bytes: 100,
            wlrt_bytes: 80,
            raw_bytes: 100,
            seven_zip_extract_ms: None,
            seven_zip_create_ms: None,
            seven_zip_bytes: None,
            winrar_extract_ms: None,
            winrar_create_ms: None,
            winrar_bytes: None,
        }
    }

    #[test]
    fn speed_kpi_sub_ms_tie_passes() {
        assert!(speed_row_ok(&report(0, 0)));
    }

    #[test]
    fn speed_kpi_thirty_percent_rule() {
        assert!(speed_row_ok(&report(100, 70)));
        assert!(!speed_row_ok(&report(100, 71)));
    }

    #[test]
    fn speed_kpi_mixed_fixture_like() {
        assert!(speed_row_ok(&report(5, 3)));
        assert!(speed_row_ok(&report(7, 3)));
    }
}
