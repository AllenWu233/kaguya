use crate::models::AddGameRequest;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a complete game configuration stored in the config file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameConfig {
    /// Game ID
    pub id: String,

    /// Friendly game name, alternative
    pub name: String,

    /// Saves and configurations paths
    pub paths: Vec<PathBuf>,

    /// Alternative comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// How many versions to keep when acting prune, cover global config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_versions: Option<i64>,
}

impl From<&AddGameRequest> for GameConfig {
    fn from(request: &AddGameRequest) -> Self {
        Self {
            id: request.id.to_string(),
            name: request
                .name
                .clone()
                .unwrap_or_else(|| request.id.to_string()),
            paths: request.paths.clone().unwrap_or_default(),
            comment: request.comment.clone(),
            keep_versions: None,
        }
    }
}

impl From<AddGameRequest> for GameConfig {
    fn from(request: AddGameRequest) -> Self {
        Self {
            id: request.id.clone(),
            name: request.name.unwrap_or_else(|| request.id.to_string()),
            paths: request.paths.unwrap_or_default(),
            comment: request.comment,
            keep_versions: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VaultConfig {
    pub games: Vec<GameConfig>,
}
