use crate::cli::AppContext;
use crate::db_manager::DbManager;
use crate::db_manager::toml::{add_or_update_game_to_file, read_toml_file, rm_game_in_file};
use crate::models::{AddGameRequest, GameConfig, GameConfigFile, KaguyaError, RmGameRequest};

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

    pub fn get_game_list(&self) -> Result<Vec<GameConfig>, KaguyaError> {
        Ok(read_toml_file::<GameConfigFile>(&self.config.game_config_path)?.games)
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
