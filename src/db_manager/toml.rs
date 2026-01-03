//! Serializing and deserializing toml config

use serde::{Deserialize, Serialize};

use crate::{
    models::{AddGameRequest, GameConfig, GameConfigFile, KaguyaError},
    utils::path::find_game_mut,
};
use std::path::Path;

/// Adds a new game [`GameConfig`] to the vault games config file
pub fn add_or_update_game_to_file(
    game_config_path: &impl AsRef<Path>,
    request: AddGameRequest,
) -> Result<(), KaguyaError> {
    // Deserialize and read string from games.toml
    let mut games_config_contents: GameConfigFile = read_toml_file(game_config_path)?;

    // Find the game if it exists
    if let Some(existing_game) = find_game_mut(&mut games_config_contents.games, request.id) {
        // Game exists, update it.
        println!("Updating existing game '{}' ...", &request.id);
        apply_update(existing_game, &request)?;
    } else {
        // Game not exists, add a new one.
        println!("Adding game '{}'...", &request.id);

        let new_game = GameConfig::from(&request);
        games_config_contents.games.push(new_game);
    }
    println!("Game added or updated successfully!");

    save_to_file(game_config_path, &games_config_contents)
}

// Update existing [`GameConfig`] with [`AddGameRequest`]
// Paths will be merged.
// See also `GameConfig`
fn apply_update(exist: &mut GameConfig, request: &AddGameRequest) -> Result<(), KaguyaError> {
    // Merge paths: combine old and new, remove duplicates
    if request.paths.is_some() {
        let mut combined_paths = exist.paths.clone();
        for path in request.paths.unwrap() {
            if !combined_paths.contains(path) {
                combined_paths.push(path.to_path_buf());
            }
        }
        exist.paths = combined_paths;
    }

    if request.name.is_some() {
        exist.name = request.name.unwrap_or_default().to_string();
    }
    if request.comment.is_some() {
        exist.comment = request.comment.map(|c| c.to_string());
    }

    Ok(())
}

/// Remove a game configuration in games.toml, backups remain.
pub fn rm_game_in_file(path: &impl AsRef<Path>, id: &str) -> Result<(), KaguyaError> {
    // Deserialize and read string from config.toml
    let mut games_config_contents: GameConfigFile = read_toml_file(path)?;

    let original_len = games_config_contents.games.len();
    games_config_contents.games.retain(|g| *g.id != *id);

    if games_config_contents.games.len() >= original_len {
        return Err(KaguyaError::GameNotFound(id.to_string()));
    }

    save_to_file(path, &games_config_contents)
}

// Read .toml file, deserialize from TOML to string
pub fn read_toml_file<T>(path: &impl AsRef<Path>) -> Result<T, KaguyaError>
where
    T: for<'de> Deserialize<'de> + Default,
{
    let path = path.as_ref();
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Ok(T::default())
    }
}

// Serialize [`GameConfigFile`] or [`GlobalConfig`] to TOML, and save it to toml file.
fn save_to_file(path: &impl AsRef<Path>, contents: &impl Serialize) -> Result<(), KaguyaError> {
    let toml_string = toml::to_string_pretty(contents)?;
    std::fs::write(path, toml_string)?;
    Ok(())
}
