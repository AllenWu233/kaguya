//! Serializing and deserializing toml config

use crate::models::{AddGameRequest, GameConfig, GamesFile, KaguyaError};
use std::path::Path;

/// Adds a new game configuration ('GameConfig') to the specified vault config file (games.toml)
pub fn add_game_to_games_file(
    games_config_path: &Path,
    request: AddGameRequest,
) -> Result<(), KaguyaError> {
    // Read the existing configuration, or create a default one if it doesn't exist
    let mut games_config_file: GamesFile = if games_config_path.exists() {
        let content = std::fs::read_to_string(games_config_path)?;
        toml::from_str(&content)?
    } else {
        GamesFile::default()
    };

    // Check for duplicate ID to ensure data intergrity
    if games_config_file.games.iter().any(|g| g.id == request.id) {
        return Err(KaguyaError::GameIdAlreadyExists(request.id));
    }

    // Create the new GameConfig from the request
    let new_game = GameConfig::from_request(request);
    // Add the new game to the list
    games_config_file.games.push(new_game);

    // Serialize the configuration back to TOML
    let toml_string = toml::to_string_pretty(&games_config_file)?;
    // Write the string back to the file
    std::fs::write(games_config_path, toml_string)?;

    Ok(())
}
