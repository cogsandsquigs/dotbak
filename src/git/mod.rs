mod tests;

use crate::errors::{io::IoError, DotbakError, Result};
use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Output,
};

/// The default remote name.
pub const REMOTE_NAME: &str = "origin";

/// The default main branch name.
pub const MAIN_BRANCH_NAME: &str = "main";

/// A git repository. This is essentially a wrapper structure around git commands performed on the repository,
/// and is not a wrapper around the git2 library. This is because when I tried to work with `git2`, I ran into
/// issues pulling and pushing to the remote repository. I'm not sure if this is a bug with `git2` or if I'm just
/// using it wrong, but I decided to just use the raw `git` command instead. This is much easier and simpler.
#[derive(Debug)]
pub struct Repository {
    /// The repository path for `dotbak`. Note that this is not the `.git` directory, but the directory
    /// containing the `.git` directory.
    path: PathBuf,
}

/// Public git API for `Repository`.
impl Repository {
    /// Initialize a new git repository. It will not return an error if the repository is already initialized.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will be created.
    /// TODO: implement logging and such.
    ///
    /// `remote_url` is the URL to the remote repository. This will be set to the `origin` remote.
    pub fn init<P>(path: P, remote_url: Option<String>) -> Result<Repository>
    where
        P: AsRef<Path>,
    {
        // Create the directory if it does not exist.
        if !path.as_ref().exists() {
            fs::create_dir_all(path.as_ref()).map_err(|err| IoError::Create {
                source: err,
                path: path.as_ref().to_path_buf(),
            })?;
        }

        // Run the init command.
        run_arbitrary_git_command(
            path.as_ref(),
            &["init", "--initial-branch", MAIN_BRANCH_NAME, "."],
        )?;

        // Create the repository.
        let mut repo = Repository {
            path: path.as_ref().to_path_buf(),
        };

        // If we want to set the remote, we set it here.
        if let Some(url) = remote_url {
            repo.set_remote(url)?;
        }

        Ok(repo)
    }

    /// Loads a pre-existing repository from a local location. It will return an error if the repository
    /// is not initialized or is not there.
    ///
    /// `path` is the path to the repository directory, and the repository exists inside the folder. If the
    /// directory does not exist, it will return an error.
    pub fn load<P>(path: P) -> Result<Repository>
    where
        P: AsRef<Path>,
    {
        // Check if the directory exists.
        if !path.as_ref().exists() {
            return Err(IoError::NotFound {
                path: path.as_ref().to_path_buf(),
            }
            .into());
        }

        // Check that the repository is initialized.
        // TODO: Stronger check?
        if !path.as_ref().join(".git").exists() {
            return Err(IoError::NotFound {
                path: path.as_ref().to_path_buf(),
            }
            .into());
        }

        // Return the repository.
        Ok(Repository {
            path: path.as_ref().to_path_buf(),
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
    pub fn clone<P, S>(path: P, url: S) -> Result<Repository>
    where
        P: AsRef<Path>,
        S: ToString,
    {
        let path = path.as_ref();
        let url = url.to_string();

        // Create the directory if it does not exist.
        if !path.exists() {
            fs::create_dir_all(path).map_err(|err| IoError::Create {
                source: err,
                path: path.to_path_buf(),
            })?;
        }

        // Run the clone command.
        run_arbitrary_git_command(path, &["clone", &url, "."])?;

        // Create the repository.
        let repo = Repository {
            path: path.to_path_buf(),
        };

        Ok(repo)
    }

    /// Runs an arbitrary `git` command. It will return an error if the repository is not initialized.
    ///
    /// `args` is a vector of arguments to pass to `git`.
    pub fn arbitrary_command(&mut self, args: &[&str]) -> Result<Output> {
        // Run the command.
        run_arbitrary_git_command(&self.path, args)
    }

    /// Set the remote for the repository. It will return an error if the repository is not
    /// initialized. The remote is named REMOTE_NAME.
    ///
    /// `url` is the URL to the remote repository.
    pub fn set_remote<S>(&mut self, url: S) -> Result<Output>
    where
        S: ToString,
    {
        let url = url.to_string();

        // Run the remote command.
        let result = self.arbitrary_command(&["remote", "set-url", REMOTE_NAME, &url]);

        match result {
            // If the command succeeded, return.
            Ok(output) => Ok(output),

            // If the remote could not be found, create it.
            Err(DotbakError::Io(IoError::CommandRun { stderr, .. }))
                if stderr == *"error: No such remote 'origin'\n" =>
            {
                // Run the remote command.
                self.arbitrary_command(&["remote", "add", REMOTE_NAME, &url])?;
                self.arbitrary_command(&["remote", "set-url", REMOTE_NAME, &url])
            }

            // If the command failed, return an error.
            Err(e) => Err(e),
        }
    }

    /// Commits all changed files to the repository. It will return an error if the repository is not initialized.
    ///
    /// `message` is the commit message.
    ///
    /// Returns the commit's OID -- this is the commit's hash.
    pub fn commit(&mut self, message: &str) -> Result<[Output; 2]> {
        Ok([
            // Run the add command.
            self.arbitrary_command(&["add", "."])?,
            // Run the commit command.
            self.arbitrary_command(&["commit", "-am", message])?,
        ])
    }

    /// Pushes all commits to the remote repository. It will return an error if the repository is not
    /// initialized.
    pub fn push(&mut self) -> Result<Output> {
        self.arbitrary_command(&["push", REMOTE_NAME, MAIN_BRANCH_NAME])
    }

    /// Pulls all commits from the remote repository. It will return an error if the repository is not
    /// initialized.
    pub fn pull(&mut self) -> Result<Output> {
        self.arbitrary_command(&["pull", REMOTE_NAME, MAIN_BRANCH_NAME])
    }

    /// Deletes the git repository. It will return an error if the repository is not initialized or is not
    /// there. Will not return an error if the repository is not empty.
    /// TODO: implement logging and such.
    /// TODO: Move symlinked files to their original location.
    pub fn delete(self) -> Result<()> {
        // Delete the repository using `fs::remove_dir_all`.
        fs::remove_dir_all(&self.path).map_err(|err| IoError::Delete {
            source: err,
            path: self.path,
        })?;

        Ok(())
    }
}

/// These are helper functions for tests on `Repository`.
#[cfg(test)]
impl Repository {
    /// Get the path to the repository.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Run a command in the repository.
///
/// `path` is the path to the repository.
///
/// `args` is the arguments to pass to the command.
///
/// Returns the output of the command.
fn run_arbitrary_git_command<P>(path: P, args: &[&str]) -> Result<Output>
where
    P: AsRef<Path>,
{
    // Run the command.
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .map_err(|err| IoError::CommandIO {
            source: err,
            command: "git".to_string(),
            args: args.iter().map(|s| s.to_string()).collect_vec(),
        })?;

    // If the command succeeded, return.
    if output.status.success() {
        return Ok(output);
    }

    let string_stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let string_stderr = String::from_utf8_lossy(&output.stderr).to_string();

    // Make sure that the error is not something benign like "nothing to commit".

    match string_stdout {
        // HACK: If it's an error, but the error is "nothing to commit", then return an empty output.
        // TODO: This is a hack. Fix this.
        _ if string_stdout.contains("nothing to commit") => Ok(output),

        // Otherwise, return the error.
        _ => Err(IoError::CommandRun {
            command: "git".to_string(),
            args: args.iter().map(|s| s.to_string()).collect_vec(),
            stdout: string_stdout,
            stderr: string_stderr,
        }
        .into()),
    }
}
