mod tests;

use crate::errors::{
    io::{DeleteSnafu, FsExtraSnafu, SymlinkSnafu},
    Result,
};
use fs_extra::dir::CopyOptions;
use itertools::Itertools;
use snafu::ResultExt;
use std::path::{Path, PathBuf};

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

    /// Move a set of files/folders from `home_dir` to `file_dir` and symlink it back to `home_dir`.
    ///
    /// `files` are the paths to the file in `home_dir`. These paths must be relative to `home_dir`.
    ///
    /// Note that this creates the exact same file structure in `file_dir` as in `home_dir`. So if `files` is
    /// `[/home/user/.config/foo/bar]`, then the file will be moved to `/home/user/.dotbak/dotfiles/config/foo/bar`
    /// and symlinked back to `/home/user/.config/foo/bar`, regardless if `file` is a file or a folder. Of course,
    /// this assumes that `file_dir` is `/home/user/.dotbak/dotfiles`.
    ///
    /// Returns either an error or `Ok(())`.
    /// TODO: Exclude files in the `exclude` list.
    pub fn move_and_symlink<P>(&self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Move the files from `home_dir` to `file_dir`.
        move_files(files, &self.home_dir, &self.file_dir)?;

        // Now symlink them back to `home_dir`. Note that `file` is the relative path to the file/folder in `home_dir`.
        for file in files {
            let file = file.as_ref();

            // Get the full path to the file/folder in `file_dir`.
            let file_dir_path = self.file_dir.join(file);

            // Get the full path to the file/folder in `home_dir`.
            let home_dir_path = self.home_dir.join(file);

            // Create the symlink.
            std::os::unix::fs::symlink(&file_dir_path, &home_dir_path).context(SymlinkSnafu {
                from: file_dir_path,
                to: home_dir_path,
            })?;
        }

        Ok(())
    }

    /// Basically undoes `move_and_symlink`. This will move the files/folders from `file_dir` to `home_dir` and
    /// delete the symlinks in `home_dir`.
    ///
    /// `files` are the paths to the file in `home_dir`. These paths must be relative to `home_dir`.
    ///
    /// Returns either an error or `Ok(())`.
    pub fn remove_and_restore<P>(&self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // First, we delete all the symlinks in `home_dir`.
        for file in files {
            let file = file.as_ref();

            // Get the full path to the file/folder in `home_dir`.
            let home_dir_path = self.home_dir.join(file);

            // Delete the symlink.
            std::fs::remove_file(&home_dir_path).context(DeleteSnafu {
                path: home_dir_path,
            })?;
        }

        move_files(files, &self.file_dir, &self.home_dir)?;

        Ok(())
    }
}

/// Helper function to move files from `from` to `to`.
///
/// `files` contains all the files with paths relative to `from`.
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
    let from_paths = files
        .iter()
        .map(|file| from.as_ref().join(file))
        .collect_vec();

    // Move the file/folder from `from` to `to`.
    fs_extra::move_items(&from_paths, &to, &CopyOptions::default()).context(FsExtraSnafu)?;

    Ok(())
}
