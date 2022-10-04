mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use std::process::ExitCode;

fn main() -> ExitCode {
    Cli::parse().run()
}
