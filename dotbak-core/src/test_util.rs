// /// Helper function to check if a repository exists at a path.
// pub fn repo_exists<P>(path: P) -> bool
// where
//     P: AsRef<Path>,
// {
//     let path = path.as_ref();

//     path.exists() && path.join(".git").exists()
// }

#[macro_export]
macro_rules! repo_exists {
    ($path:expr) => {
        assert!($path.exists());
        assert!($path.join(".git").exists());
    };
}

#[macro_export]
macro_rules! repo_not_exists {
    ($path:expr) => {
        assert!(!$path.exists());
        assert!(!$path.join(".git").exists());
    };
}
