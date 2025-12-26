use crate::cli::{AppContext, context};
use crate::db_manager::toml::{add_or_update_game_to_file, read_config_file};
use crate::models::{AddGameRequest, GAMES_FILE, GamesFile, KaguyaError};

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

        add_or_update_game_to_file(&context.games_path, request)?;

        Ok(())
    }

    /// List all games in games.toml
    /// use 'long' flag to list detailed information.
    pub fn list_games(context: &AppContext, long: &bool) -> Result<(), KaguyaError> {
        let games_config_file: GamesFile = read_config_file(&context.games_path)?;

        if *long {
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
}
