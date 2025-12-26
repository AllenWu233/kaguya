use clap::Parser;
use kaguya::cli::{self, AppContext, Cli, Commands};
use kaguya::models::KaguyaError;

fn main() -> Result<(), KaguyaError> {
    let cli = Cli::parse();
    let context = AppContext::from_cli(&cli)?;

    // dbg!(&cli);

    match &cli.command {
        Commands::Init => todo!(),

        Commands::Completion => todo!(),

        Commands::Config(subcommand) => {
            cli::handle_config(subcommand, &context)?;
        }
    }

    Ok(())
}
