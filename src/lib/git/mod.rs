use crate::{errors::Result, Dotbak};

/// Public git API for the program.
impl Dotbak {
    /// Initialize the git repository. `force` will force the initialization of the repository: if the
    /// repository is already initialized, it will delete the current repository and initialize a new one.
    /// If `force` is `false`, it will return an error if the repository is already initialized.
    pub fn init_git_repo(&self, force: bool) -> Result<()> {
        // let repo_path = Config::repo_path()?;

        Ok(())
    }
}
