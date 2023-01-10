/// `Logger` logs information out to the console, in different formats,
/// for different types of info (DEBUG vs INFO vs WARN vs ERROR etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Logger {}

/// `LogLevel` is an enum of all different `Logger` priority levels. These
/// are used to decide what messages get displayed
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
