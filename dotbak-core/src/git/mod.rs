mod tests;

use crate::errors::{
    git::{CloneSnafu, CommitSnafu, GitError, InitSnafu, RemoteSnafu},
    io::{CommandIOSnafu, CreateSnafu, DeleteSnafu, IoError},
    Result,
};
use git2::{
    build::RepoBuilder, Commit, Cred, CredentialType, ErrorCode, FetchOptions, IndexAddOption, Oid,
    PushOptions, RemoteCallbacks, Repository, RepositoryInitOptions, Signature,
};
use snafu::ResultExt;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// A wrapper structure around git2's `Repository` object.
pub struct GitRepo {
    /// The repository path for `dotbak`. Note that this is not the `.git` directory, but the directory
    /// containing the `.git` directory.
    path: PathBuf,

    /// The git2 `Repository` object.
    repo: Repository,
}

/// Public git API for `GitRepo`.
impl GitRepo {
    /// Initialize a new git repository. It will return an error if the repository is already initialized.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will be created.
    /// TODO: implement logging and such.
    ///
    /// `remote_url` is the URL to the remote repository. This will be set to the `origin` remote.
    pub fn init<P>(path: P, remote_url: Option<String>) -> Result<GitRepo>
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
        // We don't want to reinitialize the repository if it already exists.
        opts.no_reinit(true);
        // If we want to set the remote, we set it here.
        if let Some(url) = remote_url {
            opts.origin_url(&url);
        }

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

    /// Set the remote for the repository. It will return an error if the repository is not
    /// initialized. The remote is named "origin".
    ///
    /// `url` is the URL to the remote repository.
    pub fn set_remote<S>(&mut self, url: S) -> Result<()>
    where
        S: ToString,
    {
        let url = url.to_string();

        // Set the remote.
        self.repo
            .remote_set_pushurl("origin", Some(&url))
            .context(RemoteSnafu { url })?;

        Ok(())
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

    /// Runs an arbitrary `git` command. It will return an error if the repository is not initialized.
    ///
    /// `args` is a vector of arguments to pass to `git`.
    pub fn arbitrary_command(&self, args: &[&str]) -> Result<()> {
        // Run the command.
        let output = std::process::Command::new("git")
            .args(args)
            .current_dir(&self.path)
            .output()
            .context(CommandIOSnafu {
                command: "git".to_string(),
                args: args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            })?;

        // Check if the command failed.
        if !output.status.success() {
            return Err(IoError::CommandRun {
                command: "git".to_string(),
                args: args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Commits all changed files to the repository. It will return an error if the repository is not initialized.
    ///
    /// `message` is the commit message.
    ///
    /// Returns the commit's OID -- this is the commit's hash.
    pub fn commit(&mut self, message: &str) -> Result<Oid> {
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
        let parents = self.parents()?;

        // Get the signature.
        let signature = self.signature()?;

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

    /// Pushes all commits to the remote repository. It will return an error if the repository is not
    /// initialized.
    pub fn push(&mut self) -> Result<()> {
        // Get the remote.
        let mut remote = self.repo.find_remote("origin").context(CommitSnafu)?;

        // Set the callbacks.
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(git_credentials_callback);

        // Set the options.
        let mut options = PushOptions::new();
        options.remote_callbacks(callbacks);

        // Push the remote.
        remote
            .push(&["refs/heads/main:refs/heads/main"], Some(&mut options))
            .context(CommitSnafu)?;

        Ok(())
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

/// Private git API for `GitRepo`.
impl GitRepo {
    /// Gets the parents of the current HEAD. If there is no HEAD, it will return an empty vector.
    fn parents(&self) -> Result<Vec<Commit<'_>>> {
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
                Ok(vec![parent])
            }

            // If this is a newly initialized repository, there will be no HEAD. Thus,
            // we return no parents.
            Err(e) if e.code() == ErrorCode::UnbornBranch => {
                // Return an empty vector.
                Ok(vec![])
            }

            // If this is an actual error, return it.
            Err(e) => Err(GitError::Commit { source: e }.into()),
        }
    }

    /// Gets the signature for the current user.
    fn signature(&self) -> Result<Signature<'_>> {
        match self.repo.signature() {
            // If the signature exists, return it.
            Ok(signature) => Ok(signature),

            // If the signature doesn't exist, return a default signature IF in CI environment. This
            // is to get around not having a signature set up in the CI.
            Err(e)
                if e.code() == ErrorCode::NotFound
                    && env::var("CI")
                        // If the variable is not set, default to `""`.
                        .unwrap_or_default()
                        // Parse a bool from the variable.
                        .parse()
                        // If the variable is not a valid bool, default to `false`.
                        .unwrap_or_default() =>
            {
                // Get the username.
                let username = format!(
                    "dotbak-ci-build-{}",
                    env::var("CIRCLE_BUILD_NUM")
                        .expect("CIRCLE_BRANCH should be set, as we are using CircleCI!")
                );

                // Get the email.
                let email = format!(
                    "{}@circleci-branch-{}",
                    username,
                    env::var("CIRCLE_BRANCH")
                        .expect("CIRCLE_BRANCH should be set, as we are using CircleCI!")
                );

                // Create the signature.
                let signature = Signature::now(&username, &email).context(CommitSnafu)?;

                Ok(signature)
            }

            // If this is an actual error, return it.
            Err(e) => Err(GitError::Commit { source: e }.into()),
        }
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
