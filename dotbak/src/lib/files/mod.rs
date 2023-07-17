mod tests;

use crate::errors::{
    io::{CreateSnafu, DeleteSnafu, IoError, MoveSnafu, SymlinkSnafu},
    Result,
};
use itertools::Itertools;
use snafu::ResultExt;
use std::{
    fs,
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
};

/// This structure is used to manage the files/folders that `dotbak` is tracking. This does NOT manage the git repository,
/// but instead is responsible for organizing, maintaining, and updating the files/folders and their symlinks.
pub struct Files {
    /// The directory where all the files/folders from `file_dir` are symlinked to. i.e., this is where the user's home
    /// directory is.
    home_dir: PathBuf,

    /// The path to the directory that contains the files/folders. This is where all the symlinks to the files/folders
    /// in `home_dir` originate from.
    file_dir: PathBuf,
}

/// Public API for `Files`.
impl Files {
    /// Create a new instance of `Files`.
    pub fn init(home_dir: PathBuf, file_dir: PathBuf) -> Self {
        Self { home_dir, file_dir }
    }

    /// Check if a file is managed by `dotbak` in the home directory. This will check if the file is a symlink and if
    /// it's symlinked to `file_dir`.
    ///
    /// `file` is the path to the file in `home_dir`. This path must be relative to `home_dir`.
    ///
    /// Returns either `true` or `false`.
    pub fn is_managed_in_home<P>(&self, file: P) -> bool
    where
        P: AsRef<Path>,
    {
        // Get the full paths to the file in `home_dir`.
        let home_path = self.home_dir.join(file);

        // Check if the file in `home_dir` is a symlink.
        fs::symlink_metadata(&home_path)
            .and_then(|meta| match meta.file_type().is_symlink() {
                // If it's a symlink, then check if it's symlinked to `file_dir`.
                true => {
                    // Get the path that the symlink points to.
                    let symlink_path = fs::read_link(&home_path)?;

                    // Check if the symlink points to `file_dir`.
                    Ok(symlink_path.starts_with(&self.file_dir))
                }

                // If it's not a symlink, then we need to move the file.
                false => Ok(false),
            })
            // If it's not a symlink, then we need to move the file.
            .unwrap_or(false)
    }

    /// Check if a file is managed by `dotbak` and is in the `file_dir`. This will NOT check if the file is a symlink and
    /// if it's symlinked to `file_dir`.
    pub fn is_managed_in_repo<P>(&self, file: &P) -> bool
    where
        P: AsRef<Path>,
    {
        // Get the full paths to the file in `file_dir`.
        let repo_path = self.file_dir.join(file);

        // Check if the file in `file_dir` exists.
        repo_path.exists()
    }

    /// Move a file/folder from `home_dir` to `file_dir` and symlink it back to `home_dir`. If the file is already
    /// symlinked into `file_dir`, then this will do nothing.
    ///
    /// `file` is the path to the file in `home_dir`. This path must be relative to `home_dir`.
    ///
    /// Note that this creates the exact same file structure in `file_dir` as in `home_dir`. So if `file` is
    /// `[/home/user/.config/foo/bar]`, then the file will be moved to `/home/user/.dotbak/dotfiles/config/foo/bar`
    /// and symlinked back to `/home/user/.config/foo/bar`, regardless if `file` is a file or a folder. Of course,
    /// this assumes that `file_dir` is `/home/user/.dotbak/dotfiles`.
    ///
    /// Returns either an error or `Ok(())`.
    pub fn move_and_symlink<P>(&self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Filter out all the files which are already symlinked to `file_dir`.
        let files = files
            .iter()
            .filter(|file| !self.is_managed_in_home(file) && !self.is_managed_in_repo(file))
            .collect_vec();

        // Move the file from `home_dir` to `file_dir`.
        move_files(&files, &self.home_dir, &self.file_dir)?;

        // Now symlink them back to `home_dir`.
        self.symlink_back_home(&files)?;

        Ok(())
    }

