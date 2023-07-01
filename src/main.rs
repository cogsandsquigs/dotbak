mod cli;

use clap::Parser;
use cli::Cli;
use dotbak_core::{config::Config, Dotbak};
use miette::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dotbak = Dotbak::init()?;

    Ok(())
}
