use crate::cli::{BuildCommand, CreateFsCommand, CreateMachineCommand, NewCommand};
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum Commands {
    New(NewCommand),
    Build(BuildCommand),
    CreateFs(CreateFsCommand),
    CreateMachine(CreateMachineCommand),
}

impl Commands {
    pub fn execute(self) -> ExitCode {
        match self {
            Commands::New(cmd) => cmd.handle().expect("failed new"),
            Commands::Build(cmd) => cmd.handle().expect("failed build"),
            Commands::CreateFs(cmd) => cmd.handle().expect("failed build"),
            Commands::CreateMachine(cmd) => cmd.handle().expect("failed build"),
        }

        ExitCode::SUCCESS
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn run(self) -> ExitCode {
        self.command.expect("no cmd").execute()
    }
}
