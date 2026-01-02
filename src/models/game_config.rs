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

impl<'a> From<&AddGameRequest<'a>> for GameConfig {
    /// Creates a [`GameConfigFile`] from [`AddGameRequest`]
    fn from(request: &AddGameRequest) -> Self {
        Self {
            id: request.id.to_string(),
            name: request
                .name
                .map_or_else(|| request.id.to_string(), |n| n.to_string()),
            paths: request.paths.map(|p| p.to_vec()).unwrap_or_default(),
            comment: request.comment.map(|c| c.to_string()),
            keep_versions: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameConfigFile {
    pub games: Vec<GameConfig>,
}
