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
        help = "Path to the Kaguya vault directory. Overrides 'vault' value in the config file"
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
    /// Generate shell completion file of kaguya
    Completion,

    /// Managing global Kaguya config
    #[command(subcommand)]
    Config(ConfigSubcommands),

    /// Manage vault of kaguya
    #[command(subcommand)]
    Vault(VaultSubcommands),
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
        paths: Option<Vec<PathBuf>>,

        /// Comment for the game
        #[arg(short = 'o', long, value_name = "COMMENT")]
        comment: Option<String>,
    },

    /// Print game backup config information
    List {
        /// Print detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },

    /// Remove existing game config, retain backups at default
    Rm {
        /// Game ID
        #[arg(short = 'i', long)]
        id: String,

        /// Cascade delete all backups associated with the game config
        #[arg(short = 'r', long)]
        purge: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum VaultSubcommands {
    /// Backup games saves and configurations to vault
    Backup {
        /// Game ID
        /// leave empty to backup globally
        #[arg(short, long)]
        id: Option<String>,

        /// Game saves and configurations paths (use with '--id/-i <ID>')
        /// Leave empty to backup all paths in the config
        #[arg(short, long, action = clap::ArgAction::Append, requires = "id")]
        paths: Option<Vec<PathBuf>>,
    },

    /// Restore saves and configurations from backups in vault
    Restore {
        /// Game ID
        #[arg(short, long)]
        id: String,

        /// Backup version (default: latest)
        #[arg(short, long)]
        version: Option<String>,

        /// Game saves and configurations paths (use with '--id/-i <ID>')
        /// Leave empty to restore all paths in the config
        #[arg(short, long, action = clap::ArgAction::Append, requires = "id")]
        paths: Vec<PathBuf>,
    },

    /// Prune old backups based on retention policy,
    /// or delete specific backups
    Prune {
        /// Game ID (leave empty to prune globally)
        #[arg(short, long)]
        id: Option<String>,

        /// Specific a backup version to delete (use with '--id/-d <ID>')
        #[arg(short, long, requires = "id")]
        version: Option<String>,

        /// Delete all the backup versions of the game (use with '--id/-d <ID>')
        #[arg(short, long, requires = "id")]
        purge: bool,
    },

    /// Print backup, restore and prune history
    History {
        /// Game ID (leave empty for all games)
        #[arg(short, long)]
        id: Option<String>,
    },

    /// Check integrity of all backups
    /// (verify file existence and hash consistency and metadata validity)
    Check,
}
