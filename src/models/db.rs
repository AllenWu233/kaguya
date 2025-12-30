use chrono::{DateTime, Utc};

use crate::models::GameConfig;

#[derive(Debug)]
pub struct Game {
    /// ID for the table
    pub id: Option<i64>,

    /// Game ID, e.g., "outer_wilds"
    pub external_id: String,

    /// Friendly game name
    pub name: Option<String>,

    /// Alternative comment
    pub comment: Option<String>,

    /// How many versions to keep when acting prune, cover global config
    pub keep_versions: Option<i64>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<GameConfig> for Game {
    fn from(game: GameConfig) -> Self {
        let created_at = Utc::now();
        let updated_at = created_at;

        Self {
            id: None,
            external_id: game.id,
            name: game.name,
            comment: game.comment,
            keep_versions: game.keep_versions,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GamePath {
    pub id: Option<i64>,
    pub game_id: Option<i64>,
    pub original_path: String,
}
