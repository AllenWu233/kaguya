//! Coordinates the synchronization between the database and the games config file.
//!
//! This module defines the [`DbManagerSyncExt`] trait, which provides the main
//! [`sync`](DbManagerSyncExt::sync) method. This method acts as an orchestrator,
//! checking file hashes and then calling methods from other modules to upsert
//! games and prune obsolete records.

use super::{DbManager, DbManagerGameExt, DbManagerGamePathExt, DbManagerMetaExt};
use crate::{
    db_manager::toml::read_game_config_file, fs_utils::hash::calculate_file_hash,
    models::KaguyaError,
};
use std::path::Path;

pub trait DbManagerSyncExt {
    fn sync(&mut self, game_config_path: &impl AsRef<Path>, force: bool)
    -> Result<(), KaguyaError>;
}

impl DbManagerSyncExt for DbManager {
    // Sync database if game config file have been changed
    // Do nothing if game config file doesn't exist
    fn sync(
        &mut self,
        game_config_path: &impl AsRef<Path>,
        force: bool,
    ) -> Result<(), KaguyaError> {
        let file_hash = calculate_file_hash(game_config_path).ok();
        let db_hash_record = self.get_meta_value("game_config_file_hash").ok();

        if (force && file_hash.is_some()) || (!force && file_hash != db_hash_record) {
            println!("Game config file has changed, syncing database...");
            self.perform_sync(
                game_config_path,
                file_hash.expect("file_hash should be exist."),
            )?;
        }
        Ok(())
    }
}

impl DbManager {
    // Sync database with game config file
    fn perform_sync(
        &mut self,
        game_config_path: &impl AsRef<Path>,
        new_hash: String,
    ) -> Result<(), KaguyaError> {
        let game_config_file = read_game_config_file(game_config_path)?;

        self.upsert_games_from_config(&game_config_file)?;
        self.prune_obsolete_games(&game_config_file)?;
        self.prune_obsolete_paths(&game_config_file)?;

        // Update game_config_file_hash in the database
        self.update_meta_value("game_config_file_hash", &new_hash)?;

        println!("Database synced successfully.\n");
        Ok(())
    }
}
