use itertools::Itertools;
use std::{cell::RefCell, fmt::Display, io::Write, process::Output};

/// The padding used before logs (normally hidden). Normally PAD + "> "
const LOG_PAD: &str = "   > ";

/// Logger for the dotbak crate.
pub struct Logger {
    /// Whether to be verbose with logging or not.
    /// Ex: printing the output of git commands.
    verbose: bool,

    /// The stdout to use for logging.
    stdout: RefCell<Box<dyn Write>>,

    /// The stderr to use for logging.
    stderr: RefCell<Box<dyn Write>>,
}

impl Logger {
    /// Creates a new logger.
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            stdout: RefCell::new(Box::new(std::io::stdout())),
            stderr: RefCell::new(Box::new(std::io::stderr())),
        }
    }

    // Creates a new logger with custom stdout and stderr.
    pub fn new_with_streams(verbose: bool, stdout: Box<dyn Write>, stderr: Box<dyn Write>) -> Self {
        Self {
            verbose,
            stdout: RefCell::new(stdout),
            stderr: RefCell::new(stderr),
        }
    }

    /// Log an output at INFO level.
    pub fn info<S>(&self, message: S)
    where
        S: Display,
    {
        if !self.verbose {
            return;
        }

        writeln!(
            self.stdout.borrow_mut(),
            "{}",
            console::style(pad_lines_from_start(message, LOG_PAD)).dim()
        )
        .unwrap(); // TODO: Handle error
    }

    /// Log an error.
    pub fn error<S>(&self, message: S)
    where
        S: Display,
    {
        if !self.verbose {
            return;
        }

        writeln!(
            self.stderr.borrow_mut(),
            "{}",
            console::style(pad_lines_from_start(message, LOG_PAD))
                .red()
                .dim()
        )
        .unwrap(); // TODO: Handle error
    }

    /// Log an output from a command.
    pub fn log_output(&self, output: Output) {
        if !self.verbose {
            return;
        }

        let stdout_untrimmed = String::from_utf8_lossy(&output.stdout);
        let stderr_untrimmed = String::from_utf8_lossy(&output.stderr);

        let stdout = stdout_untrimmed.trim();
        let stderr = stderr_untrimmed.trim();

        if !stdout.is_empty() {
            self.info(stdout);
        }

        if !stdout.is_empty() && !stderr.is_empty() {
            println!();
        }

        if !stderr.is_empty() {
            self.error(stderr);
        }
    }

    // Log multiple outputs.
    pub fn log_outputs<const N: usize>(&self, outputs: [Output; N]) {
        if !self.verbose {
            return;
        }

        for output in outputs {
            self.log_output(output);
        }
    }
}

// Pad all lines in a string from the start with a given string.
fn pad_lines_from_start<S1, S2>(input: S1, pad: S2) -> String
where
    S1: ToString,
    S2: Display,
{
    input
        .to_string()
        .lines()
        .map(|line| format!("{}{}", pad, line))
        .join("\n")
}
