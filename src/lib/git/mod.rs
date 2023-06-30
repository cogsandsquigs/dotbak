mod tests;

use crate::{errors::Result, Dotbak};
use gix::{self, Repository};
use std::path::Path;

/// Public API for Dotbak.
impl Dotbak {
    /// Initialize the git repository. It will return an error if the repository is already initialized.
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will be created.
    pub fn init_repo<P>(path: P) -> Result<Repository>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        // Create the directory if it does not exist.
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        // Get the main repository object.
        let repo = gix::init(path)?;

        Ok(repo)
    }
}

// TODO: Implement this, but move symlinked dotfiles to their resp. locations.
// /// Delete the git repository. It will return an error if the repository is not initialized or is not
// /// there. Will not return an error if the repository is not empty.
// pub fn delete_repo<P>(path: P) -> Result<()>
// where
//     P: AsRef<Path>,
// {
//     // Delete the repository using `fs::remove_dir_all`.
//     std::fs::remove_dir_all(path)?;

//     Ok(())
// }
