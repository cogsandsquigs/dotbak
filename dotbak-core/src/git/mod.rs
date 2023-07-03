mod tests;

use crate::errors::{
    git::{CloneSnafu, CommitSnafu, GitError, InitSnafu},
    io::{CreateSnafu, DeleteSnafu, IoError},
    Result,
};
use git2::{
    build::RepoBuilder, Cred, CredentialType, ErrorCode, FetchOptions, IndexAddOption, Oid,
    RemoteCallbacks, Repository, RepositoryInitOptions,
};
use snafu::ResultExt;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// A wrapper structure around git2's `Repository` object.
pub struct GitRepo {
    /// The repository path for `dotbak`.
    path: PathBuf,

    /// The git2 `Repository` object.
    repo: Repository,
}

/// Public git API for `Dotbak`.
impl GitRepo {
    /// Initialize a new git repository. It will return an error if the repository is already initialized.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will be created.
    /// TODO: implement logging and such.
    pub fn init<P>(path: P) -> Result<GitRepo>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        // Create the directory if it does not exist.
        if !path.exists() {
            fs::create_dir_all(path).context(CreateSnafu {
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

        Ok(GitRepo {
            path: path.to_path_buf(),
            repo,
        })
    }

    /// Loads a pre-existing repository from a local location. It will return an error if the repository
    /// is not initialized or is not there.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will return an error.
    pub fn load<P>(path: P) -> Result<GitRepo>
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

        Ok(GitRepo {
            path: path.to_path_buf(),
            repo,
        })
    }

    /// Clones a pre-existing repository from a remote location. It will return an error if the repository
    /// is already initialized.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will be created.
    ///
    /// `url` is the URL to the remote repository.
    /// TODO: implement logging and such.
    pub fn clone<P, S>(path: P, url: S) -> Result<GitRepo>
    where
        P: AsRef<Path>,
        S: ToString,
    {
        let path = path.as_ref();
        let url = url.to_string();

        // Create the directory if it does not exist.
        if !path.exists() {
            fs::create_dir_all(path).context(CreateSnafu {
                path: path.to_path_buf(),
            })?;
        }

        // Get the main repository object.
        // let repo = Repository::clone(&url, path).context(CloneSnafu { url })?;

        let repo = RepoBuilder::new()
            .fetch_options({
                let mut fo = FetchOptions::new();
                fo.remote_callbacks({
                    let mut cb = RemoteCallbacks::new();
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

        Ok(GitRepo {
            path: path.to_path_buf(),
            repo,
        })
    }

    /// Gets the path at which the repository is located.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Commits all changed files to the repository. It will return an error if the repository is not initialized.
    ///
    /// `message` is the commit message.
    ///
    /// Returns the commit's OID -- this is the commit's hash.
    pub fn commit(&self, message: &str) -> Result<Oid> {
        // Get the index.
        let mut index = self.repo.index().context(CommitSnafu)?;

        // Add all files to the index.
        index
            .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .context(CommitSnafu)?;

        // Write the index.
        index.write().context(CommitSnafu)?;

        // Get the tree.
        let tree_id = index.write_tree().context(CommitSnafu)?;

        // Get the parent.
        let parents = {
            // Get the HEAD.
            let head = self.repo.head();

            // We check if the HEAD exists. This is because in a newly initialized repository, there will be
            // no HEAD.
            match head {
                // If the HEAD exists, get the parent commit.
                Ok(head) => {
                    // Get the commit.
                    let parent = head.peel_to_commit().context(CommitSnafu)?;

                    // Return the commit.
                    vec![parent]
                }

                // If this is a newly initialized repository, there will be no HEAD. Thus,
                // we return no parents.
                Err(e) if e.code() == ErrorCode::UnbornBranch => {
                    // Return an empty vector.
                    vec![]
                }

                // If this is an actual error, return it.
                Err(e) => return Err(GitError::Commit { source: e }.into()),
            }
        };

        // Get the signature.
        let signature = self.repo.signature().context(CommitSnafu)?.to_owned();

        // Create the commit.
        let oid = self
            .repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &self.repo.find_tree(tree_id).context(CommitSnafu)?,
                // HACK: This makes it so all parents are passed as a slice of references.
                &parents.iter().collect::<Vec<_>>(),
            )
            .context(CommitSnafu)?;

        Ok(oid)
    }

    /// Deletes the git repository. It will return an error if the repository is not initialized or is not
    /// there. Will not return an error if the repository is not empty.
    /// TODO: implement logging and such.
    /// TODO: Move symlinked files to their original location.
    pub fn delete(self) -> Result<()> {
        // Delete the repository using `fs::remove_dir_all`.
        fs::remove_dir_all(&self.path).context(DeleteSnafu { path: self.path })?;

        Ok(())
    }
}

/// Credentials callback for git2, so we can use SSH keys/clone private repos.
fn git_credentials_callback(
    user: &str,
    user_from_url: Option<&str>,
    cred: CredentialType,
) -> std::result::Result<Cred, git2::Error> {
    let user = user_from_url.unwrap_or(user);

    if cred.contains(CredentialType::USERNAME) {
        Cred::username(user)
    } else {
        Cred::ssh_key_from_agent(user)
    }
}
