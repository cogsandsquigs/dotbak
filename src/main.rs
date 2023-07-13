mod cli;

use clap::Parser;
use cli::Cli;
use miette::Result;

fn main() -> Result<()> {
    // // Initialize pretty_env_logger with the custom variable DOTBAK_LOG (so that it doesn't conflict with other
    // // env_logger instances in other binaries).
    // pretty_env_logger::init_custom_env("DOTBAK_LOG");

    let cli = Cli::parse();

    // Get the dotbak instance.
    let dotbak = cli.get_dotbak()?;

    todo!();

    Ok(())
}
