//! Handlers for all subcommands under the `kaguya config` command.

use crate::{
    cli::{AppContext, ConfigSubcommands},
    core::ConfigService,
    db_manager::DbManager,
    models::{AddGameRequest, KaguyaError, RmGameRequest, requests::ListGameRequest},
    utils::path::{to_absolute_path, transform_paths_option},
};

/// Handles all `kaguya config` subcommands.
pub fn handle_config(
    subcommand: ConfigSubcommands,
    context: &AppContext,
) -> Result<(), KaguyaError> {
    let db = DbManager::new(&context.db_path, &context.vault_config_path)?;
    let mut config_service = ConfigService::new(context.clone(), db);

    match subcommand {
        ConfigSubcommands::Add {
            id,
            name,
            paths,
            comment,
        } => {
            // Generate an 'AddGameRequest', send it to core service to add a new game
            let request = AddGameRequest {
                id,
                name,
                paths: transform_paths_option(paths, to_absolute_path)?,
                comment,
            };

            config_service.add_or_update_game(request)?
        }

        ConfigSubcommands::List { long } => {
            let request = ListGameRequest { long };
            handle_list(&request, &config_service)?;
        }

        ConfigSubcommands::Rm { id, purge } => {
            let request = RmGameRequest { id, purge };
            config_service.rm_game(&request)?
        }
    }
    Ok(())
}

/// Handles the logic for listing games.
fn handle_list(request: &ListGameRequest, service: &ConfigService) -> Result<(), KaguyaError> {
    let games = service.get_game_list()?;

    if games.is_empty() {
        println!("Games list is empty, use 'kaguya config add' to add some games.");
        println!(
            "If you already have a Kaguya vault, check '-v/--vault' option or 'vault' in the global config file."
        );
        return Ok(());
    }

    for game in &games {
        if request.long {
            println!("Game ID: {}", game.id);
            println!("Name: {}", game.name);
            println!("Comment: {}", game.comment.clone().unwrap_or_default());
            println!("Saves and configuration paths:");
        } else {
            println!("{} ({}):", game.name, game.id);
        }

        for path in &game.paths {
            let display_name = if request.long {
                path.to_string_lossy()
            } else {
                path.file_name().unwrap_or_default().to_string_lossy()
            };
            println!("\t- {}", display_name);
        }
        println!();
    }

    Ok(())
}
