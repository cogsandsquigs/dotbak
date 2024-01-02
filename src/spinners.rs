use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_FRAME_DURATION: Duration = Duration::from_millis(100);

/// A wrapper around a progress bar.
pub struct Spinner {
    spinner: ProgressBar,
    current: usize,
    total: usize,
}

impl Spinner {
    /// Creates a new spinner.
    pub fn new(message: String, current: usize, total: usize) -> Spinner {
        let spinner = ProgressBar::new_spinner()
            .with_message(message)
            // Default spinner style
            .with_style(
                ProgressStyle::default_spinner()
                    .template(&template_with_count(
                        "{msg}... {spinner:.cyan/blue}",
                        current,
                        total,
                    ))
                    .expect("This should not fail!")
                    .tick_strings(SPINNER_FRAMES),
            );

        spinner.enable_steady_tick(SPINNER_FRAME_DURATION);

        Spinner {
            spinner,
            current,
            total,
        }
    }

    /// Prints a message above the spinner.
    pub fn print(&self, message: String) {
        self.spinner.set_message(message);
    }

    /// Closes the spinner.
    pub fn close(self) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template(&template_with_count(
                    "{msg}... ✅",
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

/// Creates a template with an attached spinner count.
fn template_with_count(template: &str, current: usize, total: usize) -> String {
    format!(
        "   {} {}",
        console::style(format!("[{}/{}]", current, total))
            .bold()
            .dim(),
        template,
    )
}
