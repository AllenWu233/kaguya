//! Manages application metadata stored in the `meta` table.
//!
//! This module defines the [`DbManagerMetaExt`] trait to extend the [`DbManager`]
//! with simple key-value store operations. It is primarily used by the sync
//! process to store and retrieve state information, such as configuration file
//! hashes or schema versions.

use crate::models::KaguyaError;

use super::DbManager;

pub trait DbManagerMetaExt {
    fn update_meta_value(&self, key: &str, value: &str) -> Result<(), KaguyaError>;
    fn get_meta_value(&self, key: &str) -> Result<String, KaguyaError>;
}

impl DbManagerMetaExt for DbManager {
    fn update_meta_value(&self, key: &str, value: &str) -> Result<(), KaguyaError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES (?1, ?2)",
            (key, value),
        )?;
        Ok(())
    }

    fn get_meta_value(&self, key: &str) -> Result<String, KaguyaError> {
        // Return None if key == NULL in the database
        let value = self
            .conn
            .query_row("SELECT value FROM meta WHERE key = ?1", [key], |row| {
                row.get::<_, String>(0)
            })?;
        Ok(value)
    }
}
