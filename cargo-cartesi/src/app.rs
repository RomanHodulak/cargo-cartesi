use crate::commands::Commands;
use clap::{Parser, Subcommand};
use std::process::{ExitCode, ExitStatus};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn run(&mut self) -> ExitCode {
        self.command.as_mut().expect("no cmd").run()
    }
}
