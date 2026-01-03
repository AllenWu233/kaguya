//! Defines the core [`DbManager`] and handles database connection and initialization.
//!
//! The `DbManager` struct is the central entry point for all database operations.
//! The [`new`](DbManager::new) associated function is responsible for opening a
//! connection, running the initial schema if necessary, and performing the first
//! sync. Other modules extend its functionality using traits.

use super::{DbManagerMetaExt, DbManagerSyncExt};
use crate::models::KaguyaError;
use rusqlite::Connection;
use std::path::Path;

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
}
