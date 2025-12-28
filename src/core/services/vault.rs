use crate::{
    cli::AppContext,
    core::utils::{find_game_ref, get_file_name, get_time_string},
    db_manager::toml::read_games_file,
    fs_utils::archive::compress_to_tar_gz,
    models::{BackupRequest, GameConfig, KaguyaError},
};
use std::path::{Path, PathBuf};

/// Managing actions for 'kaguya vault' command
pub struct VaultService;

impl VaultService {
    /// Backup game saves and configuration.
    ///
    /// If no arguments are given, backup all games.
    /// If '--id' is given, backup specific game.
    /// If '--id' and '--paths' are given, backup specific paths
    pub fn backup(context: &AppContext, request: BackupRequest) -> Result<(), KaguyaError> {
        let games = read_games_file(&context.games_path)?.games;

        if request.id.is_some() {
            // Check whether game id exists or not.
            let game = find_game_ref(&games, request.id.unwrap());
            if game.is_none() {
                return Err(KaguyaError::GameNotFound(
                    request.id.unwrap_or_default().to_string(),
                ));
            }
            let game = game.unwrap();

            if request.paths.is_some() {
                // '--id' and '--paths' are given.
                Self::backup_single_game(context, game, request.paths)?;
            } else {
                // Only '--id' is given.
                Self::backup_single_game(context, game, None)?;
                return Ok(());
            }
        } else {
            // No arguments are given, Backup all games
            for game in &games {
                Self::backup_single_game(context, game, None)?;
            }
        }
        Ok(())
    }

    // Backup all saves and configuration of single game
    fn backup_single_game(
        context: &AppContext,
        game: &GameConfig,
        paths: Option<&Vec<PathBuf>>,
    ) -> Result<(), KaguyaError> {
        let time_string = get_time_string();
        let backup_version_path = context.vault_path.join(&game.id).join(time_string);

        if let Some(p) = paths {
            // '--paths' are given
            for path in p {
                if !game.paths.contains(path) {
                    return Err(KaguyaError::PathNotFound(
                        path.to_string_lossy().to_string(),
                    ));
                }
                std::fs::create_dir_all(&backup_version_path)?;
                Self::backup_single_path(path, &backup_version_path)?;
            }
        } else {
            std::fs::create_dir_all(&backup_version_path)?;
            for path in &game.paths {
                Self::backup_single_path(path, &backup_version_path)?;
            }
        }
        Ok(())
    }

    // Backup single path to target directory, get file name for targer archive file
    // e.g., '~/Games/game-a/saves/' -> '~/.local/bin/kaguya/vault/<ID>/<VERSION>/saves.tar.gz'
    fn backup_single_path(source_path: &PathBuf, target_dir: &Path) -> Result<(), KaguyaError> {
        let file_name = get_file_name(source_path).unwrap_or_default();
        let target = target_dir.join(file_name + ".tar.gz");

        println!("Compressing '{}'...", source_path.to_string_lossy());
        compress_to_tar_gz(source_path, &target)?;
        println!(
            "Compressed to '{}' finished.\n",
            target_dir.to_string_lossy()
        );
        Ok(())
    }
}
