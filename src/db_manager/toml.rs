//! Serializing and deserializing toml config

use crate::{
    core::utils::find_game_mut,
    models::{AddGameRequest, GameConfig, GamesFile, KaguyaError},
};
use std::path::Path;

/// Adds a new [`GameConfig`] to the specified vault config file (games.toml)
pub fn add_or_update_game_to_file(
    games_config_path: &Path,
    request: AddGameRequest,
) -> Result<(), KaguyaError> {
    // Deserialize and read string from games.toml
    let mut games_config_file: GamesFile = read_games_file(games_config_path)?;

    // Find the game if it exists
    if let Some(existing_game) = find_game_mut(&mut games_config_file.games, &request.id) {
        println!("Updating existing game '{}' ...", &request.id);
        // Merge paths: combine old and new, remove duplicates
        let mut combined_paths = existing_game.paths.clone();
        for path in &request.paths {
            if !combined_paths.contains(path) {
                combined_paths.push(path.to_path_buf());
            }
        }
        existing_game.paths = combined_paths;
        existing_game.id = request.id;
        existing_game.name = request.name;
        existing_game.comment = request.comment;
    } else {
        // Game not exists, add a new one
        println!("Adding game {} ...", &request.id);
        // Create the new GameConfig from the request.
        let new_game = GameConfig::from_request(request);
        // Add the new game to the list
        games_config_file.games.push(new_game);
    }

    // Serialize and save string to games.toml
    save_to_games_file(games_config_path, &games_config_file)
}

/// Read games.toml from vault, deserialize from TOML to string
fn read_games_file(path: &Path) -> Result<GamesFile, KaguyaError> {
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Ok(GamesFile::default())
    }
}

/// Read config.toml from Kaguya config directory, deserialize from TOML to string
pub fn read_config_file(path: &Path) -> Result<GamesFile, KaguyaError> {
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Ok(GamesFile::default())
    }
}

/// Serialize [`GamesFile`] to TOML, and save it to games.toml
fn save_to_games_file(path: &Path, contents: &GamesFile) -> Result<(), KaguyaError> {
    // Serialize the configuration back to TOML
    let toml_string = toml::to_string_pretty(contents)?;

    // Write the string back to the file
    std::fs::write(path, toml_string)?;

    Ok(())
}
