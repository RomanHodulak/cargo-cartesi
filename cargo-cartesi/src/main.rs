mod app;
mod commands;

use std::error::Error;
use std::process::ExitCode;
use app::Cli;
use clap::Parser;

fn main() -> ExitCode {
    Cli::parse().run()
}
