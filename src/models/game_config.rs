use crate::models::AddGameRequest;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
            id: request.id.to_string(),
            name: request.name.map(|n| n.to_string()),
            paths: {
                if request.paths.is_some() {
                    request.paths.unwrap().to_vec()
                } else {
                    Vec::<PathBuf>::new()
                }
            },
            comment: request.comment.map(|c| c.to_string()),
            keep_versions: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GamesFile {
    pub games: Vec<GameConfig>,
}
