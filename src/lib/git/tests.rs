#![cfg(test)]

use std::path::Path;

use crate::{
    errors::{git::GitError, io::IoError, DotbakError},
    Dotbak,
};
use assert_fs::TempDir;

const TEST_GIT_REPO_URL: &str = "https://github.com/githubtraining/hellogitworld";

/// Helper function to check if a repository exists at a path.
fn repo_exists<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    path.exists() && path.join(".git").exists()
}

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
    assert!(repo_exists(repo_dir));
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
    assert!(repo_exists(&repo_dir));
}

/// Test if we fail when initing a repo in a repository that already exists.
#[test]
fn test_init_repo_path_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::init_repo(repo_dir).unwrap();

    // Check if the repository exists.
    assert!(repo_exists(repo_dir));

    // Try to initialize the repository again.
    let result = Dotbak::init_repo(repo_dir);

    // Check if the result is an error.
    assert!(result.is_err());

    // Check that it is a git init error.
    assert!(matches!(
        result,
        Err(DotbakError::Git {
            source: GitError::Init { .. }
        })
    ));
}

/// Test if we can clone a remote repository into a given path.
#[test]
fn test_clone_repo_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::clone_repo(repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the repository exists.
    assert!(repo_exists(repo_dir));
}

/// Test if we can clone a remote repository into a given path that doesn't exist.
#[test]
fn test_clone_repo_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let _repo = Dotbak::clone_repo(&repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    assert!(repo_exists(&repo_dir));
}

/// Test if we fail when cloning a repo into a repository that already exists.
#[test]
fn test_clone_repo_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::init_repo(repo_dir);

    // Check if the repository exists.
    assert!(repo_exists(repo_dir));

    // Try to clone the repository again.
    // THIS SHOULD PANIC
    let result = Dotbak::clone_repo(repo_dir, TEST_GIT_REPO_URL);

    // Check if the result is an error.
    assert!(result.is_err());

    // Check that it is a git clone error.
    assert!(matches!(
        result,
        Err(DotbakError::Git {
            source: GitError::Clone { .. }
        })
    ));
}

/// Test the deletion of a repository.
#[test]
fn test_delete_repo() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::init_repo(repo_dir).unwrap();

    // Check if the repository exists.
    assert!(repo_exists(repo_dir));

    // Delete the repository.
    Dotbak::delete_repo(repo_dir).unwrap();

    // Check if the repository exists.
    assert!(!repo_exists(repo_dir));
}

/// Test the deletion of a repository that doesn't exist.
#[test]
fn test_delete_repo_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Delete the repository.
    let result = Dotbak::delete_repo(repo_dir);

    // Check if the result is an error.
    assert!(result.is_err());

    let err = result.unwrap_err();

    println!("{}", err);

    // Check that it is an IO error.
    assert!(matches!(
        err,
        DotbakError::Io {
            source: IoError::Delete { .. }
        }
    ));
}
