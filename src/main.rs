mod cli;
mod config;
mod errors;

use crate::config::Config;
use clap::Parser;
use cli::Cli;
use errors::DotbackError;

fn main() -> Result<(), DotbackError> {
    let cli = Cli::parse();
    let config: Config = confy::load("dotback", None)?;

    Ok(())
}
