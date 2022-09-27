mod app;
mod commands;

use app::Cli;
use clap::Parser;
use std::error::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    Cli::parse().run()
}
