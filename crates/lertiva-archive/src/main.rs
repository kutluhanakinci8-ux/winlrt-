use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use lertiva_archive::bench::{self, BenchReport, BenchSuite};
use lertiva_archive::profile::CompressionProfile;
use lertiva_archive::{rar, sevenz, volumes, wlrt, zip_io, Result};

#[derive(Parser)]
#[command(name = "lertiva-archive", about = "Lertiva arşiv CLI — Lertiva Arc / Lertiva Zip motoru")]
struct Cli {
    /// JSON çıktı (list / test / bench)
    #[arg(long, global = true)]
    json: bool,

    /// RAR parolası (Faz 2)
    #[arg(long, global = true)]
    password: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Arşiv içeriğini listele
    List {
        archive: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Zip)]
        format: FormatArg,
    },
    /// Arşivi hedef klasöre aç
    Extract {
        archive: PathBuf,
        dest: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Zip)]
        format: FormatArg,
    },
    /// Klasör veya dosyadan arşiv oluştur
    Create {
        source: PathBuf,
        dest: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Zip)]
        format: FormatArg,
        #[arg(long, value_enum, default_value_t = ProfileArg::Balanced)]
        profile: ProfileArg,
    },
    /// Bütünlük testi
    Test {
        archive: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Zip)]
        format: FormatArg,
    },
    /// Çok cilt / birleşik arşiv bilgisi
    Inspect {
        archive: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Rar)]
        format: FormatArg,
    },
    /// RAR kurtarma onarımı (UnRAR `r`, beta)
    Repair {
        archive: PathBuf,
        dest: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Rar)]
        format: FormatArg,
    },
    /// ZIP vs `.wlrt` süre ve boyut (Faz 1 KPI hazırlığı)
    Bench {
        #[arg(long)]
        fixture: Option<PathBuf>,
        /// benchmarks/fixtures altındaki tüm setler
        #[arg(long)]
        all: bool,
        #[arg(long, value_enum, default_value_t = ProfileArg::Balanced)]
        profile: ProfileArg,
        /// Markdown tablo (vitrin / PR özeti)
        #[arg(long)]
        markdown: bool,
        /// Lertiva Zip ürün KPI (`zip_kpi` alanı)
        #[arg(long)]
        zip_kpi: bool,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum FormatArg {
    Zip,
    Wlrt,
    Rar,
    #[value(name = "7z")]
    SevenZ,
}

#[derive(Clone, Copy, ValueEnum, Default)]
enum ProfileArg {
    Fast,
    #[default]
    Balanced,
    Maximum,
    Email,
}

impl From<ProfileArg> for CompressionProfile {
    fn from(p: ProfileArg) -> Self {
        match p {
            ProfileArg::Fast => CompressionProfile::Fast,
            ProfileArg::Balanced => CompressionProfile::Balanced,
            ProfileArg::Maximum => CompressionProfile::Maximum,
            ProfileArg::Email => CompressionProfile::EmailAttachment,
        }
    }
}

#[derive(Serialize)]
struct ListJson {
    archive: String,
    format: String,
    entries: Vec<String>,
}

#[derive(Serialize)]
struct OkJson {
    ok: bool,
    action: &'static str,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("hata: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let json_out = cli.json;
    let password = cli.password.clone();
    let pw = password.as_deref();
    match cli.command {
        Commands::List { archive, format } => {
            let entries = match format {
                FormatArg::Zip => zip_io::list(&archive, pw)?,
                FormatArg::Wlrt => wlrt::list(&archive)?,
                FormatArg::Rar => rar::list(&archive, pw)?,
                FormatArg::SevenZ => sevenz::list(&archive, pw)?,
            };
            if json_out {
                print_json(&ListJson {
                    archive: archive.display().to_string(),
                    format: format_label(format),
                    entries,
                })?;
            } else {
                for name in entries {
                    println!("{name}");
                }
            }
        }
        Commands::Extract { archive, dest, format } => match format {
            FormatArg::Zip => zip_io::extract(&archive, &dest, pw)?,
            FormatArg::Wlrt => wlrt::extract(&archive, &dest, pw)?,
            FormatArg::Rar => rar::extract(&archive, &dest, pw)?,
            FormatArg::SevenZ => sevenz::extract(&archive, &dest, pw)?,
        },
        Commands::Create {
            source,
            dest,
            format,
            profile,
        } => {
            let profile: CompressionProfile = profile.into();
            match format {
                FormatArg::Zip => zip_io::create_with_profile(&source, &dest, profile, pw)?,
                FormatArg::Wlrt => wlrt::create(&source, &dest, profile, pw)?,
                FormatArg::Rar => {
                    return Err(lertiva_archive::ArchiveError::Rar(
                        "RAR yazma desteklenmiyor".into(),
                    ))
                }
                FormatArg::SevenZ => {
                    return Err(lertiva_archive::ArchiveError::SevenZ(
                        "7z yazma Faz 5 planında".into(),
                    ))
                }
            }
        }
        Commands::Test { archive, format } => {
            match format {
                FormatArg::Zip => zip_io::test(&archive, pw)?,
                FormatArg::Wlrt => wlrt::test(&archive, pw)?,
                FormatArg::Rar => rar::test(&archive, pw)?,
                FormatArg::SevenZ => sevenz::test(&archive, pw)?,
            }
            if json_out {
                print_json(&OkJson {
                    ok: true,
                    action: "test",
                })?;
            } else {
                println!("OK");
            }
        }
        Commands::Inspect { archive, format } => {
            let info = match format {
                FormatArg::Rar => rar::inspect(&archive),
                _ => volumes::VolumeInfo {
                    multipart: false,
                    open_path: archive.display().to_string(),
                    selected_path: archive.display().to_string(),
                    format: "other",
                    hint: "Çok cilt algılama şu an yalnızca RAR için.".into(),
                    can_repair: false,
                },
            };
            if json_out {
                print_json(&info)?;
            } else if info.hint.is_empty() {
                println!("Tek parça arşiv: {}", info.open_path);
            } else {
                println!("{}", info.hint);
                println!("Açılacak yol: {}", info.open_path);
            }
        }
        Commands::Repair { archive, dest, format } => {
            match format {
                FormatArg::Rar => rar::repair(&archive, &dest, pw)?,
                _ => {
                    return Err(lertiva_archive::ArchiveError::NotYet(
                        "onarım yalnızca RAR (beta)",
                    ))
                }
            }
            if json_out {
                print_json(&OkJson {
                    ok: true,
                    action: "repair",
                })?;
            } else {
                println!("Onarım OK → {}", dest.display());
            }
        }
        Commands::Bench {
            fixture,
            all,
            profile,
            markdown,
            zip_kpi,
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
                if zip_kpi {
                    suite.zip_kpi = Some(bench::evaluate_zip_product_kpi(&suite));
                }
                if markdown {
                    print!("{}", bench::suite_to_markdown(&suite));
                } else if json_out {
                    print_json(&suite)?;
                } else {
                    for report in &suite.reports {
                        print_bench_human(report);
                        println!("---");
                    }
                }
            } else {
                let fixture = fixture.unwrap_or_else(|| bench::default_fixture(&cwd));
                let report = bench::run(&fixture, profile)?;
                if markdown {
                    let mut suite = BenchSuite {
                        profile: format!("{profile:?}"),
                        platform,
                        generated_at,
                        reports: vec![report],
                        kpi: None,
                        zip_kpi: None,
                    };
                    suite.kpi = Some(bench::evaluate_kpi(&suite));
                    if zip_kpi {
                        suite.zip_kpi = Some(bench::evaluate_zip_product_kpi(&suite));
                    }
                    print!("{}", bench::suite_to_markdown(&suite));
                } else if json_out {
                    print_json(&report)?;
                } else {
                    print_bench_human(&report);
                }
            }
        }
    }
    Ok(())
}

