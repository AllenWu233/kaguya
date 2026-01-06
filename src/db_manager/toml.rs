//! Serializing and deserializing toml config

use serde::{Deserialize, Serialize};

use crate::{
    models::{AddGameRequest, GameConfig, KaguyaError, VaultConfig},
    utils::path::{expand_path, find_game_mut, shrink_path, transform_paths},
};
use std::path::Path;

/// Adds a new game [`GameConfig`] to the vault config file
pub fn add_or_update_game_to_file(
    vault_config_path: &impl AsRef<Path>,
    request: AddGameRequest,
) -> Result<(), KaguyaError> {
    // Deserialize and read string from games.toml
    let mut vault_config_contents: VaultConfig = read_vault_config(vault_config_path)?;

    // Find the game if it exists
    if let Some(existing_game) = find_game_mut(&mut vault_config_contents.games, &request.id) {
        // Game exists, update it.
        println!("Updating existing game '{}' ...", &request.id);
        apply_update(existing_game, &request)?;
    } else {
        // Game not exists, add a new one.
        println!("Adding game '{}'...", &request.id);

        let new_game = GameConfig::from(&request);
        vault_config_contents.games.push(new_game);
    }
    println!("Game added or updated successfully!");

    save_vault_config(vault_config_path, &mut vault_config_contents)
}

// Update existing [`GameConfig`] with [`AddGameRequest`]
// Paths will be merged.
// See also `GameConfig`
fn apply_update(exist: &mut GameConfig, request: &AddGameRequest) -> Result<(), KaguyaError> {
    // Merge paths: combine old and new, remove duplicates
    if let Some(paths) = &request.paths {
        let mut combined_paths = exist.paths.clone();
        for path in paths {
            if !combined_paths.contains(path) {
                combined_paths.push(path.to_path_buf());
            }
        }
        exist.paths = combined_paths;
    }

    if let Some(name) = &request.name {
        exist.name = name.to_string();
    }

    if request.comment.is_some() {
        exist.comment = request.comment.clone().map(|c| c.to_string());
    }

    Ok(())
}

/// Remove a game configuration in vault config, backups remain.
pub fn rm_game_in_vault_config(path: &impl AsRef<Path>, id: &str) -> Result<(), KaguyaError> {
    // Deserialize and read string from config.toml
    let mut vault_config_contents: VaultConfig = read_vault_config(path)?;

    let original_len = vault_config_contents.games.len();
    vault_config_contents.games.retain(|g| *g.id != *id);

    if vault_config_contents.games.len() >= original_len {
        return Err(KaguyaError::GameNotFound(id.to_string()));
    }

    save_vault_config(path, &mut vault_config_contents)
}

pub fn read_vault_config(path: &impl AsRef<Path>) -> Result<VaultConfig, KaguyaError> {
    let mut vault_config = read_toml_file::<VaultConfig>(path)?;
    vault_config.games = vault_config
        .games
        .into_iter()
        .map(|mut game| {
            game.paths = transform_paths(game.paths, expand_path)?;
            Ok(game)
        })
        .collect::<Result<Vec<_>, KaguyaError>>()?;

    Ok(vault_config)
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

fn save_vault_config(
    path: &impl AsRef<Path>,
    vault_config: &mut VaultConfig,
) -> Result<(), KaguyaError> {
    vault_config.games = vault_config
        .games
        .clone()
        .into_iter()
        .map(|mut game| {
            game.paths = transform_paths(game.paths, shrink_path)?;
            Ok(game)
        })
        .collect::<Result<Vec<_>, KaguyaError>>()?;

    save_to_file(path, vault_config)
}

// Serialize config to TOML, and save it to toml file.
fn save_to_file(path: &impl AsRef<Path>, contents: &impl Serialize) -> Result<(), KaguyaError> {
    let toml_string = toml::to_string_pretty(contents)?;
    std::fs::write(path, toml_string)?;
    Ok(())
}
