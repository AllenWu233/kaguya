use std::path::PathBuf;

use crate::{cli::AppContext, core::utils::get_config_path};
use serde::{Deserialize, Serialize};

/// Kaguya global config file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    pub vault: PathBuf,
    pub prune: bool,
    pub keep_versions: u32, // Set 0 to keep all versions
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

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            vault: get_config_path::<PathBuf>(&None).unwrap_or_default(),
            prune: false,
            keep_versions: 0,
        }
    }
}
