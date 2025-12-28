//! Initialize or connect SQLite DB

use crate::models::KaguyaError;
use rusqlite::Connection;
use std::path::PathBuf;

struct DbManager {
    conn: Connection,
}

impl DbManager {
    pub fn new(path: &PathBuf) -> Result<Self, KaguyaError> {
        let flag = !path.exists();

        let conn = Connection::open(path)?;
        let mut manager = Self { conn };
        manager.initialize()?;

        if flag {
            println!("Database initialize successfully.");
        }

        Ok(manager)
    }

    fn initialize(&mut self) -> Result<(), KaguyaError> {
        // SQLite DB initialize schema, located at <project_root>/migrations/
        let init_sql = include_str!("../../migrations/V1__initial_schema.sql");

        self.conn.execute_batch(init_sql)?;
        Ok(())
    }
}
