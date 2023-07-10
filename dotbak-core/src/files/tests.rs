#![cfg(test)]

use super::*;
use assert_fs::prelude::*;

/// Test if we can move items from `home_dir` to `file_dir`.
#[test]
fn test_move_and_symlink() {
    let temp: assert_fs::TempDir = assert_fs::TempDir::new().unwrap();
    let home_dir = temp.child("home");
    let file_dir = temp.child("files");
    let file_manager = Files::init(home_dir.path().to_owned(), file_dir.path().to_owned());

    // Create the home directory.
    home_dir.create_dir_all().unwrap();

    // Create the file directory.
    file_dir.create_dir_all().unwrap();

    // Create the file.
    let original_file = home_dir.child("foo");

    // Create the expected files structure.
    let moved_file = file_dir.child("foo");

    // Actually create the files.
    original_file.touch().unwrap();

    // Check if the files exist in the correct place.
    assert!(!moved_file.exists());
    assert!(original_file.exists());

    // Now get the relative paths to the files.
    let relative_path = original_file.path().strip_prefix(home_dir.path()).unwrap();

    // Move the files.
    file_manager.move_and_symlink(relative_path).unwrap();

    // Check if the files exist in the correct place.
    assert!(moved_file.exists());
    // This is a symlink, so instead of checking if it exists, check if it's a symlink.
    assert!(original_file.read_link().is_ok());
}

/// Test the undoing of `move_and_symlink`.
#[test]
fn test_remove_and_restore() {
    let temp: assert_fs::TempDir = assert_fs::TempDir::new().unwrap();
    let home_dir = temp.child("home");
    let file_dir = temp.child("files");
    let file_manager = Files::init(home_dir.path().to_owned(), file_dir.path().to_owned());

    // Create the home directory.
    home_dir.create_dir_all().unwrap();

    // Create the file directory.
    file_dir.create_dir_all().unwrap();

    // Create the file.
    let original_file = home_dir.child("foo");

    // Create the expected files structure.
    let moved_file = file_dir.child("foo");

    // Actually create the files.
    original_file.touch().unwrap();

    // Check if the files exist in the correct place.
    assert!(!moved_file.exists());
    assert!(original_file.exists());

    // Now get the relative paths to the files.
    let relative_path = original_file.path().strip_prefix(home_dir.path()).unwrap();

    // Move the files.
    file_manager.move_and_symlink(relative_path).unwrap();

    // Check if the files exist in the correct place.
    assert!(moved_file.exists());
    // This is a symlink, so instead of checking if it exists, check if it's a symlink.
    assert!(original_file.read_link().is_ok());

    // Now undo the operation.
    file_manager.remove_and_restore(relative_path).unwrap();

    // Check if the files exist in the correct place.
    assert!(!moved_file.exists());
    assert!(original_file.exists());
}

/// Test that we can correctly run the `walk_dir` function on the `home_dir`.
#[test]
fn test_walk_dir() {
    let temp: assert_fs::TempDir = assert_fs::TempDir::new().unwrap();
    let home_dir = temp.child("home");
    let file_dir = temp.child("files");
    let file_manager = Files::init(home_dir.path().to_owned(), file_dir.path().to_owned());
    let file_config = FilesConfig {
        include: vec![
            PathBuf::from("foo"),
            PathBuf::from("qux"),
            PathBuf::from("spam/**/*"),
        ],
        exclude: vec![
            PathBuf::from("bar"),
            PathBuf::from("qux/baz/foo"),
            PathBuf::from("spam/baz/**/*"),
        ],
    };

    // Create the home directory.
    home_dir.create_dir_all().unwrap();

    // Create the files.
    let files = [
        home_dir.child("foo"),
        home_dir.child("bar"),
        home_dir.child("baz"),
        home_dir.child("qux/foo"),
        home_dir.child("qux/bar"),
        home_dir.child("qux/baz/foo"),
        home_dir.child("qux/baz/bar"),
        home_dir.child("spam/foo"),
        home_dir.child("spam/bar"),
        home_dir.child("spam/baz/foo"),
        home_dir.child("spam/baz/bar"),
    ];

    // Create the expected files structure.
    let expected_files = vec![
        PathBuf::from("foo"),
        PathBuf::from("qux/foo"),
        PathBuf::from("qux/bar"),
        PathBuf::from("qux/baz/bar"),
        PathBuf::from("spam/foo"),
        PathBuf::from("spam/bar"),
    ];

    // Actually create the files.
    for file in &files {
        file.touch().unwrap();
    }

    // Get the files.
    let files = walk_dir(".", home_dir, &file_config).unwrap();

    // Check if the files are correct.
    assert!(files.iter().all(|file| { expected_files.contains(file) }));
}

/// Test that we can correctly run the `walk_dir` function inside a dir in `home_dir`.
#[test]
fn test_walk_dir_subdir() {
    let temp: assert_fs::TempDir = assert_fs::TempDir::new().unwrap();
    let home_dir = temp.child("home");
    let file_dir = temp.child("files");
    let file_manager = Files::init(home_dir.path().to_owned(), file_dir.path().to_owned());
    let file_config = FilesConfig {
        include: vec![PathBuf::from("qux")],
        exclude: vec![PathBuf::from("qux/foo"), PathBuf::from("qux/bar/foo")],
    };

    // Create the home directory.
    home_dir.create_dir_all().unwrap();

    // Create the files.
    let files = [
        home_dir.child("qux/foo"),
        home_dir.child("qux/bar/foo"),
        home_dir.child("qux/bar/bar"),
        home_dir.child("qux/bar/baz/foo"),
    ];

    // Create the expected files structure.
    let expected_files = vec![
        PathBuf::from("qux/bar/bar"),
        PathBuf::from("qux/bar/baz/foo"),
    ];

    // Actually create the files.
    for file in &files {
        file.touch().unwrap();
    }

    // Get the files.
    let files = file_manager
        .walk_dir(home_dir.path().join("qux"), &file_config)
        .unwrap();

    // Check if the files are correct.
    assert!(files.iter().all(|file| { expected_files.contains(file) }));
}

/// Test that we can correctly run the `walk_dir` function on a file path, not a folder path.
#[test]
fn test_walk_dir_file() {
    let temp: assert_fs::TempDir = assert_fs::TempDir::new().unwrap();
    let home_dir = temp.child("home");
    let file_dir = temp.child("files");
    let file_manager = Files::init(home_dir.path().to_owned(), file_dir.path().to_owned());
    let file_config = FilesConfig {
        include: vec![PathBuf::from("foo")],
        exclude: vec![PathBuf::from("bar")],
    };

    // Create the home directory.
    home_dir.create_dir_all().unwrap();

    // Create the files.
    let files = [home_dir.child("foo"), home_dir.child("bar")];

    // Create the expected files structure.
    let expected_files = vec![PathBuf::from("foo")];

    // Actually create the files.
    for file in &files {
        file.touch().unwrap();
    }

    // Get the files.
    let files = file_manager.walk_dir("foo", &file_config).unwrap();

    // Check if the files are correct.
    assert!(files.iter().all(|file| { expected_files.contains(file) }));
}
