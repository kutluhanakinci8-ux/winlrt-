use std::path::{Path, PathBuf};
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
    zip_io::create_with_profile(&fixture, &zip_path, profile)?;
    let zip_create_ms = t0.elapsed().as_millis();

    let t1 = Instant::now();
    wlrt::create(&fixture, &wlrt_path, profile)?;
    let wlrt_create_ms = t1.elapsed().as_millis();

    let zip_bytes = std::fs::metadata(&zip_path)?.len();
    let wlrt_bytes = std::fs::metadata(&wlrt_path)?.len();

    let t2 = Instant::now();
    std::fs::create_dir_all(&zip_out)?;
    zip_io::extract(&zip_path, &zip_out)?;
    let zip_extract_ms = t2.elapsed().as_millis();

    let t3 = Instant::now();
    std::fs::create_dir_all(&wlrt_out)?;
    wlrt::extract(&wlrt_path, &wlrt_out)?;
    let wlrt_extract_ms = t3.elapsed().as_millis();

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
