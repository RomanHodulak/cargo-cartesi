mod build;
mod new;
mod run;

pub use build::*;
pub use new::*;
pub use run::*;
use std::process::ExitCode;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    New(NewCommand),
}

impl Commands {
    pub fn run(&mut self) -> ExitCode {
        match self {
            Commands::New(cmd) => NewCommand::handle(cmd.target_dir.as_ref().expect("hovno")).expect("unhandled"),
        }
    }
}
