mod tests;

use crate::{errors::Result, Dotbak};
use gix::url::Url;
use gix::{self, Repository};
use std::path::Path;

/// Public API for Dotbak.
impl Dotbak {
    /// Initialize a new git repository. It will return an error if the repository is already initialized.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will be created.
    /// TODO: implement logging and such.
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

    /// Clones a pre-existing repository from a remote location. It will return an error if the repository
    /// is already initialized.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will be created.
    ///
    /// `url` is the URL to the remote repository.
    /// TODO: implement logging and such.
    pub fn clone_repo<P>(path: P, url: Url) -> Result<Repository>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        // Create the directory if it does not exist.
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        // println!("Url: {:?}", url.to_bstring());

        let mut prepare_clone = gix::prepare_clone(url, path)?; // TODO: get rid of clone.

        // println!("Cloning {url:?} into {path:?}...");

        let (mut prepare_checkout, _) = prepare_clone
            .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
        // TODO: log progress.

        // println!(
        //     "Checking out into {:?} ...",
        //     prepare_checkout.repo().work_dir().expect("should be there")
        // );

        let (repo, _) = prepare_checkout
            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

        // println!(
        //     "Repo cloned into {:?}",
        //     repo.work_dir().expect("directory pre-created")
        // );

        // let remote: Remote = repo
        //     .find_default_remote(gix::remote::Direction::Fetch)
        //     .expect("always present after clone")?;

        // println!(
        //     "Default remote: {} -> {}",
        //     remote
        //         .name()
        //         .expect("default remote is always named")
        //         .as_bstr(),
        //     remote
        //         .url(gix::remote::Direction::Fetch)
        //         .expect("should be the remote URL")
        //         .to_bstring(),
        // );

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
