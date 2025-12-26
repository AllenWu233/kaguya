//! Save some global options for config and bakcup services

use std::path::PathBuf;

use crate::{
    cli::Cli,
    core::utils::{get_config_path, get_vault_path},
    models::{GAMES_FILE, KaguyaError},
};

/// Represents the parsed and resolved global context for the application.
/// This struct holds global values like the vault path, dry-run flag, etc.
#[derive(Debug)]
pub struct AppContext {
    pub vault_path: PathBuf,
    pub games_path: PathBuf,
    pub config_path: PathBuf,
    pub dry_run: bool,
}

impl AppContext {
    pub fn from_cli(cli: &Cli) -> Result<Self, KaguyaError> {
        let vault_path = get_vault_path(&cli.vault)?;
        let config_path = get_config_path(&cli.config)?;
        let games_path = vault_path.join(GAMES_FILE);

        Ok(Self {
            vault_path,
            games_path,
            config_path,
            dry_run: cli.dry_run,
        })
    }
}
