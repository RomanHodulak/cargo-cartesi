mod cli;
mod commands;
pub(crate) mod services;

use clap::Parser;
use cli::Cli;
use std::process::ExitCode;

fn main() -> ExitCode {
    Cli::parse().run()
}
