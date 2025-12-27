use crate::{
    cli::AppContext,
    models::{BackupRequest, KaguyaError},
};

/// Managing actions for 'kaguya vault' command
pub struct VaultService;

impl VaultService {
    pub fn backup(context: &AppContext, request: BackupRequest) -> Result<(), KaguyaError> {
        todo!()
    }
}
