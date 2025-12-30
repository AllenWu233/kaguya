use clap::Parser;
use kaguya::cli::{self, AppContext, Cli, Commands};
use kaguya::models::KaguyaError;

fn main() -> Result<(), KaguyaError> {
    let cli = Cli::parse();
    let context = AppContext::new(&cli)?;

    // dbg!(&cli, &context);

    match &cli.command {
        Commands::Completion => todo!("Generate shell completion"),

        Commands::Config(subcommand) => {
            cli::handle_config(subcommand, &context)?;
        }

        Commands::Vault(subcommand) => {
            cli::handle_vault(subcommand, &context)?;
        }
    }

    Ok(())
}
