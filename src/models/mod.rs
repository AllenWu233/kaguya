//! Config / Request / Service struct, constants, and custom error type

pub use constants::*;
pub use error::KaguyaError;
pub use game_config::{AddGameRequest, GameConfig, GamesFile};

pub mod constants;
pub mod error;
pub mod game_config;
pub mod global_config;
