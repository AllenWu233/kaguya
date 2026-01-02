//! Initialize or connect to SQLite DB

use crate::{
    db_manager::toml::read_game_config_file,
    fs_utils::hash::calculate_file_hash,
    models::{Game, GameConfigFile, KaguyaError},
};
use rusqlite::Connection;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct DbManager {
    pub conn: Connection,
}

/// Database initialize and sync
impl DbManager {
    // If no kaguya SQLite DB exists, initialize it.
    // Otherwise, sync with game config file if needed
    pub fn new(
        db_path: &impl AsRef<Path>,
        game_config_path: &impl AsRef<Path>,
    ) -> Result<Self, KaguyaError> {
        let conn = Connection::open(db_path)?;
        let mut manager = Self { conn };
        manager.ensure_initialized()?;
        manager.sync_if_needed(game_config_path)?;
        Ok(manager)
    }

    fn ensure_initialized(&mut self) -> Result<(), KaguyaError> {
        let result = self.conn.query_row(
            "SELECT value FROM kaguya_meta WHERE key = 'schema_version'",
            [],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(_version) => {
                // eprintln!("Database is up to date (schema version: {}).", version);
            }
            Err(e) => {
                if e.to_string().contains("no such table") {
                    println!("Database not found. Initializing new database...");
                    self.run_initialize_schema()?;
                } else {
                    return Err(KaguyaError::from(e));
                }
            }
        }

        Ok(())
    }

    // SQLite DB initialize schema, located at <project_root>/migrations/
    fn run_initialize_schema(&mut self) -> Result<(), KaguyaError> {
        let init_sql = include_str!("../../migrations/V1__initial_schema.sql");
        self.conn.execute_batch(init_sql)?;
        println!("Database initialized successfully.");
        Ok(())
    }

    // Sync database if game config file have been changed
    // Do nothing if game config file doesn't exist
    fn sync_if_needed(&mut self, game_config_path: &impl AsRef<Path>) -> Result<(), KaguyaError> {
        if let Some(new_hash) = self.get_sync_status(game_config_path)? {
            println!("Game config file has changed, syncing database...");
            self.sync_database_with_config(game_config_path, new_hash)?;
        }
        Ok(())
    }

    // Check whther needs to sync or not, return hash of the game config file
    fn get_sync_status(
        &self,
        game_config_path: &impl AsRef<Path>,
    ) -> Result<Option<String>, KaguyaError> {
        // Return None if game_config_file_hash == NULL in the database
        let db_hash_record: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM kaguya_meta WHERE key = 'game_config_file_hash'",
                [],
                |row| row.get(0),
            )
            .ok();

        let file_hash = calculate_file_hash(game_config_path).ok();

        if file_hash.is_some() && db_hash_record != file_hash {
            Ok(file_hash)
        } else {
            Ok(None)
        }
    }

    // Sync database with game config file
    fn sync_database_with_config(
        &mut self,
        game_config_path: &impl AsRef<Path>,
        new_hash: String,
    ) -> Result<(), KaguyaError> {
        let game_config_file: GameConfigFile = read_game_config_file(game_config_path)?;

        self.upsert_games_from_config(&game_config_file)?;
        self.prune_obsolete_games(&game_config_file)?;

        // Update game_config_file_hash in the database
        self.update_meta_value("game_config_file_hash", &new_hash)?;

        println!("Database synced successfully.\n");
        Ok(())
    }

    // Upsert games from the game config file
    fn upsert_games_from_config(
        &mut self,
        game_config_file: &GameConfigFile,
    ) -> Result<(), KaguyaError> {
        for game_config in &game_config_file.games {
            let game = Game::from(game_config);
            self.upsert_game(game)?;
        }
        Ok(())
    }

    // Prune database of games not found int the games config file.
    fn prune_obsolete_games(
        &mut self,
        game_config_file: &GameConfigFile,
    ) -> Result<(), KaguyaError> {
        let db_game_list = self.get_game_list()?;
        let config_game_ids: HashSet<&String> = game_config_file
            .games
            .iter()
            .map(|config_game| &config_game.id)
            .collect();

        for db_game in &db_game_list {
            if !config_game_ids.contains(&db_game.external_id) {
                let sql = "DELETE FROM game WHERE id = ?1";
                self.conn.execute(sql, [&db_game.id])?;
                println!(
                    "Pruned game with ID '{}' from DB as it's not in config.",
                    &db_game.external_id
                );
            }
        }

        Ok(())
    }
}

/// Query actions
impl DbManager {
    // /// Check if game exists or not, according to Game ID - game(external_id)
    // pub fn game_exists(&self, external_id: i64) -> Result<bool, KaguyaError> {
    //     let mut stmt = self
    //         .conn
    //         .prepare("SELECT 1 FROM game WHERE external_id = ?1")?;
    //     let exists = stmt
    //         .query_row([external_id], |_| Ok(true))
    //         .unwrap_or(Ok(false));
    //     Ok(exists)
    // }

    /// Get games list from database.
    pub fn get_game_list(&self) -> Result<Vec<Game>, KaguyaError> {
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

    /// Accept gamd_id, return game paths list from database.
    pub fn get_paths(&self, game_id: i64) -> Result<Vec<PathBuf>, KaguyaError> {
        let mut stmt = self
            .conn
            .prepare("SELECT original_path FROM game_path WHERE game_id = ?1")?;

        let paths_iter = stmt.query_map([game_id], |row| {
            let original_path_str: String = row.get(0)?;
            Ok(PathBuf::from(original_path_str))
        })?;

        let paths = paths_iter.collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(paths)
    }

    /// Update or insert a game record to the DB, return game.id
    pub fn upsert_game(&self, game: Game) -> Result<i64, KaguyaError> {
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
        let new_game_added = last_id > 0;
        let exist_game_updated = self.conn.changes() > 0;

        if new_game_added {
            // INSERT
            println!(
                "Added new game '{}' with ID: '{}'.",
                game.name, game.external_id
            );
        } else if exist_game_updated {
            // UPDATE
            println!(
                "Updated existing game '{}' with ID: '{}'.",
                game.name, game.external_id
            );
        }

        Ok(last_id)
    }

    pub fn update_meta_value(&self, key: &str, value: &str) -> Result<(), KaguyaError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO kaguya_meta (key, value) VALUES (?1, ?2)",
            (key, value),
        )?;
        Ok(())
    }
}
