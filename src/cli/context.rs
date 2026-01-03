//! Save some global options for config and bakcup services

use std::path::PathBuf;

use crate::{
    cli::Cli,
    models::{BACKUP_DIR, DB_FILE, KaguyaError, VAULT_CONFIG_FILE},
    utils::path::{get_global_config_path, get_vault_dir},
};

/// Represents the parsed and resolved global context for the application.
#[derive(Debug, Clone)]
pub struct AppContext {
    pub global_config_path: PathBuf,
    pub vault_dir: PathBuf,
    pub vault_config_path: PathBuf,
    pub backup_dir: PathBuf,
    pub db_path: PathBuf,
    pub dry_run: bool,
}

impl AppContext {
    pub fn new(cli: &Cli) -> Result<Self, KaguyaError> {
        let global_config_path = get_global_config_path(&cli.config)?;
        let vault_dir = get_vault_dir(&cli.vault)?;
        let vault_config_path = vault_dir.join(VAULT_CONFIG_FILE);
        let backup_dir = vault_dir.join(BACKUP_DIR);
        let db_path = vault_dir.join(DB_FILE);

        Ok(Self {
            vault_dir,
            vault_config_path,
            global_config_path,
            backup_dir,
            db_path,
            dry_run: cli.dry_run,
        })
    }
}
