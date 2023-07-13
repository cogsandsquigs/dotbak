use std::path::PathBuf;

use clap::Parser;
use dotbak::{errors::Result, Dotbak};
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The action to perform
    #[clap(subcommand)]
    pub action: Action,
}

impl Cli {
    /// Get the dotbak structure depending on the action.
    pub fn get_dotbak(&self) -> Result<Dotbak> {
        // Initialize the `Dotbak` instance depending on what the user wants.
        match &self.action {
            // If we are initializing, then just initialize.
            Action::Init { repo_url: None } => Dotbak::init(),

            // If we're provided a repository URL, then clone it.
            Action::Clone { repo_url }
            | Action::Init {
                repo_url: Some(repo_url),
            } => Dotbak::clone(repo_url),

            // Otherwise, we just load the instance.
            _ => Dotbak::load(),
        }
    }

    /// Runs the command-line interface for `dotbak` based on the user's input.
    pub fn run(&self) -> Result<()> {
        // Get the dotbak instance.
        let mut dotbak = self.get_dotbak()?;

        // Run the action.
        match &self.action {
            // Do nothing if we've already initialized.
            Action::Init { .. } | Action::Clone { .. } => (),

            // Add the files.
            Action::Add { paths } => {
                dotbak.add(paths)?;
            }

            // Synchonize the files.
            Action::Sync => {
                dotbak.sync()?;
            }

            // Remove the files.
            Action::Remove { paths } => {
                dotbak.remove(paths)?;
            }

            // Push changes to remote.
            Action::Push => {
                dotbak.push()?;
            }

            // Pull changes from remote.
            Action::Pull => {
                dotbak.pull()?;
            }

            // Run an arbitrary git command.
            Action::Git { args } => {
                dotbak.arbitrary_git_command(&args.iter().map(|s| s.as_str()).collect_vec())?;
            }

            // Deinitialize `dotbak`.
            Action::Deinit => {
                dotbak.deinit()?;
            }
        }

        Ok(())
    }
}

#[derive(Parser)]
pub enum Action {
    /// Initializes a new instance of `dotbak` in your home directory (at `~/.dotbak`).
    Init {
        /// The URL of the repository to clone. This is essentially the same as 'dotbak clone <REPO_URL>'.
        #[arg(short, long)]
        repo_url: Option<String>,
    },

    /// Clones an instance of `dotbak` from the given URL to your home directory (at `~/.dotbak`).
    Clone {
        /// The URL of the repository to clone.
        repo_url: String,
    },

    /// Adds files to the repository.
    Add {
        /// The paths to the files to add.
        paths: Vec<PathBuf>,
    },

    /// Synchonizes the home directory with the repository.
    Sync,

    /// Removes files from the repository.
    Remove {
        /// The paths to the files to remove.
        paths: Vec<PathBuf>,
    },

    /// Pushes the repository to the remote.
    Push,

    /// Pulls the repository from the remote.
    Pull,

    /// Runs an arbitrary git command on the repository, as if you were in the repository directory.
    /// TODO: this does not work with flags passed to git.
    Git {
        /// The arguments to pass to git.
        args: Vec<String>,
    },

    /// Deinitializes an instance of `dotbak` in your home directory.
    Deinit,
}
