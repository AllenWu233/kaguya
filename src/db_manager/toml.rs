//! Serializing and deserializing toml config

use serde::Serialize;

use crate::{
    core::utils::find_game_mut,
    models::{
        AddGameRequest, GameConfig, GamesFile, KaguyaError, ListGameRequest,
        global_config::GlobalConfig,
    },
};
use std::path::Path;

/// Adds a new game [`GameConfig`] to the vault games config file
pub fn add_or_update_game_to_file(
    games_config_path: &Path,
    request: AddGameRequest,
) -> Result<(), KaguyaError> {
    // Deserialize and read string from games.toml
    let mut games_config_contents: GamesFile = read_games_file(games_config_path)?;

    // Find the game if it exists
    if let Some(existing_game) = find_game_mut(&mut games_config_contents.games, request.id) {
        // Game exists, update it.
        println!("Updating existing game '{}' ...", &request.id);
        apply_update(existing_game, &request)?;
    } else {
        // Game not exists, add a new one.
        println!("Adding game '{}'...", &request.id);
        // Create the new GameConfig from the request.
        let new_game = GameConfig::from_request(request);
        // Add the new game to the list.
        games_config_contents.games.push(new_game);
    }
    println!("Game added or updated successfully!");
    // Serialize and save string to games.toml
    save_to_file(games_config_path, &games_config_contents)
}

// Update existing [`GameConfig`] with [`AddGameRequest`]
// Paths will be merged.
// See also `GameConfig`
fn apply_update(exist: &mut GameConfig, request: &AddGameRequest) -> Result<(), KaguyaError> {
    // Merge paths: combine old and new, remove duplicates
    let mut combined_paths = exist.paths.clone();
    for path in request.paths {
        if !combined_paths.contains(path) {
            combined_paths.push(path.to_path_buf());
        }
    }

    if request.name.is_some() {
        exist.name = request.name.map(|n| n.to_string())
    }
    if request.comment.is_some() {
        exist.comment = request.comment.map(|c| c.to_string())
    }

    Ok(())
}

/// Read vault games config file, list all the games
/// Print detailed information if 'long' flag is true
pub fn list_games_form_file(path: &Path, request: &ListGameRequest) -> Result<(), KaguyaError> {
    let games_config_file: GamesFile = read_games_file(path)?;
    if games_config_file.games.is_empty() {
        println!("Games list is empty, use 'kaguya config add' to add some games.");
        println!(
            "If you already have a Kaguya vault, check '-v/--vault' option or 'vault' in the config file."
        );
        return Ok(());
    }

    if *request.long {
        // Print detailed games list
        for game in &games_config_file.games {
            println!("Game ID: {}", game.id);
            println!("Name: {}", game.name.clone().unwrap_or_default());
            println!("Comment: {}", game.comment.clone().unwrap_or_default());
            println!("Saves and configuration paths:");
            for path in &game.paths {
                println!("\t- {}", path.to_string_lossy());
            }
            println!();
        }
    } else {
        // Print concise games list
        for game in &games_config_file.games {
            println!("Game ID: {}", game.id);
            println!("Saves and configuration paths:");
            for path in &game.paths {
                println!(
                    "\t- {}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                );
            }
            println!();
        }
    }

    Ok(())
}

/// Remove a game configuration in games.toml, backups remain.
pub fn rm_game_in_file(path: &Path, id: &str) -> Result<(), KaguyaError> {
    // Deserialize and read string from config.toml
    let mut games_config_contents: GamesFile = read_games_file(path)?;

    let original_len = games_config_contents.games.len();
    games_config_contents.games.retain(|g| *g.id != *id);

    if games_config_contents.games.len() < original_len {
        println!("Game {} removed from config.", id);
    } else {
        return Err(KaguyaError::GameNotFound(id.to_string()));
    }

    save_to_file(path, &games_config_contents)
}

// Read games.toml from vault, deserialize from TOML to string
pub fn read_games_file(path: &Path) -> Result<GamesFile, KaguyaError> {
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Ok(GamesFile::default())
    }
}

// Read config.toml from Kaguya config directory, deserialize from TOML to string
fn _read_config_file(path: &Path) -> Result<GlobalConfig, KaguyaError> {
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Ok(GlobalConfig::default())
    }
}

// Serialize [`GamesFile`] or [`GlobalConfig`] to TOML, and save it to toml file.
fn save_to_file<F: Serialize>(path: &Path, contents: &F) -> Result<(), KaguyaError> {
    // Serialize the configuration back to TOML
    let toml_string = toml::to_string_pretty(contents)?;

    // Write the string back to the file
    std::fs::write(path, toml_string)?;

    Ok(())
}
