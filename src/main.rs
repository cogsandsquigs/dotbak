mod cli;
mod config;
mod errors;

use crate::config::Config;
use clap::Parser;
use cli::Cli;
use miette::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load_config()?;

    Ok(())
}
