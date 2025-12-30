//! Initialize or connect to SQLite DB

use crate::{
    db_manager::toml::read_games_file,
    fs_utils::hash::calculate_file_hash,
    models::{Game, GamesFile, KaguyaError},
};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DbManager {
    conn: Connection,
}

/// Database initialize and sync
impl DbManager {
    // If no kaguya SQLite DB exists, initialize it.
    // Otherwise, sync with games config file if needed
    pub fn new(db_path: &Path, games_config_path: &Path) -> Result<Self, KaguyaError> {
        let conn = Connection::open(db_path)?;
        let mut manager = Self { conn };
        manager.ensure_initialized()?;
        manager.sync_if_needed(games_config_path)?;
        Ok(manager)
    }

    fn ensure_initialized(&mut self) -> Result<(), KaguyaError> {
        let result = self.conn.query_row(
            "SELECT value FROM kaguya_meta WHERE key = 'schema_version'",
            [],
            |row| row.get::<_, String>(0),
        );

        // dbg!(&result);

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

    fn run_initialize_schema(&mut self) -> Result<(), KaguyaError> {
        // SQLite DB initialize schema, located at <project_root>/migrations/
        let init_sql = include_str!("../../migrations/V1__initial_schema.sql");
        self.conn.execute_batch(init_sql)?;
        println!("Database initialized successfully.");
        Ok(())
    }

    // Sync database if games config file have been changed
    // Do nothing if games config file doesn't exist
    fn sync_if_needed(&mut self, games_config_path: &Path) -> Result<(), KaguyaError> {
        // Return None if games_config_hash == NULL
        let last_synced_hash: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM kaguya_meta WHERE key = 'games_config_hash'",
                [],
                |row| row.get(0),
            )
            .ok();

        let db_hash_record = last_synced_hash;
        let file_hash = calculate_file_hash(games_config_path).ok();

        dbg!(&db_hash_record, &file_hash);

        // Database isn't up to date with games config file
        if file_hash.is_some() && db_hash_record != file_hash {
            println!("Games config file has changed, syncing database...");

            let game_config_file: GamesFile = read_games_file(games_config_path)?;
            for game_config in game_config_file.games {
                let game = Game::from(game_config);
                self.insert_or_update_game(game)?;
            }

            // Update games config file hash in the DB
            self.update_meta_value("games_config_hash", &file_hash.unwrap_or_default())?;

            println!("Database synced successfully.");
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

    /// Create or update a game record to the DB, return game.id
    pub fn insert_or_update_game(&self, game: Game) -> Result<i64, KaguyaError> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO game (external_id, name, comment, keep_versions, created_at, updated_at)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                    ON CONFLICT(external_id) DO UPDATE SET
                    name = excluded.name,
                    comment = excluded.comment,
                    keep_versions = excluded.keep_versions,
                    updated_at = excluded.updated_at",
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

        if last_id > 0 {
            println!(
                "Add new game '{}' with ID: '{}'.",
                game.name.unwrap_or_default(),
                last_id
            );
        } else {
            println!(
                "Updated existing game '{}' with ID: '{}'.",
                game.name.unwrap_or_default(),
                last_id
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
