//! Provides CRUD operations for the `game_path` database table.
//!
//! This module defines the [`DbManagerGamePathExt`] trait, which extends the
//! [`DbManager`] with methods to manage paths for a specific game. It includes
//! functionality for upserting multiple paths and pruning obsolete ones.

use super::DbManager;
use crate::models::{KaguyaError, VaultConfig, db::DbPathInfo};
use std::{collections::HashSet, path::PathBuf};

pub trait DbManagerGamePathExt {
    fn prune_obsolete_paths(
        &mut self,
        vault_config_file: &VaultConfig,
    ) -> Result<Vec<(String, String)>, KaguyaError>;

    fn get_all_db_paths(&self) -> Result<Vec<DbPathInfo>, KaguyaError>;

    fn upsert_paths(&mut self, game_id: i64, paths: &[PathBuf]) -> Result<(), KaguyaError>;
}

impl DbManagerGamePathExt for DbManager {
    // Prune database of game paths not found int the vault config.
    fn prune_obsolete_paths(
        &mut self,
        vault_config_file: &VaultConfig,
    ) -> Result<Vec<(String, String)>, KaguyaError> {
        let db_paths = self.get_all_db_paths()?;
        let config_paths: HashSet<(String, String)> = vault_config_file
            .games
            .iter()
            .flat_map(|game_config| {
                game_config
                    .paths
                    .iter()
                    .map(|path| (game_config.id.clone(), path.to_string_lossy().into_owned()))
            })
            .collect();

        let mut pruned_paths = Vec::new();

        for db_path_info in db_paths {
            let key = (
                db_path_info.external_id.clone(),
                db_path_info.original_path.clone(),
            );

            if !config_paths.contains(&key) {
                let sql = "DELETE FROM game_path WHERE id = ?1";
                self.conn.execute(sql, [db_path_info.id])?;

                pruned_paths.push((
                    db_path_info.external_id.clone(),
                    db_path_info.original_path.clone(),
                ));

                println!(
                    "Pruned path '{}' of game with ID '{}' from DB as it's not in config.",
                    &db_path_info.original_path, &db_path_info.external_id
                );
            }
        }

        Ok(pruned_paths)
    }

    // Get a path with external_id list
    fn get_all_db_paths(&self) -> Result<Vec<DbPathInfo>, KaguyaError> {
        let mut stmt = self.conn.prepare(
            "SELECT gp.id, g.external_id, gp.original_path
             FROM game_path AS gp
            JOIN game AS g ON gp.game_id = g.id",
        )?;

        let path_iter = stmt.query_map([], |row| {
            Ok(DbPathInfo {
                id: row.get(0)?,
                external_id: row.get(1)?,
                original_path: row.get(2)?,
            })
        })?;

        let paths = path_iter.collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(paths)
    }

    // /// Accept gamd_id, return game paths list from database.
    // pub fn get_paths_of_single_game(&self, game_id: i64) -> Result<Vec<PathBuf>, KaguyaError> {
    //     let mut stmt = self
    //         .conn
    //         .prepare("SELECT original_path FROM game_path WHERE game_id = ?1")?;
    //
    //     let paths_iter = stmt.query_map([game_id], |row| {
    //         let original_path_str: String = row.get(0)?;
    //         Ok(PathBuf::from(original_path_str))
    //     })?;
    //
    //     let paths = paths_iter.collect::<Result<Vec<_>, rusqlite::Error>>()?;
    //     Ok(paths)
    // }

    // Upsert game paths from the vault config
    fn upsert_paths(&mut self, game_id: i64, paths: &[PathBuf]) -> Result<(), KaguyaError> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO game_path (game_id, original_path)
                VALUES (?1, ?2)
                ON CONFLICT(game_id, original_path) DO NOTHING",
            )?;

            for path in paths {
                stmt.execute((game_id, path.to_string_lossy().to_string()))?;
            }
        } // stmt end life here
        tx.commit()?;
        Ok(())
    }
}
