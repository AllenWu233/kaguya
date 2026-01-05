use crate::{
    cli::AppContext,
    db_manager::{
        DbManager,
        sqlite::{DbManagerBackupExt, DbManagerGameExt},
        toml::read_toml_file,
    },
    fs_utils::{
        archive::{calculate_file_bytes, compress_to_tar_gz},
        hash::calculate_entry_checksum,
        restore::restore_archive,
    },
    models::{
        BackupRequest, GameConfig, KaguyaError, VaultConfig,
        db::{Backup, BackupFile},
        requests::RestoreRequest,
    },
    utils::{
        path::{find_game_ref, get_file_name},
        time::{get_time_string, get_timestamp},
    },
};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

/// Managing actions for 'kaguya vault' command
pub struct VaultService {
    config: AppContext,
    db: DbManager,
}

impl VaultService {
    pub fn new(config: AppContext, db: DbManager) -> Self {
        Self { config, db }
    }

    /// Backup game saves and configuration.
    ///
    /// If no arguments are given, backup all games.
    /// If '--id' is given, backup specific game.
    /// If '--id' and '--paths' are given, backup specific paths
    pub fn backup(&mut self, request: BackupRequest) -> Result<(), KaguyaError> {
        let games = read_toml_file::<VaultConfig>(&self.config.vault_config_path)?.games;

        match request.id {
            // '--id' is given.
            Some(id) => match find_game_ref(&games, &id) {
                // '--paths' is given or is None.
                Some(game) => self.backup_single_game(game, request.paths.as_ref()),

                None => Err(KaguyaError::GameNotFound(id)),
            },

            // No arguments are given, Backup all games
            None => {
                for game in &games {
                    self.backup_single_game(game, None)?;
                }
                Ok(())
            }
        }
    }

    // Backup all saves and configuration of single game
    fn backup_single_game(
        &mut self,
        game: &GameConfig,
        paths: Option<&Vec<PathBuf>>,
    ) -> Result<(), KaguyaError> {
        let paths_to_backup = match paths {
            // '--paths' is given.
            Some(p) => {
                // Check whether backup paths exist or not
                if let Some(invalid_path) = p.iter().find(|path| !game.paths.contains(path)) {
                    return Err(KaguyaError::PathNotFound(
                        invalid_path.to_string_lossy().to_string(),
                    ));
                }
                p
            }
            // No arguments.
            None => &game.paths,
        };

        if paths_to_backup.is_empty() {
            println!(
                "No paths specified for game '{}' with ID '{}', skipping backup.",
                &game.name, &game.id
            );
            return Ok(());
        }

        let time_string = get_time_string();
        let backup_version_dir = &self.config.backup_dir.join(&game.id).join(&time_string);
        create_dir_all(backup_version_dir)?;

        // Update DB
        // Table `Backup`
        let backup_record = Backup {
            id: 0,
            game_id: self.db.get_game_id_with_external_id(&game.id)?,
            version: time_string,
            timestamp: get_timestamp(),
        };
        let backup_record_game_id = self.db.insert_backup(&backup_record)?;

        // Table `BackupFile`
        let mut backup_file_records = Vec::new();
        for path in paths_to_backup {
            // Backup action
            let archive_path = Self::backup_single_path(path, backup_version_dir)?;

            let backup_file_record = BackupFile {
                id: 0,
                backup_id: backup_record_game_id,
                original_path: path.to_string_lossy().to_string(),
                archive_path: archive_path.clone().to_string_lossy().to_string(),
                size_bytes: calculate_file_bytes(&archive_path)?,
                checksum: calculate_entry_checksum(&archive_path)?,
            };
            backup_file_records.push(backup_file_record);
        }
        self.db
            .insert_backup_file(backup_record_game_id, backup_file_records)?;

        Ok(())
    }

    // Backup single path to target directory, get file name for targer archive file.
    // Return archive file path.
    //
    // e.g., '~/Games/game-a/saves/' -> '~/.local/bin/kaguya/vault/<ID>/<VERSION>/saves.tar.gz'
    fn backup_single_path(
        src: &impl AsRef<Path>,
        dst: &impl AsRef<Path>,
    ) -> Result<PathBuf, KaguyaError> {
        let src = src.as_ref();
        let dst = dst.as_ref();

        let file_name = get_file_name(src).unwrap_or_default();
        let backup_file = dst.join(file_name + ".tar.gz");

        println!("Compressing '{}'...", src.to_string_lossy());
        compress_to_tar_gz(&src, &backup_file)?;
        println!();
        Ok(backup_file)
    }

    pub fn restore(&mut self, request: &RestoreRequest) -> Result<(), KaguyaError> {
        let games = self.get_game_list()?;
        // '--id'
        match find_game_ref(&games, &request.id) {
            Some(game) => {
                // '--paths'
                let restore_paths = match &request.paths {
                    Some(p) => p,
                    None => &game.paths,
                };
                let game_id = self.db.get_game_id_with_external_id(&game.id)?;

                for path in restore_paths {
                    let archive_path =
                        self.db
                            .get_archive_file_path(game_id, request.version.clone(), path)?; // '--version'

                    restore_archive(&archive_path, &path)?;
                }
                Ok(())
            }
            None => Err(KaguyaError::GameNotFound(request.id.clone())),
        }
    }

    fn get_game_list(&self) -> Result<Vec<GameConfig>, KaguyaError> {
        Ok(read_toml_file::<VaultConfig>(&self.config.vault_config_path)?.games)
    }
}
