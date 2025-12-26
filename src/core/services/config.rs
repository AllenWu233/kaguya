use crate::cli::AppContext;
use crate::db_manager::toml::{add_or_update_game_to_file, list_games_form_file, rm_game_in_file};
use crate::models::{AddGameRequest, KaguyaError};

/// Managing actions for 'kaguya config' command
pub struct ConfigService;

impl ConfigService {
    /// Receive a 'AddGameRequest' and add a new game to games.toml file
    pub fn add_or_update_game(
        context: &AppContext,
        request: AddGameRequest,
    ) -> Result<(), KaguyaError> {
        std::fs::create_dir_all(&context.vault_path)?;

        add_or_update_game_to_file(&context.games_path, request)?;

        Ok(())
    }

    /// List all games in games.toml
    /// use 'long' flag to list detailed information.
    pub fn list_games(context: &AppContext, long: &bool) -> Result<(), KaguyaError> {
        list_games_form_file(&context.games_path, long)
    }

    /// Remove a game config by ID in games.toml
    /// If 'purge' flag is true, backups of the game will NOT retain!
    pub fn rm_game(context: &AppContext, id: &String, purge: &bool) -> Result<(), KaguyaError> {
        if *purge {
            rm_game_in_file(&context.games_path, id)?;

            todo!("Remove game backups action to be done...")
        } else {
            rm_game_in_file(&context.games_path, id)
        }
    }
}
