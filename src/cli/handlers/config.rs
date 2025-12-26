//! Handlers for all subcommands under the `kaguya config` command.

use crate::cli::{AppContext, ConfigSubcommands};
use crate::core::ConfigService;
use crate::models::{AddGameRequest, KaguyaError};
use std::path::PathBuf;

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
        } => handle_add_game(context, id, name, paths, comment)?,

        ConfigSubcommands::List { long } => handle_list_games(context, long)?,

        ConfigSubcommands::Rm { id, purge } => handle_rm_game(context, id, purge)?,
    }

    Ok(())
}

/// Generate an 'AddGameRequest', send it to core service to add a new game
fn handle_add_game(
    context: &AppContext,
    id: &str,
    name: &Option<String>,
    paths: &[PathBuf],
    comment: &Option<String>,
) -> Result<(), KaguyaError> {
    let request = AddGameRequest {
        id: id.to_string(),
        name: name.clone(),
        paths: paths.to_vec(),
        comment: comment.clone(),
    };

    ConfigService::add_or_update_game(context, request)?;
    Ok(())
}

/// List all games in the vault config file
fn handle_list_games(context: &AppContext, long: &bool) -> Result<(), KaguyaError> {
    ConfigService::list_games(context, long)?;
    Ok(())
}

fn handle_rm_game(context: &AppContext, id: &String, purge: &bool) -> Result<(), KaguyaError> {
    ConfigService::rm_game(context, id, purge)?;
    Ok(())
}
