#![cfg(test)]

use crate::Dotbak;
use assert_fs::TempDir;

/// Test if we can create a new repository at a given path.
#[test]
fn test_init_repo_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::init_repo(repo_dir).unwrap();

    println!("{:?}", repo_dir);

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());
}

/// Test if we can create a new repository at a given path that doesn't exist.
#[test]
fn test_init_repo_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let _repo = Dotbak::init_repo(&repo_dir).unwrap();

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());
}
