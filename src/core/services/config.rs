use crate::cli::AppContext;
use crate::db_manager::DbManager;
use crate::db_manager::toml::{add_or_update_game_to_file, list_games_form_file, rm_game_in_file};
use crate::models::{AddGameRequest, KaguyaError, ListGameRequest, RmGameRequest};

/// Managing actions for 'kaguya config' command
pub struct ConfigService {
    config: AppContext,
    db: DbManager,
}

impl ConfigService {
    pub fn new(config: AppContext, db: DbManager) -> Self {
        Self { config, db }
    }

    /// Receive a [`AddGameRequest`] and add a new game to the game config file
    pub fn add_or_update_game(&self, request: AddGameRequest) -> Result<(), KaguyaError> {
        std::fs::create_dir_all(&self.config.vault_path)?;

        add_or_update_game_to_file(&self.config.game_config_path, request)?;

        Ok(())
    }

    /// List all games
    pub fn list_games(&self, request: &ListGameRequest) -> Result<(), KaguyaError> {
        list_games_form_file(&self.config.game_config_path, request)

        // let games = &self.db.get_game_list()?;
        // if games.is_empty() {
        //     return Err(KaguyaError::EmptyGameList());
        // }
        //
        // if *request.long {
        //     // Print detailed games list
        //     for game in games {
        //         println!("Game ID: {}", game.external_id);
        //         println!("Name: {}", game.name);
        //         println!("Comment: {}", game.comment.unwrap_or_default());
        //         println!("Saves and configuration paths:");
        //         for path in &game.paths {
        //             println!("\t- {}", path.to_string_lossy());
        //         }
        //         println!();
        //     }
        // } else {
        //     // Print concise games list
        //     for game in games {
        //         println!("Game ID: {}", game.external_id);
        //         println!("Saves and configuration paths:");
        //         for path in &game.paths {
        //             println!(
        //                 "\t- {}",
        //                 path.file_name().unwrap_or_default().to_string_lossy()
        //             );
        //         }
        //         println!();
        //     }
        // }
        // Ok(())
    }

    /// Remove a game config by ID in the game config file
    /// If 'purge' flag is true, backups of the game will NOT retain!
    pub fn rm_game(&self, request: &RmGameRequest) -> Result<(), KaguyaError> {
        if *request.purge {
            rm_game_in_file(&self.config.game_config_path, request.id)?;

            todo!("Todo: Remove game backups action")
        } else {
            rm_game_in_file(&self.config.game_config_path, request.id)
        }
    }
}
