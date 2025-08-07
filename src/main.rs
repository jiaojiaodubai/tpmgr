use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod config;
mod package;
mod resolver;
mod error;
mod mirror;
mod texlive;
mod tex_parser;

use commands::*;

#[derive(Parser)]
#[command(name = "tpmgr")]
#[command(about = "A lightweight LaTeX package manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new LaTeX project with package management
    Init {
        /// Project name (optional, if not provided, initializes in current directory)
        name: Option<String>,
    },
    /// Install packages
    Install {
        /// Package names to install (if empty, scan and install missing packages)
        packages: Vec<String>,
        /// Install packages globally
        #[arg(short, long)]
        global: bool,
        /// Path to TeX file or project directory for auto-detection
        #[arg(short, long, default_value = ".")]
        path: String,
        /// Use compilation errors to detect missing packages
        #[arg(short, long)]
        compile: bool,
    },
    /// Remove packages
    Remove {
        /// Package names to remove
        packages: Vec<String>,
        /// Remove packages globally
        #[arg(short, long)]
        global: bool,
    },
    /// Update packages
    Update {
        /// Package names to update (all if not specified)
        packages: Vec<String>,
    },
    /// List installed packages
    List {
        /// Show global packages
        #[arg(short, long)]
        global: bool,
    },
    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
    /// Show package information
    Info {
        /// Package name
        package: String,
    },
    /// Mirror management
    Mirror {
        #[command(subcommand)]
        action: MirrorAction,
    },
    /// Analyze TeX file dependencies
    Analyze {
        /// Path to TeX file or project directory
        #[arg(short, long, default_value = ".")]
        path: String,
        /// Show detailed dependency information
        #[arg(short, long)]
        verbose: bool,
        /// Use compilation errors to detect missing packages
        #[arg(short, long)]
        compile: bool,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Compile LaTeX project using predefined compilation chain
    Compile {
        /// Path to project directory or TeX file
        #[arg(short, long, default_value = ".")]
        path: String,
        /// Clean intermediate files after compilation
        #[arg(short = 'c', long)]
        clean: bool,
        /// Show verbose compilation output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show {
        /// Show global configuration only
        #[arg(long, short)]
        global: bool,
    },
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
        /// Set global configuration
        #[arg(long, short)]
        global: bool,
    },
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
        /// Get from global configuration only
        #[arg(long, short)]
        global: bool,
    },
    /// List all configuration keys
    List {
        /// Show global configuration keys only
        #[arg(long, short)]
        global: bool,
    },
    /// Reset configuration to defaults
    Reset {
        /// Reset global configuration only
        #[arg(long, short)]
        global: bool,
    },
}

#[derive(Subcommand)]
enum MirrorAction {
    /// List available mirrors
    List,
    /// Use a specific mirror or auto-select the best one
    Use {
        /// Mirror name (optional if using --auto)
        name: Option<String>,
        /// Auto-select the best mirror based on speed
        #[arg(short, long)]
        auto: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize global configuration on first run
    if let Err(e) = commands::ensure_global_config_initialized().await {
        eprintln!("Warning: Failed to initialize global configuration: {}", e);
    }

    match &cli.command {
        Some(Commands::Init { name }) => init_command(name.clone()).await,
        Some(Commands::Install { packages, global, path, compile }) => {
            install_command(packages, *global, path, *compile).await
        },
        Some(Commands::Remove { packages, global }) => remove_command(packages, *global).await,
        Some(Commands::Update { packages }) => update_command(packages).await,
        Some(Commands::List { global }) => list_command(*global).await,
        Some(Commands::Search { query }) => search_command(query).await,
        Some(Commands::Info { package }) => info_command(package).await,
        Some(Commands::Mirror { action }) => mirror_command(action).await,
        Some(Commands::Analyze { path, verbose, compile }) => {
            analyze_command(path, *verbose, *compile).await
        },
        Some(Commands::Config { action }) => config_command(action).await,
        Some(Commands::Compile { path, clean, verbose }) => {
            compile_command(path, *clean, *verbose).await
        },
        None => {
            println!("tpmgr - LaTeX Package Manager");
            println!("Use 'tpmgr --help' for more information.");
            Ok(())
        }
    }
}
