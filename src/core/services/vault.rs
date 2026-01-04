use crate::{
    cli::AppContext,
    db_manager::{DbManager, toml::read_toml_file},
    fs_utils::archive::compress_to_tar_gz,
    models::{BackupRequest, GameConfig, KaguyaError, VaultConfig, requests::RestoreRequest},
    utils::{
        path::{find_game_ref, get_file_name},
        time::get_time_string,
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
    pub fn backup(&self, request: BackupRequest) -> Result<(), KaguyaError> {
        let games = read_toml_file::<VaultConfig>(&self.config.vault_config_path)?.games;

        match request.id {
            // '--id' is given.
            Some(id) => match find_game_ref(&games, id) {
                // '--paths' is given or is None.
                Some(game) => self.backup_single_game(game, request.paths),

                None => Err(KaguyaError::GameNotFound(
                    request.id.unwrap_or_default().to_string(),
                )),
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
        &self,
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
        let backup_version_dir = &self.config.backup_dir.join(&game.id).join(time_string);
        create_dir_all(backup_version_dir)?;

        for path in paths_to_backup {
            Self::backup_single_path(path, backup_version_dir)?;
        }

        Ok(())
    }

    // Backup single path to target directory, get file name for targer archive file
    // e.g., '~/Games/game-a/saves/' -> '~/.local/bin/kaguya/vault/<ID>/<VERSION>/saves.tar.gz'
    fn backup_single_path(
        src: &impl AsRef<Path>,
        dst: &impl AsRef<Path>,
    ) -> Result<(), KaguyaError> {
        let src = src.as_ref();
        let dst = dst.as_ref();

        let file_name = get_file_name(src).unwrap_or_default();
        let backup_file = dst.join(file_name + ".tar.gz");

        println!("Compressing '{}'...", src.to_string_lossy());
        compress_to_tar_gz(&src, &backup_file)?;
        println!();
        Ok(())
    }

    pub fn restore(&mut self, request: &RestoreRequest) -> Result<(), KaguyaError> {
        todo!("Restore action")
        // let games = self.get_game_list()?;
        // match find_game_ref(games, id) {
        //     Some(game) => {
        //         match
        //     }
        //     None => Err(KaguyaError::GameNotFound(
        //         request.id.unwrap_or_default().to_string(),
        //     )),
        // }
    }

    fn get_game_list(&self) -> Result<Vec<GameConfig>, KaguyaError> {
        Ok(read_toml_file::<VaultConfig>(&self.config.vault_config_path)?.games)
    }
}
