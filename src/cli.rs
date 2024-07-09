use crate::{
    dotbak::{daemon::Daemon, Dotbak},
    errors::Result,
};
use clap::Parser;
use indicatif::HumanDuration;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The action to perform
    #[clap(subcommand)]
    pub action: Action,

    /// Whether to be verbose with logging or not.
    /// Ex: printing the output of git commands.
    #[clap(short, long)]
    pub verbose: bool,
}

impl Cli {
    /// Gets the action that's currently being performed, as a human-readable string.
    pub fn action(&self) -> String {
        match &self.action {
            Action::Init { repo_url } => format!(
                "Initializing{}...",
                if repo_url.is_some() {
                    format!(" with url '{}'", repo_url.as_ref().unwrap())
                } else {
                    String::new()
                }
            ),
            Action::Clone { repo_url } => format!("Cloning with url {}", repo_url).to_string(),
            Action::Add { paths } => format!("Adding {} file(s)", paths.len()),
            Action::Sync => "Synchronizing".to_string(),
            Action::Remove { paths } => format!("Removing {} file(s)", paths.len()),
            Action::Push => "Pushing".to_string(),
            Action::Pull => "Pulling".to_string(),
            Action::Git { args } => format!("Running 'git {}'", args.join(" ")),
            Action::Deinit => "Deinitializing".to_string(),
            Action::StartDaemon => "Starting daemon".to_string(),
            Action::StopDaemon => "Stopping daemon".to_string(),
        }
    }

    /// Runs the command-line interface for `dotbak` based on the user's input.
    pub fn run(&self) -> Result<()> {
        // Get the dotbak instance.
        let mut dotbak = self.get_dotbak()?;
        let started = Instant::now();

        println!("⏳ {}...", self.action());

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
                dotbak
                    .arbitrary_git_command(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
            }

            // Deinitialize `dotbak`.
            Action::Deinit => {
                dotbak.deinit()?;
            }

            // Run the daemon, don't use `dotbak` result.
            Action::StartDaemon => {
                Daemon::new()?.run();
            }

            // Stop the daemon, don't use `dotbak` result.
            Action::StopDaemon => {
                Daemon::stop()?;
            }
        }

        println!(
            "✨ Done! {}",
            console::style(format!("[{}]", HumanDuration(started.elapsed())))
                .bold()
                .dim(),
        );

        Ok(())
    }
}

impl Cli {
    /// Get the dotbak structure depending on the action.
    fn get_dotbak(&self) -> Result<Dotbak> {
        // Initialize the `Dotbak` instance depending on what the user wants.
        match &self.action {
            // If we are initializing, then just initialize.
            Action::Init { repo_url: None } => Dotbak::init(self.verbose),

            // If we're provided a repository URL, then clone it.
            Action::Clone { repo_url }
            | Action::Init {
                repo_url: Some(repo_url),
            } => Dotbak::clone(repo_url, self.verbose),

            // Otherwise, we just load the instance.
            _ => Dotbak::load(self.verbose),
        }
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

    /// Runs a daemon variant of `dotbak`.
    StartDaemon,

    /// Stops the daemon variant of `dotbak`.
    StopDaemon,
}
