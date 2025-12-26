//! Parse CLI subcommands and arguments

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI tool for managing Linux game saves and configurations.
#[derive(Parser, Debug)]
#[command(version, about, author)]
pub struct Cli {
    /// Path to the global Kaguya configuration file.
    /// If not provided, defaults to the standard XDG location.
    #[arg(
        short = 'c',
        long,
        value_name = "FILE",
        help = "Path to the global Kaguya configuration file"
    )]
    pub config: Option<PathBuf>,

    /// Path to the Kaguya vault directory.
    /// This setting overrides the path defined in the global configuration file.
    #[arg(
        short = 'v',
        long,
        value_name = "DIR",
        help = "Path to the Kaguya vault directory. Overrides the 'vault_path' in the config file"
    )]
    pub vault: Option<PathBuf>,

    /// Run a command without making actual changes.
    #[arg(short = 'n', long, action = clap::ArgAction::SetTrue, global = true)]
    pub dry_run: bool,

    /// Subcommands to execute.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize config, database and vault for kaguya
    Init,

    /// Generate shell completion file of kaguya
    Completion,

    /// Managing global Kaguya config
    #[command(subcommand)]
    Config(ConfigSubcommands),
    // /// Manage vault of kaguya
    // #[command(subcommand)]
    // Vault(VaultSubcommands),
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubcommands {
    /// Add a new game configuration
    Add {
        /// A unique identifier for the game (e.g., 'outer_wilds')
        #[arg(short = 'i', long, value_name = "ID")]
        id: String,

        /// A friendly name for the game
        #[arg(short = 'a', long, value_name = "NAME")]
        name: Option<String>,

        /// One or more paths to save files or directories
        #[arg(short = 'p', long, value_name = "PATH", num_args = 1..)]
        paths: Vec<PathBuf>,

        /// Comment for the game
        #[arg(short = 'c', long, value_name = "COMMENT")]
        comment: Option<String>,
    },

    /// Print game backup config information
    List {
        /// Print detailed information
        #[arg(short, long)]
        long: bool,
    },

    /// Remove existing game config, retain backups at default
    Rm {
        /// Game ID
        #[arg(short, long)]
        id: String,

        /// Cascade delete all backups associated with the game config
        #[arg(short, long)]
        purge: bool,
    },
}
