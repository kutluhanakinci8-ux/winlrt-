use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use winlrt_archive::{profile::CompressionProfile, wlrt, zip_io, Result};

#[derive(Parser)]
#[command(name = "winlrt-archive", about = "winlrt arşiv CLI — winlrtRAR motoru")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// ZIP arşiv içeriğini listele
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
    /// Benchmark (Faz 1 — genişletilecek)
    Bench {
        #[arg(long)]
        fixture: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum FormatArg {
    Zip,
    Wlrt,
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

fn main() {
    if let Err(e) = run() {
        eprintln!("hata: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { archive, format } => match format {
            FormatArg::Zip => {
                for name in zip_io::list(&archive)? {
                    println!("{name}");
                }
            }
            FormatArg::Wlrt => {
                for name in wlrt::list(&archive)? {
                    println!("{name}");
                }
            }
        },
        Commands::Extract { archive, dest, format } => match format {
            FormatArg::Zip => zip_io::extract(&archive, &dest)?,
            FormatArg::Wlrt => wlrt::extract(&archive, &dest)?,
        },
        Commands::Create {
            source,
            dest,
            format,
            profile,
        } => {
            let _profile: CompressionProfile = profile.into();
            match format {
                FormatArg::Zip => zip_io::create(&source, &dest)?,
                FormatArg::Wlrt => wlrt::create(&source, &dest)?,
            }
        }
        Commands::Test { archive, format } => match format {
            FormatArg::Zip => zip_io::test(&archive)?,
            FormatArg::Wlrt => wlrt::test(&archive)?,
        },
        Commands::Bench { fixture } => {
            println!("benchmark: Faz 1 — fixture={:?}", fixture);
            println!("KPI (P0): hız | sıkıştırma | kurumsal güven — benchmarks/README.md");
        }
    }
    Ok(())
}
