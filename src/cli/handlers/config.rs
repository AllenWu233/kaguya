//! Handlers for all subcommands under the `kaguya config` command.

use crate::cli::{Cli, ConfigSubcommands};
use crate::core::ConfigService;
use crate::models::{AddGameRequest, KaguyaError};

/// Handles all `kaguya config` subcommands.
pub fn handle_config(subcommand: &ConfigSubcommands, cli: &Cli) -> Result<(), KaguyaError> {
    let config_service = ConfigService::new(cli)?;

    match subcommand {
        ConfigSubcommands::Add {
            id,
            name,
            paths,
            comment,
        } => {
            dbg!(id, name, paths, comment);

            let request = AddGameRequest {
                id: id.clone(),
                name: name.clone(),
                paths: paths.clone(),
                comment: comment.clone(),
            };

            config_service.add_game(request)?;

            println!("Game {} added successfully!", id);
        }

        ConfigSubcommands::List { long } => todo!(),

        ConfigSubcommands::Rm { id, purge } => todo!(),
    }

    Ok(())
}

// fn handle_add_game(
//     service: &ConfigService,
//     id: &String,
//     name: &String,
//     paths: &Vec<PathBuf>,
//     comment: &Option<String>,
// ) -> Result<(), KaguyaError> {
// }
