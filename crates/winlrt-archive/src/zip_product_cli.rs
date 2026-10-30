//! winlrtZIP — ZIP odaklı CLI (Faz 1).

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

use crate::bench::{self, BenchReport, BenchSuite};
use crate::profile::CompressionProfile;
use crate::archive_sign;
use crate::lerta_mail;
use crate::mobile_reader;
use crate::secure_share;
use crate::store_gate;
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
    Sign {
        archive: PathBuf,
        /// 32 bayt Ed25519 seed (hex); yoksa `WINLRT_SIGNING_SEED_HEX`.
        #[arg(long)]
        signing_seed_hex: Option<String>,
    },
    VerifySig { archive: PathBuf },
    SharePack {
        source: PathBuf,
        dest: PathBuf,
        #[arg(long)]
        self_hosted_base: Option<String>,
        #[arg(long)]
        max_retention_days: Option<u32>,
    },
    VerifyShare { pack_zip: PathBuf },
    MobileCapabilities,
    MobileInspect { archive: PathBuf },
    StoreGate {
        #[arg(long)]
        workspace: Option<PathBuf>,
    },
    LertaMailPrepare {
        source: PathBuf,
        dest: PathBuf,
        #[arg(long, value_enum, default_value_t = LertaTierArg::Standard10Mb)]
        tier: LertaTierArg,
    },
    LertaMailValidate {
        zip: PathBuf,
        #[arg(long, value_enum, default_value_t = LertaTierArg::Standard10Mb)]
        tier: LertaTierArg,
    },
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
pub enum LertaTierArg {
    #[default]
    Standard10Mb,
    Extended25Mb,
}

impl From<LertaTierArg> for lerta_mail::LertaAttachmentTier {
    fn from(t: LertaTierArg) -> Self {
        match t {
            LertaTierArg::Standard10Mb => lerta_mail::LertaAttachmentTier::Standard10Mb,
            LertaTierArg::Extended25Mb => lerta_mail::LertaAttachmentTier::Extended25Mb,
        }
    }
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

#[derive(Serialize)]
struct SignJson {
    ok: bool,
    action: &'static str,
    sidecar: String,
}

#[derive(Serialize)]
struct SharePackJson {
    ok: bool,
    action: &'static str,
    pack_id: String,
    zip: String,
    manifest: String,
}

#[derive(Serialize)]
struct VerifyShareJson {
    ok: bool,
    action: &'static str,
    pack_id: String,
    kvkk_local_only: bool,
}

fn parse_seed_hex(hex: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(hex.trim())
        .map_err(|e| crate::ArchiveError::Zip(format!("seed hex: {e}")))?;
    bytes
        .try_into()
        .map_err(|_| crate::ArchiveError::Zip("seed 32 bayt olmali".into()))
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
        ZipCommands::Sign {
            archive,
            signing_seed_hex,
        } => {
            let seed_hex = signing_seed_hex.or_else(|| {
                std::env::var("WINLRT_SIGNING_SEED_HEX")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            });
            let Some(seed_hex) = seed_hex else {
                return Err(crate::ArchiveError::Zip(
                    "imza seed gerekli (--signing-seed-hex veya WINLRT_SIGNING_SEED_HEX)".into(),
                ));
            };
            let seed = parse_seed_hex(&seed_hex)?;
            let key = archive_sign::signing_key_from_seed(seed);
            let sidecar = zip_io::sign_zip_archive(&archive, &key)?;
            if json_out {
                print_json(&SignJson {
                    ok: true,
                    action: "sign",
                    sidecar: sidecar.display().to_string(),
                })?;
            } else {
                println!("OK imza: {}", sidecar.display());
            }
        }
        ZipCommands::VerifySig { archive } => {
            zip_io::verify_zip_archive_signature(&archive)?;
            if json_out {
                print_json(&OkJson {
                    ok: true,
                    action: "verify-sig",
                })?;
            } else {
                println!("OK imza dogrulandi");
            }
        }
        ZipCommands::SharePack {
            source,
            dest,
            self_hosted_base,
            max_retention_days,
        } => {
            let password = pw.ok_or_else(|| {
                crate::ArchiveError::Zip(
                    "share-pack icin --password gerekli (min 8 karakter)".into(),
                )
            })?;
            let result = secure_share::create_secure_share_pack(
                &source,
                &dest,
                password,
                self_hosted_base.as_deref(),
                max_retention_days,
            )?;
            if json_out {
                print_json(&SharePackJson {
                    ok: true,
                    action: "share-pack",
                    pack_id: result.pack_id,
                    zip: result.zip_path.display().to_string(),
                    manifest: result.manifest_path.display().to_string(),
                })?;
            } else {
                println!(
                    "OK paylasim paketi: {} (manifest: {}, id: {})",
                    result.zip_path.display(),
                    result.manifest_path.display(),
                    result.pack_id
                );
            }
        }
        ZipCommands::VerifyShare { pack_zip } => {
            let manifest = secure_share::verify_secure_share_pack(&pack_zip, pw)?;
            if json_out {
                print_json(&VerifyShareJson {
                    ok: true,
                    action: "verify-share",
                    pack_id: manifest.pack_id,
                    kvkk_local_only: manifest.kvkk_local_only,
                })?;
            } else {
                println!("OK KVKK paylasim paketi: {}", manifest.pack_id);
            }
        }
        ZipCommands::MobileCapabilities => {
            print_json(&mobile_reader::capabilities())?;
        }
        ZipCommands::MobileInspect { archive } => {
            let report = mobile_reader::inspect(&archive, pw)?;
            if json_out {
                print_json(&report)?;
            } else {
                println!(
                    "mobil: {} girdi, sifreli={}, split={}",
                    report.entry_count, report.encryption_detected, report.split_zip
                );
            }
        }
        ZipCommands::StoreGate { workspace } => {
            let root = workspace.unwrap_or_else(store_gate::default_workspace_root);
            let report = store_gate::evaluate_store_gate(&root)?;
            if !json_out {
                for line in &report.items {
                    println!(
                        "[{}] {} — {}",
                        if line.pass { "OK" } else { "FAIL" },
                        line.id,
                        line.note
                    );
                }
                println!(
                    "store gate: all_pass={} version={}",
                    report.all_pass, report.app_version
                );
            } else {
                print_json(&report)?;
            }
            if !report.all_pass {
                return Err(crate::ArchiveError::Zip(
                    "magaza kapisi: eksik maddeler var".into(),
                ));
            }
        }
        ZipCommands::LertaMailPrepare {
            source,
            dest,
            tier,
        } => {
            let prep = lerta_mail::prepare_lerta_mail_attachment(
                &source,
                &dest,
                tier.into(),
                pw,
            )?;
            if json_out {
                print_json(&lerta_mail::webhook_payload_from_prepare(&prep))?;
            } else {
                println!(
                    "OK Lerta eki: {} ({} B, sha256={})",
                    prep.filename,
                    prep.size_bytes,
                    &prep.sha256_hex[..16]
                );
            }
        }
        ZipCommands::LertaMailValidate { zip, tier } => {
            lerta_mail::validate_lerta_mail_attachment(&zip, tier.into())?;
            if json_out {
                print_json(&OkJson {
                    ok: true,
                    action: "lerta-mail-validate",
                })?;
            } else {
                println!("OK Lerta posta eki limiti");
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
