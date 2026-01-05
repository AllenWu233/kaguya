//! Handlers for all subcommands under the `kaguya vault` command.

use crate::{
    cli::{AppContext, parser::VaultSubcommands},
    core::VaultService,
    db_manager::DbManager,
    models::{BackupRequest, KaguyaError, requests::RestoreRequest},
};

pub fn handle_vault(subcommand: VaultSubcommands, context: &AppContext) -> Result<(), KaguyaError> {
    let db = DbManager::new(&context.db_path, &context.vault_config_path)?;
    let mut vault_service = VaultService::new(context.clone(), db);

    match subcommand {
        VaultSubcommands::Backup { id, paths } => {
            let request = BackupRequest { id, paths };
            vault_service.backup(request)?
        }

        VaultSubcommands::Restore { id, version, paths } => {
            let request = RestoreRequest { id, version, paths };
            vault_service.restore(&request)?
        }

        _ => todo!(),
    }

    Ok(())
}
