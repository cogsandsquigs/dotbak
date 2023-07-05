#![cfg(test)]

use crate::{
    errors::{io::IoError, DotbakError},
    git::Repository,
    repo_exists, repo_not_exists,
};
use assert_fs::{prelude::*, TempDir};
use std::env;

// Check if we are in a CI environment.
fn in_ci() -> bool {
    match env::var("CI") {
        Ok(s) => s == "true",
        _ => false,
    }
}

/// The repository URL for the test repository.
const TEST_GIT_REPO_URL: &str = "https://github.com/cogsandsquigs/dotbak";

/// Test if we can create a new repository at a given path.
#[test]
fn test_init_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = Repository::init(repo_dir, None).unwrap();

    println!("{:?}", repo_dir);

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can create a new repository at a given path that doesn't exist.
#[test]
fn test_init_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let repo = Repository::init(&repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test that we don't fail when initing a repo in a repository that already exists.
#[test]
fn test_init_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = Repository::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Initialize the repository again.
    let repo = Repository::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can load a pre-existing repository.
#[test]
fn test_load_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = Repository::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);

    // Load the repository.
    let result = Repository::load(repo_dir);

    // Check if the result is ok.
    assert!(result.is_ok());

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can load a pre-existing repository that doesn't exist.
#[test]
fn test_load_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Load the repository.
    let result = Repository::load(repo_dir);

    // Check if the result is an error.
    assert!(result.is_err());

    // Check that it is an IO error.
    assert!(matches!(
        result,
        Err(DotbakError::Io {
            source: IoError::NotFound { .. }
        })
    ));
}

/// Test if we can clone a remote repository into a given path.
#[test]
fn test_clone_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = Repository::clone(repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can clone a remote repository into a given path that doesn't exist.
#[test]
fn test_clone_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let repo = Repository::clone(&repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we fail when cloning a repo into a repository that already exists.
#[test]
fn test_clone_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = Repository::init(repo_dir, None);

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.unwrap().path, repo_dir);

    // Try to clone the repository again.
    // THIS SHOULD PANIC
    let result = Repository::clone(repo_dir, TEST_GIT_REPO_URL);

    // Check if the result is an error.
    assert!(result.is_err());

    // Check that it is a git clone error.
    assert!(matches!(
        result,
        Err(DotbakError::Io {
            source: IoError::CommandRun { stderr, .. }
        }) if stderr.contains("already exists and is not an empty directory")
    ));
}

/// Test if we can run arbitrary commands in a repository.
/// This just does `git add .` and `git commit -m "Test commit"`.
#[test]
fn test_arbitrary_command() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let mut repo = Repository::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Create a file in the repository.
    tmp_dir.child("test.txt").touch().unwrap();

    // Run the arbitrary command.
    repo.arbitrary_command(&["add", "."]).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());

    // Run the arbitrary command.
    repo.arbitrary_command(&["commit", "-m", "Test commit"])
        .unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());
}

/// Test if we can commit changes to a repository.
#[test]
fn test_commit() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let mut repo = Repository::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Create the git config.
    repo.arbitrary_command(&["config", "user.name", "Test User"])
        .unwrap();
    repo.arbitrary_command(&["config", "user.email", "test_user@tests"])
        .unwrap();

    // Create a file in the repository.
    tmp_dir.child("test.txt").touch().unwrap();

    // Commit the changes.
    repo.commit("Initial commit").unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());

    // Create a new file in the repository.
    tmp_dir.child("test2.txt").touch().unwrap();

    // Commit the changes.
    repo.commit("Second commit").unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());
    assert!(tmp_dir.child("test2.txt").path().exists());
}

/// Test setting the remote of a repository.
#[test]
fn test_set_remote() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let mut repo_dir = tmp_dir.path().to_path_buf();

    // Initialize the repository.
    let mut repo = Repository::init(&repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Set the remote.
    repo.set_remote(TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Clone the repository.
    repo_dir = tmp_dir.path().join("clone");
    let mut repo = Repository::clone(&repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Set the remote.
    repo.set_remote(TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test pushing data to a remote repository.
#[test]
fn test_push() {
    // Check if we are in a CI environment. If we are, skip the test.
    // This is because the environment doesn't have the correct credentials.
    // If we are not in a CI environment, run the test.
    if !in_ci() {
        // Create a temporary directory.
        let tmp_dir = TempDir::new().unwrap();

        // Get the path to the repo directory.
        let repo_dir = tmp_dir.path();

        // Initialize the repository.
        let mut repo = Repository::clone(repo_dir, TEST_GIT_REPO_URL).unwrap();

        // Check if the repository exists.
        repo_exists!(repo_dir);
        assert_eq!(repo.path, repo_dir);

        // Push the changes.
        repo.push().unwrap();

        // Check if the repository exists.
        repo_exists!(repo_dir);
        assert_eq!(repo.path(), repo_dir);
    }
    // Otherwise, skip the test.
    else {
        println!("Skipping test_push because we are in a CI environment.");
    }
}

/// Test pulling data from a remote repository.
#[test]
fn test_pull() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let mut repo = Repository::clone(repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Pull the changes.
    repo.pull().unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test the deletion of a repository.
#[test]
fn test_delete() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = Repository::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Delete the repository.
    repo.delete().unwrap();

    // Check if the repository exists.
    repo_not_exists!(repo_dir);
}
