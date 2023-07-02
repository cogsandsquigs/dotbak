mod tests;

use crate::{
    errors::{
        git::{CloneSnafu, InitSnafu},
        io::{CreateSnafu, DeleteSnafu, IoError},
        Result,
    },
    Dotbak,
};
use git2::{build::RepoBuilder, Repository, RepositoryInitOptions};
use snafu::ResultExt;
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
            std::fs::create_dir_all(path).context(CreateSnafu {
                path: path.to_path_buf(),
            })?;
        }

        // Set the options.
        let mut opts = RepositoryInitOptions::new();
        opts.no_reinit(true);

        // Get the main repository object.
        let repo = Repository::init_opts(path, &opts).context(InitSnafu {
            path: path.to_path_buf(),
            url: None,
        })?;

        Ok(repo)
    }

    /// Loads a pre-existing repository from a local location. It will return an error if the repository
    /// is not initialized or is not there.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will return an error.
    pub fn load_repo<P>(path: P) -> Result<Repository>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        // Check if the directory exists.
        if !path.exists() {
            return Err(IoError::NotFound {
                path: path.to_path_buf(),
            }
            .into());
        }

        // Get the main repository object.
        let repo = Repository::open(path).context(InitSnafu {
            path: path.to_path_buf(),
            url: None,
        })?;

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
    pub fn clone_repo<P, S>(path: P, url: S) -> Result<Repository>
    where
        P: AsRef<Path>,
        S: ToString,
    {
        let path = path.as_ref();
        let url = url.to_string();

        // Create the directory if it does not exist.
        if !path.exists() {
            std::fs::create_dir_all(path).context(CreateSnafu {
                path: path.to_path_buf(),
            })?;
        }

        // Get the main repository object.
        // let repo = Repository::clone(&url, path).context(CloneSnafu { url })?;

        let repo = RepoBuilder::new()
            .fetch_options({
                let mut fo = git2::FetchOptions::new();
                fo.remote_callbacks({
                    let mut cb = git2::RemoteCallbacks::new();
                    cb.credentials(git_credentials_callback);
                    cb
                });
                fo
            })
            .clone(&url, path)
            .context(CloneSnafu {
                path: path.to_path_buf(),
                url,
            })?;

        Ok(repo)
    }

    /// Deletes the git repository. It will return an error if the repository is not initialized or is not
    /// there. Will not return an error if the repository is not empty.
    /// TODO: implement logging and such.
    /// TODO: Move symlinked files to their original location.
    pub fn delete_repo<P>(path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        // Delete the repository using `fs::remove_dir_all`.
        std::fs::remove_dir_all(path).context(DeleteSnafu {
            path: path.to_path_buf(),
        })?;

        Ok(())
    }
}

/// Credentials callback for git2, so we can use SSH keys/clone private repos.
pub fn git_credentials_callback(
    user: &str,
    user_from_url: Option<&str>,
    cred: git2::CredentialType,
) -> std::result::Result<git2::Cred, git2::Error> {
    let user = user_from_url.unwrap_or(user);

    if cred.contains(git2::CredentialType::USERNAME) {
        git2::Cred::username(user)
    } else {
        git2::Cred::ssh_key_from_agent(user)
    }
}
