// use crate::core::utils::get_vault_path;
use crate::cli::Cli;
use crate::core::utils::get_vault_path;
use crate::db_manager::toml::add_game_to_games_file;
use crate::models::{AddGameRequest, GAMES_FILE, KaguyaError};
use std::path::PathBuf;

pub struct ConfigService {
    // config_path: PathBuf
    vault_path: PathBuf,
}

impl ConfigService {
    pub fn new(cli: &Cli) -> Result<Self, KaguyaError> {
        let vault_path = get_vault_path(&cli.vault_path)?;
        Ok(Self { vault_path })
    }

    /// Receive a 'AddGameRequest' and add a new game to games.toml file
    pub fn add_game(&self, request: AddGameRequest) -> Result<(), KaguyaError> {
        std::fs::create_dir_all(&self.vault_path)?;
        let games_config_path = &self.vault_path.join(GAMES_FILE);
        add_game_to_games_file(games_config_path, request)?;
        Ok(())
    }
}
