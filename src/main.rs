mod cli;

use clap::Parser;
use cli::Cli;
use dotbak::config::Config;
use miette::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load_config()?;

    Ok(())
}
