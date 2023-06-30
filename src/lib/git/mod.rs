use crate::{errors::Result, locations::repo_path, Dotbak};
use gix;

/// Public git API for the program.
impl Dotbak {
    /// Initialize the git repository. `force` will force the initialization of the repository: if the
    /// repository is already initialized, it will delete the current repository and initialize a new one.
    /// If `force` is `false`, it will return an error if the repository is already initialized.
    pub fn init_git_repo(&self, force: bool) -> Result<()> {
        let repo_path = repo_path()?;

        // Get the main repository object.
        let repo = gix::init_bare(repo_path)?;

        // let repo_path = Config::repo_path()?;

        Ok(())
    }
}
