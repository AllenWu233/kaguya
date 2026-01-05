use super::DbManager;
use crate::models::{
    KaguyaError,
    db::{Backup, BackupFile},
};

pub trait DbManagerBackupExt {
    fn insert_backup(&mut self, backup: &Backup) -> Result<i64, KaguyaError>;

    fn insert_backup_file(
        &mut self,
        game_id: i64,
        files: Vec<BackupFile>,
    ) -> Result<(), KaguyaError>;
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
}
