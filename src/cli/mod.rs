//! CLI entry

// pub use handlers::config::handle_config;
pub use handlers::config::handle_config;
pub use parser::{Cli, Commands, ConfigSubcommands};

pub mod handlers;
pub mod parser;
