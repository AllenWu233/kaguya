use crate::cli::{AppContext, context};
use crate::db_manager::toml::add_or_update_game_to_file;
use crate::models::{AddGameRequest, GAMES_FILE, KaguyaError};

/// Managing actions for 'kaguya config' command
pub struct ConfigService;

impl ConfigService {
    /// Receive a 'AddGameRequest' and add a new game to games.toml file
    pub fn add_or_update_game(
        context: &AppContext,
        request: AddGameRequest,
    ) -> Result<(), KaguyaError> {
        let vault_path = &context.vault_path;
        std::fs::create_dir_all(vault_path)?;

        let games_config_path = vault_path.join(GAMES_FILE);
        add_or_update_game_to_file(&games_config_path, request)?;

        Ok(())
    }

    pub fn list_games(context: &AppContext, long: &bool) -> Result<(), KaguyaError> {
        todo!()
    }
}
