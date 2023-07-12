use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The action to perform
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Parser)]
pub enum Action {
    /// Initializes a new instance of `dotbak` in the user's home directory.
    Init {
        /// The URL of the repository to clone.
        #[clap(short, long)]
        repo_url: Option<String>,
    },

    /// Clones an instance of `dotbak` from the given URL to the user's home directory.
    /// The user's home directory is assumed to be `<home>`.
    Clone {
        /// The URL of the repository to clone.
        #[clap(short, long)]
        repo_url: String,
    },

    /// Deinitializes an instance of `dotbak` in the user's home directory.
    Deinit,
}
