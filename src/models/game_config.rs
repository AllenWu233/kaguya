use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a request to add a new game, coming directly from the CLI
#[derive(Debug)]
pub struct AddGameRequest {
    pub id: String,
    pub name: Option<String>,
    pub paths: Vec<PathBuf>,
    pub comment: Option<String>,
}

/// Represents a complete game configuration stored in the config file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameConfig {
    /// Game ID
    pub id: String,

    /// Friendly game name, alternative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Saves and configurations paths
    pub paths: Vec<PathBuf>,

    /// Alternative comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// How many versions to keep when acting prune, cover global config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_versions: Option<u32>,
}

impl GameConfig {
    /// Creates a 'GameConfig' from 'AddGameRequest'
    pub fn from_request(request: AddGameRequest) -> Self {
        Self {
            id: request.id,
            name: request.name,
            paths: request.paths,
            comment: request.comment,
            keep_versions: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GamesFile {
    pub games: Vec<GameConfig>,
}
