mod cli;
mod config;
mod errors;

use crate::config::Config;
use clap::Parser;
use cli::Cli;
use errors::DotbakError;

fn main() -> Result<(), DotbakError> {
    let cli = Cli::parse();

    Ok(())
}
