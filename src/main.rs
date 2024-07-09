mod cli;
mod config;
mod dotbak;
mod errors;
mod files;
mod git;
mod test_util;
mod ui;
use clap::Parser;
use cli::Cli;
use miette::Result;

fn main() -> Result<()> {
    amend_panic_with_issue_msg();

    let cli = Cli::parse();

    cli.run()?;

    Ok(())
}

/// OVerride panic messages with a message to submit an issue at the git repo.
fn amend_panic_with_issue_msg() {
    let default_panic = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);

        println!();

        println!("{}", console::style("This panic most likely should not have happened (unless your OS is very weird). However, Dotbak is experimental and these types of things can happen.").yellow());
        println!("{}", console::style("If you feel that this panic was unjustified or unreasonable, submit an issue at https://github.com/cogsandsquigs/dotbak if you encounter any problems.").yellow());
        println!("{}", console::style("If you aren't sure what to do, submit an issue just in case. Better safe than sorry ;).").yellow());
    }));
}
