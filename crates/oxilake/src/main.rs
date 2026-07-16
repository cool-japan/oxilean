use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cache;
mod commands;
mod error;
mod lockfile;
mod manifest;
mod resolver;
mod workspace;

#[derive(Parser)]
#[command(
    name = "oxilake",
    about = "Package manager for OxiLean projects",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new OxiLean package
    New {
        /// Package name
        name: String,
        /// Target directory (defaults to `<name>`)
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Build the current package
    Build {
        /// Path to oxilake.toml (default: ./oxilake.toml)
        #[arg(long, default_value = "oxilake.toml")]
        manifest: PathBuf,
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Type-check the current package without emitting output artifacts
    Check {
        /// Path to oxilake.toml (default: ./oxilake.toml)
        #[arg(long, default_value = "oxilake.toml")]
        manifest: PathBuf,
        /// Print each declaration as it is verified
        #[arg(long)]
        verbose: bool,
    },
    /// Run all checks and tests for the current package
    Test {
        /// Path to oxilake.toml (default: ./oxilake.toml)
        #[arg(long, default_value = "oxilake.toml")]
        manifest: PathBuf,
    },
    /// Build and run the Main declaration of the current package
    Run {
        /// Path to oxilake.toml (default: ./oxilake.toml)
        #[arg(long, default_value = "oxilake.toml")]
        manifest: PathBuf,
    },
    /// Format all .lean source files in the current package
    Fmt {
        /// Path to oxilake.toml (default: ./oxilake.toml)
        #[arg(long, default_value = "oxilake.toml")]
        manifest: PathBuf,
        /// Check formatting without writing changes (exits non-zero if any file needs formatting)
        #[arg(long)]
        check: bool,
    },
    /// Remove cached build artifacts
    Clean {
        /// Remove only artifacts for this package name (removes all if omitted)
        #[arg(long)]
        package: Option<String>,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name, path } => commands::new::run(&name, path),
        Commands::Build { manifest, release } => commands::build::run(&manifest, release),
        Commands::Check { manifest, verbose } => commands::check::run(&manifest, verbose),
        Commands::Test { manifest } => commands::test::run(&manifest),
        Commands::Run { manifest } => commands::run::run(&manifest),
        Commands::Fmt { manifest, check } => commands::fmt::run(&manifest, check),
        Commands::Clean { package } => commands::clean::run(package.as_deref()),
    }
}
