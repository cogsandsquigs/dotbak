mod cli;

use clap::Parser;
use cli::Cli;
use dotbak::Dotbak;
use miette::Result;

fn main() -> Result<()> {
    let _cli = Cli::parse();
    let _dotbak = Dotbak::init()?;

    Ok(())
}
