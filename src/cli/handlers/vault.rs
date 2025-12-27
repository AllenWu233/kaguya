//! Handlers for all subcommands under the `kaguya vault` command.

use crate::{
    cli::{AppContext, parser::VaultSubcommands},
    core::VaultService,
    models::{BackupRequest, KaguyaError},
};

pub fn handle_vault(
    subcommand: &VaultSubcommands,
    context: &AppContext,
) -> Result<(), KaguyaError> {
    match subcommand {
        VaultSubcommands::Backup { id, paths } => {
            let request = BackupRequest {
                id: id.as_deref(),
                paths: paths.as_ref(),
            };
            VaultService::backup(context, request)?
        }

        VaultSubcommands::Restore { id, version, paths } => todo!(),

        _ => todo!(),
    }

    Ok(())
}
