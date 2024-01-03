use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_FRAME_DURATION: Duration = Duration::from_millis(100);

/// A wrapper around a progress bar.
#[derive(Clone, Debug)]
pub struct Spinner {
    spinner: ProgressBar,
    padding: usize,
    current: usize,
    total: usize,
}

impl Spinner {
    /// Creates a new spinner.
    pub fn new(message: String, padding: usize, current: usize, total: usize) -> Spinner {
        Spinner {
            spinner: ProgressBar::new_spinner()
                .with_message(message)
                // Default spinner style
                .with_style(
                    ProgressStyle::default_spinner()
                        .template(&template_with_ending(
                            "{spinner:.cyan/blue}",
                            padding,
                            current,
                            total,
                        ))
                        .expect("This should not fail!")
                        .tick_strings(SPINNER_FRAMES),
                ),
            padding,
            current,
            total,
        }
    }

    /// Starts the spinner.
    pub fn start(&mut self) {
        self.spinner.enable_steady_tick(SPINNER_FRAME_DURATION);
    }

    /// Prints a message above the spinner.
    pub fn print(&self, message: String) {
        self.spinner.set_message(message);
    }

    /// Closes the spinner.
    pub fn close(self) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template(&template_with_ending(
                    "✅",
                    self.padding,
                    self.current,
                    self.total,
                ))
                .expect("This should not fail!")
                .tick_strings(SPINNER_FRAMES),
        );

        self.spinner.tick();

        self.spinner.finish();
    }
}

/// Creates a template with an attached spinner count and padding, with a custom ending.
fn template_with_ending(ending: &str, padding: usize, current: usize, total: usize) -> String {
    format!(
        "   {} {{msg}}...{} {}",
        console::style(format!("[{}/{}]", current, total))
            .bold()
            .dim(),
        ".".repeat(padding),
        ending,
    )
}
