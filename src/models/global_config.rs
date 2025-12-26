use std::path::PathBuf;

use crate::{
    cli::{AppContext, context},
    core::utils::get_config_path,
    models::KaguyaError,
};

/// Kaguya global config file
#[derive(Debug)]
pub struct GlobalConfig {
    pub vault: PathBuf,
    pub prune: bool,
    pub keep_versions: u32,
}

impl GlobalConfig {
    pub fn new(context: &AppContext) -> Self {
        Self {
            vault: context.vault_path.clone(),
            prune: false,
            keep_versions: 0,
        }
    }
}
