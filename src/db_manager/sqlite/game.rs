//! Database operations for the `game` table.
//!
//! This module defines the [`DbManagerGameExt`] trait, which extends the main
//! [`DbManager`] with methods to create, read, update, and delete game records.

use std::collections::HashSet;

use super::{DbManager, DbManagerGamePathExt};
use crate::models::{Game, KaguyaError, VaultConfig};

pub trait DbManagerGameExt {
    fn upsert_games_from_config(
        &mut self,
        vault_config_file: &VaultConfig,
    ) -> Result<(), KaguyaError>;

    fn prune_obsolete_games(
        &mut self,
        vault_config_file: &VaultConfig,
    ) -> Result<Vec<String>, KaguyaError>;

    fn find_game_with_external_id(&self, external_id: &str) -> Result<i64, KaguyaError>;
    fn get_db_game_list(&self) -> Result<Vec<Game>, KaguyaError>;
    fn upsert_game(&self, game: &Game) -> Result<Option<i64>, KaguyaError>;
}

impl DbManagerGameExt for DbManager {
    // Upsert games from the vault config
    fn upsert_games_from_config(
        &mut self,
        vault_config_file: &VaultConfig,
    ) -> Result<(), KaguyaError> {
        for game_config in &vault_config_file.games {
            let game = Game::from(game_config);

            // Use reterned game id if fields except paths have been updated
            let game_id = if let Some(id) = self.upsert_game(&game)? {
                id
            } else {
                self.find_game_with_external_id(&game.external_id)?
            };

            self.upsert_paths(game_id, &game_config.paths)?;
        }
        Ok(())
    }

    // Prune database of games and paths not found int the vault config.
    fn prune_obsolete_games(
        &mut self,
        vault_config_file: &VaultConfig,
    ) -> Result<Vec<String>, KaguyaError> {
        let db_game_list = self.get_db_game_list()?;
        let config_game_ids: HashSet<&str> = vault_config_file
            .games
            .iter()
            .map(|config_game| config_game.id.as_ref())
            .collect();

        let mut pruned_games = Vec::new();
        for db_game in &db_game_list {
            if !config_game_ids.contains(&db_game.external_id.as_ref()) {
                let sql = "DELETE FROM game WHERE id = ?1";
                self.conn.execute(sql, [&db_game.id])?;
                pruned_games.push(db_game.external_id.clone());

                println!(
                    "Pruned game with ID '{}' from DB as it's not in config.",
                    &db_game.external_id
                );
            }
        }

        Ok(pruned_games)
    }

    // Return game.id if game exists, according to Game ID: game(external_id)
    fn find_game_with_external_id(&self, external_id: &str) -> Result<i64, KaguyaError> {
        let sql = "SELECT id FROM game WHERE external_id = ?1";
        let id = self
            .conn
            .query_row(sql, [external_id], |row| row.get::<_, i64>(0))?;
        Ok(id)
    }

    /// Get games list from database.
    fn get_db_game_list(&self) -> Result<Vec<Game>, KaguyaError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, external_id, name, comment, keep_versions, created_at, updated_at 
             FROM game 
             ORDER BY name",
        )?;

        let game_list_iter = stmt.query_map([], |row| {
            Ok(Game {
                id: Some(row.get(0)?),
                external_id: row.get(1)?,
                name: row.get(2)?,
                comment: row.get(3)?,
                keep_versions: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let game_list = game_list_iter.collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(game_list)
    }

    /// Update or insert a game record to the DB, return game.id
    fn upsert_game(&self, game: &Game) -> Result<Option<i64>, KaguyaError> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO game (external_id, name, comment, keep_versions, created_at, updated_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(external_id) DO UPDATE SET
                    name = excluded.name,
                    comment = excluded.comment,
                    keep_versions = excluded.keep_versions,
                    updated_at = excluded.updated_at
                WHERE name IS NOT excluded.name
                    OR comment IS NOT excluded.comment
                    OR keep_versions IS NOT excluded.keep_versions",
        )?;

        stmt.execute((
            &game.external_id,
            &game.name,
            &game.comment,
            &game.keep_versions,
            &game.created_at,
            &game.updated_at,
        ))?;

        let last_id = self.conn.last_insert_rowid();

        // let new_game_added = last_id > 0;
        // let exist_game_updated = self.conn.changes() > 0;
        //
        // if new_game_added {
        //     // INSERT
        //     println!(
        //         "Added new game '{}' with ID: '{}'.",
        //         game.name, game.external_id
        //     );
        // } else if exist_game_updated {
        //     // UPDATE
        //     println!(
        //         "Updated existing game '{}' with ID: '{}'.",
        //         game.name, game.external_id
        //     );
        // }

        if last_id == 0 {
            Ok(None)
        } else {
            Ok(Some(last_id))
        }
    }
}
