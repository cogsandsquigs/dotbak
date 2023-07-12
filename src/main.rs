mod cli;

use clap::Parser;
use cli::{Action, Cli};
use dotbak::Dotbak;
use miette::Result;

fn main() -> Result<()> {
    // Initialize pretty_env_logger with the custom variable DOTBAK_LOG (so that it doesn't conflict with other
    // env_logger instances in other binaries).
    pretty_env_logger::init_custom_env("DOTBAK_LOG");

    let cli = Cli::parse();

    // If the command is to deinitialize, then do so.
    if matches!(cli.action, Action::Deinit) {
        todo!();
    }

    // Otherwise, we continue along our happy path

    // Initialize the `Dotbak` instance depending on what the user wants.
    let _dotbak = match cli.action {
        Action::Init { .. } => Dotbak::init()?,
        _ => todo!(),
    };

    Ok(())
}
