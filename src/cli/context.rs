//! Save some global options for config and bakcup services

use std::path::PathBuf;

use crate::{
    cli::Cli,
    models::{DB_FILE, GAMES_FILE, KaguyaError},
    utils::path::{get_config_path, get_vault_path},
};

/// Represents the parsed and resolved global context for the application.
/// This struct holds global values like the vault path, dry-run flag, etc.
/// Also, it generates a database connection.
#[derive(Debug, Clone)]
pub struct AppContext {
    pub vault_path: PathBuf,
    pub games_path: PathBuf,
    pub config_path: PathBuf,
    pub db_path: PathBuf,
    pub dry_run: bool,
}

impl AppContext {
    pub fn new(cli: &Cli) -> Result<Self, KaguyaError> {
        let vault_path = get_vault_path(&cli.vault)?;
        let config_path = get_config_path(&cli.config)?;
        let games_path = vault_path.join(GAMES_FILE);
        let db_path = vault_path.join(DB_FILE);

        Ok(Self {
            vault_path,
            games_path,
            config_path,
            db_path,
            dry_run: cli.dry_run,
        })
    }
}
