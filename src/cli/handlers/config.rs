//! Handlers for all subcommands under the `kaguya config` command.

use crate::{
    cli::{AppContext, ConfigSubcommands},
    core::ConfigService,
    db_manager::DbManager,
    models::{AddGameRequest, KaguyaError, RmGameRequest, requests::ListGameRequest},
};

/// Handles all `kaguya config` subcommands.
pub fn handle_config(
    subcommand: &ConfigSubcommands,
    context: &AppContext,
) -> Result<(), KaguyaError> {
    let db = DbManager::new(&context.db_path, &context.game_config_path)?;
    let config_service = ConfigService::new(context.clone(), db);

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
                name: name.as_deref(),
                paths: paths.as_ref(),
                comment: comment.as_deref(),
            };
            config_service.add_or_update_game(request)?
        }

        ConfigSubcommands::List { long } => {
            let request = ListGameRequest { long };
            config_service.list_games(&request)?
        }

        ConfigSubcommands::Rm { id, purge } => {
            let request = RmGameRequest { id, purge };
            config_service.rm_game(&request)?
        }
    }

    Ok(())
}
