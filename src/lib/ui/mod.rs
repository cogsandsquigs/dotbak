/// These define macros for the UI of the CLI.

#[macro_export]
pub macro info {
    ($($arg:tt)*) => {
        println!($($arg)*);
    }
}
