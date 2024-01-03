mod cli;
mod config;
mod dotbak;
mod errors;
mod files;
mod git;
mod spinners;

use clap::Parser;
use cli::Cli;
use miette::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.run()?;

    Ok(())
}
