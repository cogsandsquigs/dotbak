#![cfg(test)]

use super::Files;
use assert_fs::prelude::*;
use itertools::Itertools;

/// Test if we can move items from `home_dir` to `file_dir`.
#[test]
fn test_move_and_symlink() {
    let temp: assert_fs::TempDir = assert_fs::TempDir::new().unwrap();
    let home_dir = temp.child("home");
    let file_dir = temp.child("files");
    let file_manager = Files::init(
        home_dir.path().to_owned(),
        file_dir.path().to_owned(),
        Default::default(),
    );

    // Create the home directory.
    home_dir.create_dir_all().unwrap();

    // Create the file directory.
    file_dir.create_dir_all().unwrap();

    // Create the files.
    let original_files = [
        home_dir.child("foo"),
        home_dir.child("bar"),
        home_dir.child("baz"),
    ];

    // Create the expected files structure.
    let moved_files = [
        file_dir.child("foo"),
        file_dir.child("bar"),
        file_dir.child("baz"),
    ];

    // Actually create the files.
    for file in &original_files {
        file.touch().unwrap();
    }

    // Check if the files exist in the correct place.
    for file in &moved_files {
        assert!(!file.exists());
    }

    // Check if the files exist in the correct place.
    for file in &original_files {
        assert!(file.exists());
    }

    // Now get the relative paths to the files.
    let relative_paths = original_files
        .iter()
        .map(|file| file.path().strip_prefix(home_dir.path()).unwrap())
        .collect_vec();

    // Move the files.
    file_manager.move_and_symlink(&relative_paths).unwrap();

    // Check if the files exist in the correct place.
    for file in &moved_files {
        assert!(file.exists());
    }

    // Check if the files exist in the correct place.
    for file in &original_files {
        // This is a symlink, so instead of checking if it exists, check if it's a symlink.
        assert!(file.read_link().is_ok());
    }
}

/// Test the undoing of `move_and_symlink`.
#[test]
fn test_remove_and_restore() {
    let temp: assert_fs::TempDir = assert_fs::TempDir::new().unwrap();
    let home_dir = temp.child("home");
    let file_dir = temp.child("files");
    let file_manager = Files::init(
        home_dir.path().to_owned(),
        file_dir.path().to_owned(),
        Default::default(),
    );

    // Create the home directory.
    home_dir.create_dir_all().unwrap();

    // Create the file directory.
    file_dir.create_dir_all().unwrap();

    // Create the files.
    let original_files = [
        home_dir.child("foo"),
        home_dir.child("bar"),
        home_dir.child("baz"),
    ];

    // Create the expected files in `file_dir`.
    let moved_files = [
        file_dir.child("foo"),
        file_dir.child("bar"),
        file_dir.child("baz"),
    ];

    // Actually create the files.
    for file in &original_files {
        file.touch().unwrap();
    }

    // Now get the relative paths to the files.
    let relative_paths = original_files
        .iter()
        .map(|file| file.path().strip_prefix(home_dir.path()).unwrap())
        .collect_vec();

    // Move the files.
    file_manager.move_and_symlink(&relative_paths).unwrap();

    // Check if the files exist in the correct place.
    for file in &moved_files {
        assert!(file.exists());
    }

    // Check if the files exist in the correct place.
    for file in &original_files {
        assert!(file.exists());
    }

    // Now undo the operation.
    file_manager.remove_and_restore(&relative_paths).unwrap();

    // Check if the files exist in the correct place.
    for file in &moved_files {
        assert!(!file.exists());
    }

    // Check if the files exist in the correct place.
    for file in &original_files {
        assert!(file.exists());
    }
}
