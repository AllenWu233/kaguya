//! Config / Request / Service struct, constants, and custom error type

pub use constants::*;
pub use db::{Game, GamePath};
pub use error::KaguyaError;
pub use game_config::{GameConfig, GameConfigFile};
pub use requests::{AddGameRequest, BackupRequest, ListGameRequest, RmGameRequest};

pub mod constants;
pub mod db;
pub mod error;
pub mod game_config;
pub mod global_config;
pub mod requests;
