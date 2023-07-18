use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_FRAME_DURATION: Duration = Duration::from_millis(100);

/// A wrapper around a progress bar.
pub struct Spinner {
    spinner: ProgressBar,
}

impl Spinner {
    /// Creates a new spinner.
    pub fn new(message: String) -> Spinner {
        let spinner = ProgressBar::new_spinner()
            .with_message(message)
            // Default spinner style
            .with_style(
                ProgressStyle::default_spinner()
                    .template(&template_with_time("{spinner:.cyan/blue} {msg}"))
                    .expect("This should not fail!")
                    .tick_strings(SPINNER_FRAMES),
            );

        spinner.enable_steady_tick(SPINNER_FRAME_DURATION);

        Spinner { spinner }
    }

    /// Prints a message above the spinner.
    pub fn print(&self, message: String) {
        self.spinner.set_message(message);
    }

    /// Closes the spinner.
    pub fn close(self) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template(&template_with_time("✨ {msg}"))
                .expect("This should not fail!")
                .tick_strings(SPINNER_FRAMES),
        );

        self.spinner.tick();

        self.spinner.finish_with_message("Done!")
    }
}

/// Creates a template with an attached time, bolded and dimmed.
fn template_with_time(template: &str) -> String {
    format!(
        "{} {}{{elapsed:.bold.dim}}{}",
        template,
        console::style("[").bold().dim(),
        console::style("]").bold().dim()
    )
}
