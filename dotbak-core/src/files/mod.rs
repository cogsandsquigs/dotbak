mod tests;

use crate::{
    config::files::FilesConfig,
    errors::{
        io::{DeleteSnafu, MoveSnafu, SymlinkSnafu},
        GlobSnafu, Result,
    },
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use snafu::ResultExt;
use std::{
    fs,
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

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

    /// Move a file/folder from `home_dir` to `file_dir` and symlink it back to `home_dir`.
    ///
    /// `file` is the path to the file in `home_dir`. This path must be relative to `home_dir`.
    ///
    /// Note that this creates the exact same file structure in `file_dir` as in `home_dir`. So if `file` is
    /// `[/home/user/.config/foo/bar]`, then the file will be moved to `/home/user/.dotbak/dotfiles/config/foo/bar`
    /// and symlinked back to `/home/user/.config/foo/bar`, regardless if `file` is a file or a folder. Of course,
    /// this assumes that `file_dir` is `/home/user/.dotbak/dotfiles`.
    ///
    /// Returns either an error or `Ok(())`.
    pub fn move_and_symlink<P>(&self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Move the file from `home_dir` to `file_dir`.
        move_file(&file, &self.home_dir, &self.file_dir)?;

        // Now symlink it back to `home_dir`. Note that `file` is the relative path to the file/folder in `home_dir`.
        // Get the full path to the file/folder in `file_dir`.
        let file_dir_path = self.file_dir.join(&file);

        // Get the full path to the file/folder in `home_dir`.
        let home_dir_path = self.home_dir.join(file);

        // Create the symlink.
        unix_fs::symlink(&file_dir_path, &home_dir_path).context(SymlinkSnafu {
            from: file_dir_path,
            to: home_dir_path,
        })?;

        Ok(())
    }

    /// Basically undoes `move_and_symlink`. This will move the files/folders from `file_dir` to `home_dir` and
    /// delete the symlinks in `home_dir`.
    ///
    /// `files` are the paths to the file in `file_dir`. These paths must be relative to `file_dir`.
    ///
    /// Returns either an error or `Ok(())`.
    pub fn remove_and_restore<P>(&self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // First, we delete  the symlink in `home_dir`.
        let file = file.as_ref();

        // Get the full path to the file/folder in `home_dir`.
        let home_dir_path = self.home_dir.join(file);

        // Delete the symlink.
        fs::remove_file(&home_dir_path).context(DeleteSnafu {
            path: home_dir_path,
        })?;

        move_file(file, &self.file_dir, &self.home_dir)?;

        Ok(())
    }
}

/// Walks a directory relative to `home_dir`, getting all the files/folders that match the glob patterns in `include`
/// and not in`exclude`. These return relative (to `home_dir`) paths to the files/folders. If a file is entered, it will
/// be returned as is if not excluded. If a folder is entered, then all the files/folders in that folder not excluded
/// will be returned.
fn walk_dir<P>(dir: P, parent_dir: P, config: &FilesConfig) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let parent_dir = parent_dir.as_ref();
    let (include, exclude) = get_globsets(parent_dir, config)?;

    // If the path is a file, then we just return it if it matches the include glob and not the exclude glob.
    if parent_dir.join(&dir).is_file() {
        if include.is_match(&dir) && !exclude.is_match(&dir) {
            return Ok(vec![dir.as_ref().to_owned()]);
        } else {
            return Ok(Vec::new());
        }
    }

    let mut matched_files = Vec::new();

    for entry in WalkDir::new(&dir) {
        let entry = entry.unwrap(); // TODO: Get rid of unwraps.
        let entry_path = entry.path().strip_prefix(&dir).unwrap(); // TODO: Get rid of unwraps.

        // Skip any directories, as we're only interested in the files.
        if entry.file_type().is_dir() {
            continue;
        }

        // If it matches the include glob and not the exclude glob, then add it to the list of matched files.
        if include.is_match(entry_path) && !exclude.is_match(entry_path) {
            matched_files.push(entry_path.to_owned());
        }
    }

    Ok(matched_files)
}
/// Turns the `include` and `exclude` glob patterns into respective `GlobSet`s as (include, exclude). If a folder is
/// specified in `include` or `exclude`, then all the files/folders in that folder will be included/excluded. This is
/// done by simply appending `/*/**` to the glob pattern.
fn get_globsets<P>(parent_dir: P, config: &FilesConfig) -> Result<(GlobSet, GlobSet)>
where
    P: AsRef<Path>,
{
    let parent_dir = parent_dir.as_ref();
    let mut include_globset = GlobSetBuilder::new();
    let mut exclude_globset = GlobSetBuilder::new();

    for pattern in &config.include {
        if parent_dir.join(pattern).is_dir() {
            // HACK: This is a hack to get around the fact that glob patterns don't implicitly match a folder's child
            // files/folders. So if we want to include a folder, we have to explicitly include all the files/folders in
            // that folder.
            include_globset
                .add(Glob::new(&pattern.join("**/*").to_string_lossy()).context(GlobSnafu)?);
        } else {
            include_globset.add(Glob::new(&pattern.to_string_lossy()).context(GlobSnafu)?);
        }
    }

    for pattern in &config.exclude {
        if parent_dir.join(pattern).is_dir() {
            // HACK: This is a hack to get around the fact that glob patterns don't implicitly match a folder's child
            // files/folders. So if we want to exclude a folder, we have to explicitly exclude all the files/folders in
            // that folder.
            exclude_globset
                .add(Glob::new(&pattern.join("**/*").to_string_lossy()).context(GlobSnafu)?);
        } else {
            exclude_globset.add(Glob::new(&pattern.to_string_lossy()).context(GlobSnafu)?);
        }
    }

    Ok((
        include_globset.build().context(GlobSnafu)?,
        exclude_globset.build().context(GlobSnafu)?,
    ))
}

/// Helper function to move files from `from` to `to`.
///
/// `file` contains the file with a path relative to `from`.
///
/// `from` and `to` are the full paths to the directories.
///
/// Returns either an error or `Ok(())`.
fn move_file<P1, P2, P3>(file: P1, from: P2, to: P3) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    // Append all the paths to `from` to get the full path to the file/folder.
    let from_path = from.as_ref().join(file);

    let to_path = to.as_ref().join(from_path.file_name().unwrap()); // TODO: Get rid of unwraps.

    fs::rename(&from_path, &to_path).context(MoveSnafu {
        from: from_path,
        to: to_path,
    })?;

    Ok(())
}
