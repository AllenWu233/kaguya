//! Initialize or connect to SQLite DB

use crate::{
    db_manager::toml::read_game_config_file,
    fs_utils::hash::calculate_file_hash,
    models::{Game, GameConfigFile, KaguyaError, db::DbPathInfo},
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
        manager.sync(game_config_path, false)?;
        Ok(manager)
    }

    fn ensure_initialized(&mut self) -> Result<(), KaguyaError> {
        let result = self.get_meta_value("schema_version");
        if result.is_err() {
            println!("Database not found. Initializing new database...");
            self.run_initialize_schema()?;
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
    pub fn sync(
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

    // Sync database with game config file
    fn perform_sync(
        &mut self,
        game_config_path: &impl AsRef<Path>,
        new_hash: String,
    ) -> Result<(), KaguyaError> {
        let game_config_file: GameConfigFile = read_game_config_file(game_config_path)?;

        self.upsert_games_from_config(&game_config_file)?;
        self.prune_obsolete_games(&game_config_file)?;
        self.prune_obsolete_paths(&game_config_file)?;

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

    // Prune database of games and paths not found int the game config file.
    fn prune_obsolete_games(
        &mut self,
        game_config_file: &GameConfigFile,
    ) -> Result<Vec<String>, KaguyaError> {
        let db_game_list = self.get_db_game_list()?;
        let config_game_ids: HashSet<&str> = game_config_file
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

    // Prune database of game paths not found int the game config file.
    fn prune_obsolete_paths(
        &mut self,
        game_config_file: &GameConfigFile,
    ) -> Result<Vec<(String, String)>, KaguyaError> {
        let db_paths = self.get_all_db_paths()?;
        let config_paths: HashSet<(String, String)> = game_config_file
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
}

/// Query actions
impl DbManager {
    // Return game.id if game exists, according to Game ID: game(external_id)
    fn find_game_with_external_id(&self, external_id: &String) -> Result<i64, KaguyaError> {
        let sql = "SELECT id FROM game WHERE external_id = ?1";
        let id = self
            .conn
            .query_row(sql, [external_id], |row| row.get::<_, i64>(0))?;
        Ok(id)
    }

    /// Get games list from database.
    pub fn get_db_game_list(&self) -> Result<Vec<Game>, KaguyaError> {
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

    /// Update or insert a game record to the DB, return game.id
    pub fn upsert_game(&self, game: &Game) -> Result<Option<i64>, KaguyaError> {
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

    // Upsert game paths from the game config file
    fn upsert_paths(&mut self, game_id: i64, paths: &Vec<PathBuf>) -> Result<(), KaguyaError> {
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

    fn update_meta_value(&self, key: &str, value: &str) -> Result<(), KaguyaError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO kaguya_meta (key, value) VALUES (?1, ?2)",
            (key, value),
        )?;
        Ok(())
    }

    fn get_meta_value(&self, key: &str) -> Result<String, KaguyaError> {
        // Return None if key == NULL in the database
        let value = self.conn.query_row(
            "SELECT value FROM kaguya_meta WHERE key = ?1",
            [key],
            |row| row.get::<_, String>(0),
        )?;
        Ok(value)
    }
}
