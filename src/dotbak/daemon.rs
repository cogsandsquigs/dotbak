use super::Dotbak;
use crate::errors::io::IoError;
use crate::errors::Result;
use daemonize::Daemonize;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

const PID_FILE: &str = "/tmp/dotbak-daemon.pid";

pub struct Daemon<'a> {
    /// The dotbak instance.
    pub dotbak: Dotbak,

    /// The daemonize instance created by the daemon.
    pub daemonize: Daemonize<&'a str>,
}

impl Daemon<'_> {
    /// Crate a new daemon instance.
    pub fn new<'a>() -> Result<Daemon<'a>> {
        let stdout = File::create("/tmp/dotbak-daemon.out").unwrap();
        let stderr = File::create("/tmp/dotbak-daemon.err").unwrap();

        let dotbak =
            Dotbak::load_for_daemon(stdout.try_clone().unwrap(), stderr.try_clone().unwrap())?;

        let daemonize = Daemonize::new()
            .pid_file("/tmp/dotbak-daemon.pid") // Every method except `new` and `start`
            .chown_pid_file(true) // is optional, see `Daemonize` documentation
            .working_directory("/tmp") // for default behaviour.
            .umask(0o777) // Set umask, `0o027` by default.
            .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
            .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
            .privileged_action(|| "");

        Ok(Daemon { dotbak, daemonize })
    }

    /// Run dotbak daemon wrapper.
    /// TODO: Signal handling, so that the process stops gracefully.
    pub fn run(mut self) {
        self.dotbak.logger.info("Running dotbak daemon...");

        self.daemonize.start().unwrap();

        let delay_between_sync = Duration::from_secs(self.dotbak.config.delay_between_sync);

        // Run forever, until the user stops the daemon OR it panics OR the computer shuts down.
        loop {
            // Run the sync command
            self.dotbak
                .sync()
                .expect("This should not error out when running on the daemon!");

            thread::sleep(delay_between_sync);
        }
    }

    /// Stops the daemon.
    pub fn stop() -> Result<()> {
        // Get the PID
        let pid = std::fs::read_to_string(PID_FILE).map_err(|err| IoError::Read {
            source: err,
            path: PathBuf::from_str(PID_FILE).expect("The PID_FILE path should always exist!"),
        })?;

        let pid = pid.trim();

        // Run the kill command
        let output = Command::new("kill")
            .arg(pid)
            .output()
            .map_err(|err| IoError::CommandIO {
                command: "kill".to_string(),
                args: vec![format!("{}", pid)],
                source: err,
            })?;

        // If the output isn't a success, then return an error.
        if !output.status.success() {
            return Err(IoError::CommandRun {
                command: "kill".to_string(),
                args: vec![format!("{}", pid)],
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            }
            .into());
        }

        // Delete the PID file
        std::fs::remove_file(PID_FILE).map_err(|err| IoError::Delete {
            path: PathBuf::from_str(PID_FILE).expect("The PID_FILE path should always exist!"),
            source: err,
        })?;

        Ok(())
    }
}
