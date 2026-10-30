//! winlrtZIP — ZIP odaklı CLI (Faz 1).

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

use crate::bench::{self, BenchReport, BenchSuite};
use crate::profile::CompressionProfile;
use crate::{zip_io, Result};

#[derive(Parser)]
#[command(
    name = "winlrt-zip",
    about = "winlrtZIP — ZIP/ZIP64 CLI (ortak winlrt motoru)",
    version
)]
pub struct ZipCli {
    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: ZipCommands,
}

#[derive(Subcommand)]
pub enum ZipCommands {
    List { archive: PathBuf },
    Extract { archive: PathBuf, dest: PathBuf },
    Create {
        source: PathBuf,
        dest: PathBuf,
        #[arg(long, value_enum, default_value_t = ZipProfileArg::Email)]
        profile: ZipProfileArg,
    },
    CreateSplit {
        source: PathBuf,
        dest: PathBuf,
        #[arg(long, value_enum, default_value_t = ZipProfileArg::Balanced)]
        profile: ZipProfileArg,
        #[arg(long, default_value_t = zip_io::SPLIT_ZIP_MIN_VOLUME_BYTES)]
        volume_size: u64,
    },
    Test { archive: PathBuf },
    Bench {
        #[arg(long)]
        fixture: Option<PathBuf>,
        #[arg(long)]
        all: bool,
        #[arg(long, value_enum, default_value_t = ZipProfileArg::Fast)]
        profile: ZipProfileArg,
        #[arg(long)]
        markdown: bool,
    },
}

#[derive(Clone, Copy, ValueEnum, Default)]
pub enum ZipProfileArg {
    Fast,
    Balanced,
    Maximum,
    #[default]
    Email,
}

impl From<ZipProfileArg> for CompressionProfile {
    fn from(p: ZipProfileArg) -> Self {
        match p {
            ZipProfileArg::Fast => CompressionProfile::Fast,
            ZipProfileArg::Balanced => CompressionProfile::Balanced,
            ZipProfileArg::Maximum => CompressionProfile::Maximum,
            ZipProfileArg::Email => CompressionProfile::EmailAttachment,
        }
    }
}

#[derive(Serialize)]
struct ListJson {
    archive: String,
    format: &'static str,
    entries: Vec<String>,
}

#[derive(Serialize)]
struct OkJson {
    ok: bool,
    action: &'static str,
}

#[derive(Serialize)]
struct SplitCreateJson {
    ok: bool,
    action: &'static str,
    volume_count: u32,
    total_bytes: u64,
}

pub fn run() -> Result<()> {
    let cli = ZipCli::parse();
    let json_out = cli.json;
    let pw = cli.password.as_deref();
    match cli.command {
        ZipCommands::List { archive } => {
            let entries = zip_io::list(&archive, pw)?;
            if json_out {
                print_json(&ListJson {
                    archive: archive.display().to_string(),
                    format: "zip",
                    entries,
                })?;
            } else {
                for name in entries {
                    println!("{name}");
                }
            }
        }
        ZipCommands::Extract { archive, dest } => zip_io::extract(&archive, &dest, pw)?,
        ZipCommands::Create {
            source,
            dest,
            profile,
        } => {
            zip_io::create_with_profile(&source, &dest, profile.into(), pw)?;
            if json_out {
                print_json(&OkJson {
                    ok: true,
                    action: "create",
                })?;
            }
        }
        ZipCommands::CreateSplit {
            source,
            dest,
            profile,
            volume_size,
        } => {
            let info = zip_io::create_split_with_profile(
                &source,
                &dest,
                profile.into(),
                pw,
                volume_size,
            )?;
            if json_out {
                print_json(&SplitCreateJson {
                    ok: true,
                    action: "create-split",
                    volume_count: info.volume_count,
                    total_bytes: info.total_bytes,
                })?;
            } else {
                println!(
                    "OK split: {} cilt, {} B",
                    info.volume_count, info.total_bytes
                );
            }
        }
        ZipCommands::Test { archive } => {
            zip_io::test(&archive, pw)?;
            if json_out {
                print_json(&OkJson {
                    ok: true,
                    action: "test",
                })?;
            } else {
                println!("OK");
            }
        }
        ZipCommands::Bench {
            fixture,
            all,
            profile,
            markdown,
        } => {
            let profile: CompressionProfile = profile.into();
            let cwd = std::env::current_dir().unwrap_or_default();
            let (platform, generated_at) = bench::suite_metadata();
            if all {
                let mut suite = BenchSuite {
                    profile: format!("{profile:?}"),
                    platform,
                    generated_at,
                    reports: bench::run_all(&cwd, profile)?,
                    kpi: None,
                    zip_kpi: None,
                };
                suite.kpi = Some(bench::evaluate_kpi(&suite));
                suite.zip_kpi = Some(bench::evaluate_zip_product_kpi(&suite));
                emit_bench(&suite, markdown, json_out)?;
            } else {
                let fixture = fixture.unwrap_or_else(|| bench::default_fixture(&cwd));
                let report = bench::run(&fixture, profile)?;
                let mut suite = BenchSuite {
                    profile: format!("{profile:?}"),
                    platform,
                    generated_at,
                    reports: vec![report],
                    kpi: None,
                    zip_kpi: None,
                };
                suite.kpi = Some(bench::evaluate_kpi(&suite));
                if markdown {
                    print!("{}", bench::suite_to_markdown(&suite));
                } else if json_out {
                    print_json(&suite.reports[0])?;
                } else {
                    print_bench_human(&suite.reports[0]);
                }
            }
        }
    }
    Ok(())
}

fn emit_bench(suite: &BenchSuite, markdown: bool, json_out: bool) -> Result<()> {
    if markdown {
        print!("{}", bench::suite_to_markdown(suite));
    } else if json_out {
        print_json(suite)?;
    }
    Ok(())
}

fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value).map_err(|e| {
        crate::ArchiveError::Zip(format!("json: {e}"))
    })?);
    Ok(())
}

fn print_bench_human(r: &BenchReport) {
    println!("fixture: {}", r.fixture);
    println!("profile: {}", r.profile);
    println!(
        "zip: {} bytes | create {} ms | extract {} ms",
        r.zip_bytes, r.zip_create_ms, r.zip_extract_ms
    );
}
