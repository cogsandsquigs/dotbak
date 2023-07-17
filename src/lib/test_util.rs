#![cfg(test)]

/// Helper function to check if a repository exists at a path.
#[macro_export]
macro_rules! repo_exists {
    ($path:expr) => {
        assert!($path.exists());
        assert!($path.join(".git").exists());
    };
}

/// Helper function to check if a repository doesn't exist at a path.
#[macro_export]
macro_rules! repo_not_exists {
    ($path:expr) => {
        assert!(!$path.exists());
        assert!(!$path.join(".git").exists());
    };
}
