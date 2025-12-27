use crate::{
    cli::AppContext,
    core::utils::{find_game_ref, get_file_name, get_time_string},
    db_manager::toml::read_games_file,
    fs_utils::archive::compress_to_tar_gz,
    models::{BackupRequest, GameConfig, KaguyaError},
};

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
            if request.paths.is_some() {
                // '--id' and '--paths' are given.
                todo!("Todo: with '--paths'")
            } else {
                // Only '--id' is given.
                if let Some(game) = find_game_ref(&games, request.id.unwrap()) {
                    Self.backup_single_game(context, game)?;
                } else {
                    return Err(KaguyaError::GameNotFound(
                        request.id.unwrap_or_default().to_string(),
                    ));
                }
                return Ok(());
            }
        } else {
            // No arguments are given, Backup all games
            for game in &games {
                Self.backup_single_game(context, game)?;
            }
        }
        Ok(())
    }

    // Backup all saves and configuration of single game
    fn backup_single_game(
        &self,
        context: &AppContext,
        game: &GameConfig,
    ) -> Result<(), KaguyaError> {
        let time_string = get_time_string();
        let backup_version_path = context.vault_path.join(&game.id).join(time_string);
        std::fs::create_dir_all(&backup_version_path)?;

        for path in &game.paths {
            let file_name = get_file_name(path).unwrap_or_default();
            let target = backup_version_path.join(file_name + ".tar.gz");

            println!("Compressing '{}'...", path.to_string_lossy());
            compress_to_tar_gz(path, &target)?;
            println!("Compressed to '{}' finished.\n", target.to_string_lossy());
        }
        Ok(())
    }

    // fn backup_single_path(&self, context: &AppContext, path: &PathBuf) -> Result<(), KaguyaError> {
    //     let time_string = get_time_string();
    //     let file_name = path
    //         .file_name()
    //         .unwrap_or_default()
    //         .to_string_lossy()
    //         .into_owned();
    //
    //     let target = &context
    //         .vault_path
    //         .join(BACKUP_DIR)
    //         .join(time_string)
    //         .join(file_name + ".tar.gz");
    //
    //     println!("Compressing '{}'...", path.to_string_lossy());
    //
    //     dbg!(path, target);
    //     Ok(())
    //
    //     // compress_to_tar_gz(path, target)
    // }
}
