//! Coordinates the synchronization between the database and the vault config file.
//!
//! This module defines the [`DbManagerSyncExt`] trait, which provides the main
//! [`sync`](DbManagerSyncExt::sync) method. This method acts as an orchestrator,
//! checking file hashes and then calling methods from other modules to upsert
//! games and prune obsolete records.

use super::{DbManager, DbManagerGameExt, DbManagerGamePathExt, DbManagerMetaExt};
use crate::{
    db_manager::toml::read_toml_file,
    fs_utils::hash::calculate_entry_checksum,
    models::{KEY_VAULT_CONFIG_HASH, KaguyaError},
};
use std::path::Path;

pub trait DbManagerSyncExt {
    fn sync(
        &mut self,
        vault_config_path: &impl AsRef<Path>,
        force: bool,
    ) -> Result<(), KaguyaError>;
}

impl DbManagerSyncExt for DbManager {
    // Sync database if vault config have been changed
    // Do nothing if vault config doesn't exist
    fn sync(
        &mut self,
        vault_config_path: &impl AsRef<Path>,
        force: bool,
    ) -> Result<(), KaguyaError> {
        let file_hash = calculate_entry_checksum(vault_config_path).ok();
        let db_hash_record = self.get_meta_value(KEY_VAULT_CONFIG_HASH).ok();

        if (force && file_hash.is_some()) || (!force && file_hash != db_hash_record) {
            println!("Vault config has been changed, syncing database...");
            self.perform_sync(
                vault_config_path,
                file_hash.expect("file_hash should be exist."),
            )?;
        }
        Ok(())
    }
}

impl DbManager {
    // Sync database with vault config
    fn perform_sync(
        &mut self,
        vault_config_path: &impl AsRef<Path>,
        new_hash: String,
    ) -> Result<(), KaguyaError> {
        let vault_config_file = read_toml_file(vault_config_path)?;

        self.upsert_games_from_config(&vault_config_file)?;
        self.prune_obsolete_games(&vault_config_file)?;
        self.prune_obsolete_paths(&vault_config_file)?;

        self.update_meta_value(KEY_VAULT_CONFIG_HASH, &new_hash)?;

        println!("Database synced successfully.\n");
        Ok(())
    }
}
