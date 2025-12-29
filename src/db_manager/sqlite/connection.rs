//! Initialize or connect SQLite DB

use crate::models::{Game, GamePath, KaguyaError};
use rusqlite::Connection;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DbManager {
    conn: Connection,
}

/// Database initialize and synchronic
impl DbManager {
    // If no kaguya SQLite DB exists, initialize it.
    // Otherwise, sync with games config file if needed
    pub fn new(path: &PathBuf) -> Result<Self, KaguyaError> {
        let conn = Connection::open(path)?;
        let mut manager = Self { conn };
        manager.ensure_initialized()?;
        manager.sync_if_needed()?;
        Ok(manager)
    }

    fn ensure_initialized(&mut self) -> Result<(), KaguyaError> {
        let result = self.conn.query_row(
            "SELECT value FROM kaguya_meta WHERE key = 'schema_version'",
            [],
            |row| row.get::<_, String>(0),
        );

        dbg!(&result);

        match result {
            Ok(_version) => {
                // eprintln!("Database is up to date (schema version: {}).", version);
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("Database not found. Initializing new database...");
                self.run_initialize_schema()?;
            }
            Err(e) => return Err(KaguyaError::from(e)),
        }

        Ok(())
    }

    fn run_initialize_schema(&mut self) -> Result<(), KaguyaError> {
        // SQLite DB initialize schema, located at <project_root>/migrations/
        let init_sql = include_str!("../../migrations/V1__initial_schema.sql");
        self.conn.execute_batch(init_sql)?;
        println!("Database initialized successfully.");
        Ok(())
    }

    fn sync_if_needed(&mut self) -> Result<(), KaguyaError> {
        let last_synced_hash: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM kaguya_meta WHERE key = 'games_config_hash'",
                [],
                |row| row.get(0),
            )
            .ok();

        todo!("Sync database")
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
    pub fn get_games_list(&self) -> Result<Vec<Game>, KaguyaError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, external_id, name, comment, keep_versions, created_at, updated_at 
             FROM game 
             ORDER BY name",
        )?;

        let games_iter = stmt.query_map([], |row| {
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

        let games = games_iter.collect::<Result<Vec<_>, rusqlite::Error>>()?;
        Ok(games)
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
}
