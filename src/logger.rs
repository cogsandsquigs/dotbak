use std::{fmt::Display, io};

use console::{style, Term};

/// `Logger` logs information out to the console, in different formats,
/// for different types of info (DEBUG vs INFO vs WARN vs ERROR etc.).
#[derive(Clone, Debug)]
pub struct Logger {
    /// The level at which we log events.
    level: LogLevel,

    /// The terminal stdout
    stdout: Term,

    /// The terminal stderr
    stderr: Term,
}

/// Public API for `Logger`.
impl Logger {
    /// Creates a new `Logger` instance. Note that any logging levels equal or greater to
    /// `level` will be logged, and those less will not.
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            stdout: Term::stdout(),
            stderr: Term::stderr(),
        }
    }

    /// Logging a message at the DEBUG level.
    pub fn debug<S: ToString + Display>(&self, message: S) -> io::Result<()> {
        let message = style(format!("[DEBUG] {}", message)).blue().to_string();

        self.log(message, LogLevel::Debug)
    }

    /// Logging a message at the INFO level.
    pub fn info<S: ToString + Display>(&self, message: S) -> io::Result<()> {
        let message = format!("[INFO] {}", message);

        self.log(message, LogLevel::Info)
    }

    /// Logging a message at the WARN level.
    pub fn warn<S: ToString + Display>(&self, message: S) -> io::Result<()> {
        let message = style(format!("[WARN] {}", message)).yellow().to_string();

        self.log(message, LogLevel::Warn)
    }

    /// Logging a message at the WARN level.
    pub fn error<S: ToString + Display>(&self, message: S) -> io::Result<()> {
        let message = style(format!("[ERROR] {}", message)).red().to_string();

        self.log(message, LogLevel::Error)
    }
}

/// Private API for `Logger`.
impl Logger {
    /// Log a message to the output.
    fn log(&self, message: String, level: LogLevel) -> io::Result<()> {
        // Don't log output that is lower than our logging level.
        if self.level > level {
            return Ok(());
        }

        // Checking if this should output to stderr
        if level >= LogLevel::Warn {
            let result = self.stderr.write_line(message.as_str());

            // This is here because if we encounter an error, we want to log it immediately.
            if result.is_err() && level >= LogLevel::Fatal {
                eprintln!(
                    "[FATAL] {}",
                    result
                        .as_ref()
                        .err()
                        .expect("This should be safe because we know the result is an error.")
                );
            }

            return result;
        }

        self.stdout.write_line(message.as_str())?;

        Ok(())
    }
}

/// `LogLevel` is an enum of all different `Logger` priority levels. These
/// are used to decide what messages get displayed
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
