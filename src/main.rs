mod dotback;

use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Input};
use dotback::config::Config;
use std::io;

/// Manage and backup dotfiles with ease!
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[command(subcommand)]
    action: Action,
}

/// The collection of subcommands/actions one can use with dotback.
#[derive(Debug, Subcommand)]
enum Action {
    /// Initialize dotback for your current home directory.
    Init {
        // /// Where the '.dotback' directory lives.
        // #[arg(short, long, default_value = "~/.dotback")]
        // location: PathBuf,
        /// The remote Git repository where the dotfiles are stored.
        /// TODO: Maybe git url type?
        #[arg(short, long)]
        repository: Option<String>,
    },
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Init {
            // location,
            repository,
        } => {
            // Get the repository from the user if it is not provided already.
            let repository = match repository {
                Some(r) => r,
                None => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Git repo where dotfiles are stored")
                    .with_initial_text("https://github.com/username/repository")
                    .interact_text()?,
            };

            // TODO: business logic
            todo!();

            println!(
                "Done! dotback is now initialized at '~/.dotback', and syncs dotfiles to '{}'.",
                // location.display(),
                repository
            );
            println!(
                "Note that the default synced dotfiles are '.dotback/*' and '.config/*', not including subdirectories"
            );
            println!("To start syncing more dotfiles, run 'dotback add <dotfile>', or run 'dotback -h' for more information.");
        }
    }

    Ok(())
}