    /// Symlinks the files back to `home_dir`. This will symlink the files from `file_dir` to `home_dir`.
    /// If the file is already symlinked into `home_dir`, then this will do nothing.
    ///
    /// `files` are the paths to the file in `file_dir`. These paths must be relative to `file_dir`.
    ///
    /// Returns either an error or `Ok(())`.
    pub fn symlink_back_home<P>(&self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Filter out all the files which are already symlinked to `file_dir`.
        let files = files
            .iter()
            .filter(|file| !self.is_managed_in_home(file) && self.is_managed_in_repo(file))
            .collect_vec();

        // Symlink the files from `file_dir` to `home_dir`.
        symlink_files(&files, &self.file_dir, &self.home_dir)?;

        Ok(())
    }

    /// Basically undoes `move_and_symlink`. This will move the files/folders from `file_dir` to `home_dir` and
    /// delete the symlinks in `home_dir`.
    ///
    /// `files` are the paths to the file in `file_dir`. These paths must be relative to `file_dir`.
    ///
    /// Returns either an error or `Ok(())`.
    pub fn remove_and_restore<P>(&self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // First, delete all the symlinks in `home_dir`.
        delete_files(files, &self.home_dir)?;

        // Next, move the files/folders from `file_dir` to `home_dir`.
        move_files(files, &self.file_dir, &self.home_dir)?;

        Ok(())
    }
}

/// Helper function to delete files in `dir`.
///
/// `files` contains the files with a path relative to `dir`.
///
/// `dir` is the full path to the directory.
///
/// Returns either an error or `Ok(())`.
fn delete_files<P1, P2>(files: &[P1], dir: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    // Append all the paths to `dir` to get the full path to the file/folder.
    let paths = files.iter().map(|file| dir.as_ref().join(file));

    for path in paths {
        // Delete the file.
        fs::remove_file(&path).context(DeleteSnafu { path })?;
    }

    Ok(())
}

/// Helper function to symlink files from `from` to `to`.
///
/// `file` contains the file with a path relative to `from`.
///
/// `from` and `to` are the full paths to the directories.
///
/// Returns either an error or `Ok(())`.
fn symlink_files<P1, P2, P3>(files: &[P1], from: P2, to: P3) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    // Append all the paths to `from` to get the full path to the file/folder.
    let from_paths = files.iter().map(|file| from.as_ref().join(file));

    let to_paths = files.iter().map(|file| to.as_ref().join(file));

    for (from_path, to_path) in from_paths.zip(to_paths) {
        // Create the symlink.
        match unix_fs::symlink(&from_path, &to_path) {
            // If ok, just return.
            Ok(_) => {}

            // If the error says that the file exists, then delete the file and try again.
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                fs::remove_file(&to_path).context(DeleteSnafu {
                    path: to_path.clone(),
                })?;

                unix_fs::symlink(&from_path, &to_path).context(SymlinkSnafu {
                    from: from_path,
                    to: to_path,
                })?;
            }

            // If it's any other error, then return it.
            Err(err) => {
                return Err(IoError::Symlink {
                    from: from_path,
                    to: to_path,
                    source: err,
                }
                .into())
            }
        }

        // // If the error says that the file exists, then delete the file and try again.
        // .map_err(|err| {
        //     if err.kind() == std::io::ErrorKind::AlreadyExists {
        //         fs::remove_file(&to_path).context(DeleteSnafu { path: to_path })?;

        //         unix_fs::symlink(&from_path, &to_path).context(SymlinkSnafu {
        //             from: from_path,
        //             to: to_path,
        //         })
        //     } else {
        //         Err(err)
        //     }
        // })
    }

    Ok(())
}

/// Helper function to move files from `from` to `to`.
///
/// `file` contains the file with a path relative to `from`.
///
/// `from` and `to` are the full paths to the directories.
///
/// Returns either an error or `Ok(())`.
fn move_files<P1, P2, P3>(files: &[P1], from: P2, to: P3) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    // Append all the paths to `from` to get the full path to the file/folder.
    let from_paths = files.iter().map(|file| from.as_ref().join(file));

    let to_paths = files.iter().map(|file| to.as_ref().join(file));

    for (from_path, to_path) in from_paths.zip(to_paths) {
        // Create any and all parent directories.
        fs::create_dir_all(to_path.parent().unwrap()).context(CreateSnafu {
            path: to_path.parent().unwrap().to_path_buf(),
        })?;

        // Move the file.
        fs::rename(&from_path, &to_path).context(MoveSnafu {
            from: from_path,
            to: to_path,
        })?;
    }

    Ok(())
}
