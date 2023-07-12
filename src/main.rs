mod cli;

use clap::Parser;
use cli::{Action, Cli};
use dotbak::Dotbak;
use miette::Result;

fn main() -> Result<()> {
    // // Initialize pretty_env_logger with the custom variable DOTBAK_LOG (so that it doesn't conflict with other
    // // env_logger instances in other binaries).
    // pretty_env_logger::init_custom_env("DOTBAK_LOG");

    let cli = Cli::parse();

    // If the command is to deinitialize, then do so.
    if matches!(cli.action, Action::Deinit) {
        todo!();
    }

    // Otherwise, we continue along our happy path

    // Initialize the `Dotbak` instance depending on what the user wants.
    let dotbak = match cli.action {
        // If we are initializing, then just initialize.
        Action::Init { repo_url: None } => Dotbak::init()?,

        // If we're provided a repository URL, then clone it.
        Action::Clone { repo_url }
        | Action::Init {
            repo_url: Some(repo_url),
        } => Dotbak::clone(&repo_url)?,

        // Otherwise, we just load the instance.
        _ => Dotbak::load()?,
    };

    todo!();

    Ok(())
}
