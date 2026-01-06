use crate::{
    cli::AppContext,
    db_manager::{
        DbManager,
        sqlite::{DbManagerBackupExt, DbManagerGameExt},
        toml::read_vault_config,
    },
    fs_utils::{
        archive::{calculate_file_bytes, compress_to_tar_gz},
        hash::calculate_entry_checksum,
        restore::restore_archive,
    },
    models::{
        BackupRequest, GameConfig, KaguyaError,
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
        let games = read_vault_config(&self.config.vault_config_path)?.games;

        match request.id {
            // '--id' is given.
            Some(id) => match find_game_ref(&games, &id) {
                // '--paths' is given or is None.
                Some(game) => {
                    self.backup_single_game(game, request.paths.as_ref())?;
                    println!("Backup finished!");
                    Ok(())
                }
                None => Err(KaguyaError::GameNotFound(id)),
            },
            // No arguments are given, Backup all games
            None => {
                for game in &games {
                    self.backup_single_game(game, None)?;
                }
                println!("Backup finished!");
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
        // Resolve and validate paths
        let paths_to_backup = self.resolve_backup_paths(game, paths)?;
        if paths_to_backup.is_empty() {
            println!(
                "No paths specified for game '{}' with ID '{}', skipping backup.",
                game.name, game.id
            );
            return Ok(());
        }

        // Prepare backup directory and DB record
        let time_string = get_time_string();
        let backup_version_dir = &self.config.backup_dir.join(&game.id).join(&time_string);
        create_dir_all(backup_version_dir)?;

        let backup_record = Backup {
            id: 0,
            game_id: self.db.get_game_id_with_external_id(&game.id)?,
            version: time_string.clone(),
            timestamp: get_timestamp(),
        };
        let backup_id = self.db.insert_backup(&backup_record)?;

        // Execute backup and collect metadata
        println!("Backing up '{} ({})'...", game.name, game.id);
        let backup_file_records =
            self.perform_backup_and_collect_meta(paths_to_backup, backup_version_dir, backup_id)?;

        // Persist metadata
        self.db.insert_backup_file(backup_id, backup_file_records)?;
        println!("Backup '{} ({})' completed.\n", game.name, game.id);

        Ok(())
    }

    // Resolves the list of paths to backup based on user input and game config.
    // Validates that user-provided paths exist in the game configuration.
    fn resolve_backup_paths<'a>(
        &self,
        game: &'a GameConfig,
        paths: Option<&'a Vec<PathBuf>>,
    ) -> Result<&'a Vec<PathBuf>, KaguyaError> {
        match paths {
            Some(p) => {
                // Validate that all provided paths are part of the game config
                if let Some(invalid_path) = p.iter().find(|path| !game.paths.contains(path)) {
                    return Err(KaguyaError::PathNotFound(
                        invalid_path.to_string_lossy().to_string(),
                    ));
                }
                Ok(p)
            }
            None => Ok(&game.paths),
        }
    }

    // Iterates through paths, performs the backup, and collects metadata.
    fn perform_backup_and_collect_meta(
        &self,
        paths: &Vec<PathBuf>,
        target_dir: &impl AsRef<Path>,
        backup_id: i64,
    ) -> Result<Vec<BackupFile>, KaguyaError> {
        let mut records = Vec::new();

        for path in paths {
            // Perform the actual file system backup
            let archive_path = Self::backup_single_path(path, target_dir)?;

            // Collect metadata
            let record = BackupFile {
                id: 0,
                backup_id,
                original_path: path.to_string_lossy().to_string(),
                archive_path: archive_path.to_string_lossy().to_string(),
                size_bytes: calculate_file_bytes(&archive_path)?,
                checksum: calculate_entry_checksum(&archive_path)?,
            };

            records.push(record);
        }

        Ok(records)
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

        println!("\tCompressing '{}'...", src.display());
        compress_to_tar_gz(&src, &backup_file)?;
        println!("\tCompressed to '{}' completed.\n", backup_file.display());

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

                    println!("Restoring from '{}'...", archive_path.display());

                    restore_archive(&archive_path, &path)?;

                    println!("Restore to '{}' succeeded.\n", path.display());
                }
                println!("Restore finished!");
                Ok(())
            }
            None => Err(KaguyaError::GameNotFound(request.id.clone())),
        }
    }

    fn get_game_list(&self) -> Result<Vec<GameConfig>, KaguyaError> {
        Ok(read_vault_config(&self.config.vault_config_path)?.games)
    }
}
