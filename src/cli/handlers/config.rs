//! Handlers for all subcommands under the `kaguya config` command.

use crate::{
    cli::{AppContext, ConfigSubcommands},
    core::ConfigService,
    models::{AddGameRequest, KaguyaError, RmGameRequest, requests::ListGameRequest},
};

/// Handles all `kaguya config` subcommands.
pub fn handle_config(
    subcommand: &ConfigSubcommands,
    context: &AppContext,
) -> Result<(), KaguyaError> {
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
                paths,
                comment: comment.as_deref(),
            };
            ConfigService::add_or_update_game(context, request)?
        }

        ConfigSubcommands::List { long } => {
            let request = ListGameRequest { long };
            ConfigService::list_games(context, &request)?
        }

        ConfigSubcommands::Rm { id, purge } => {
            let request = RmGameRequest { id, purge };
            ConfigService::rm_game(context, &request)?
        }
    }

    Ok(())
}
