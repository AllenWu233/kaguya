use std::path::{Path, PathBuf};

use rusqlite::params;

use super::DbManager;
use crate::{
    models::{
        KaguyaError,
        db::{Backup, BackupFile},
    },
    utils::path::expand_path,
};

pub trait DbManagerBackupExt {
    fn insert_backup(&mut self, backup: &Backup) -> Result<i64, KaguyaError>;

    fn insert_backup_file(
        &mut self,
        game_id: i64,
        files: Vec<BackupFile>,
    ) -> Result<(), KaguyaError>;

    fn get_archive_file_path(
        &self,
        game_id: i64,
        version: Option<String>,
        original_path: &impl AsRef<Path>,
    ) -> Result<PathBuf, KaguyaError>;
}

impl DbManagerBackupExt for DbManager {
    fn insert_backup(&mut self, backup: &Backup) -> Result<i64, KaguyaError> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO backup (game_id, version, timestamp)
                VALUES (?1, ?2, ?3)",
        )?;

        stmt.execute((&backup.game_id, &backup.version, &backup.timestamp))?;

        Ok(self.conn.last_insert_rowid())
    }

    fn insert_backup_file(
        &mut self,
        backup_id: i64,
        files: Vec<BackupFile>,
    ) -> Result<(), KaguyaError> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO backup_file (backup_id, original_path, archive_path, size_bytes, checksum)
                    VALUES (?1, ?2, ?3, ?4, ?5)"
            )?;

            for file in files {
                stmt.execute((
                    backup_id,
                    file.original_path,
                    file.archive_path,
                    file.size_bytes,
                    file.checksum,
                ))?;
            }
        } // stmt end life here
        tx.commit()?;
        Ok(())
    }

    fn get_archive_file_path(
        &self,
        game_id: i64,
        version: Option<String>,
        original_path: &impl AsRef<Path>,
    ) -> Result<PathBuf, KaguyaError> {
        // Convert Path to String for SQLite comparison (TEXT field)
        let path_str = original_path.as_ref().to_string_lossy().to_string();

        let archive_path_str: String = match version {
            // Case 1: Specific version provided
            Some(ver) => self.conn.query_row(
                "SELECT bf.archive_path 
             FROM backup b 
             JOIN backup_file bf ON b.id = bf.backup_id 
             WHERE b.game_id = ?1 AND b.version = ?2 AND bf.original_path = ?3",
                params![game_id, ver, path_str],
                |row| row.get(0),
            )?,

            // Case 2: Version is None, find the latest by timestamp
            None => self.conn.query_row(
                "SELECT bf.archive_path 
             FROM backup b 
             JOIN backup_file bf ON b.id = bf.backup_id 
             WHERE b.game_id = ?1 AND bf.original_path = ?2
             ORDER BY b.timestamp DESC 
             LIMIT 1",
                params![game_id, path_str],
                |row| row.get(0),
            )?,
        };

        // Ok(PathBuf::from(archive_path_str))
        expand_path(&archive_path_str)
    }
}
