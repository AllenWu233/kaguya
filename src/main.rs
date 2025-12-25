use clap::Parser;
use kaguya::cli::{self, Cli, Commands};
use kaguya::models::KaguyaError;

fn main() -> Result<(), KaguyaError> {
    let cli = Cli::parse();
    dbg!(&cli);

    match &cli.command {
        Commands::Init => todo!(),

        Commands::Completion => todo!(),

        Commands::Config(subcommand) => {
            cli::handle_config(subcommand, &cli)?;
        }
    }

    Ok(())
}
