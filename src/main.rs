mod cli;
mod config;
mod dotbak;
mod errors;
mod files;
mod git;
mod test_util;

use clap::Parser;
use cli::Cli;
use miette::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.run()?;

    Ok(())
}