fn format_label(format: FormatArg) -> String {
    match format {
        FormatArg::Zip => "zip".into(),
        FormatArg::Wlrt => "wlrt".into(),
        FormatArg::Rar => "rar".into(),
        FormatArg::SevenZ => "7z".into(),
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value).map_err(|e| {
        lertiva_archive::ArchiveError::Wlrt(format!("json: {e}"))
    })?);
    Ok(())
}

fn print_bench_human(r: &BenchReport) {
    println!("fixture: {}", r.fixture);
    println!("profile: {}", r.profile);
    println!("raw_bytes: {}", r.raw_bytes);
    println!("zip: {} bytes | create {} ms | extract {} ms", r.zip_bytes, r.zip_create_ms, r.zip_extract_ms);
    println!(
        "wlrt: {} bytes | create {} ms | extract {} ms",
        r.wlrt_bytes, r.wlrt_create_ms, r.wlrt_extract_ms
    );
    if r.wlrt_bytes > 0 && r.zip_bytes > 0 {
        let ratio = (1.0 - (r.wlrt_bytes as f64 / r.zip_bytes as f64)) * 100.0;
        println!("wlrt vs zip boyut farkı: {ratio:.1}%");
    }
    if let Some(b) = r.seven_zip_bytes {
        println!("7-Zip: {b} bytes");
    }
    if let Some(ms) = r.seven_zip_create_ms {
        println!("7-Zip create: {ms} ms");
    }
    if let Some(b) = r.winrar_bytes {
        println!("WinRAR ZIP: {b} bytes");
    }
    if let Some(ms) = r.winrar_create_ms {
        println!("WinRAR create: {ms} ms");
    }
    if let Some(b) = r.winzip_bytes {
        println!("WinZip: {b} bytes");
    }
    if let Some(ms) = r.winzip_create_ms {
        println!("WinZip create: {ms} ms");
    }
}
