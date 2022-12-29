use crate::commands;
use crate::services::{DockerCartesiMachine, HostCargo, HostFileSystem};
use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug, Args)]
pub struct RunCommand {
    #[clap(value_parser)]
    target_bin: Option<String>,
    #[clap(value_parser, default_value = "dapp.ext2")]
    output_fs: String,
}

impl RunCommand {
    pub fn handle(self) -> Result<(), RunCommandError> {
        let cargo = HostCargo;
        let file_system = HostFileSystem;
        let cartesi_machine = DockerCartesiMachine;

        commands::RunCommand::handle(self.target_bin, self.output_fs, &cargo, &file_system, &cartesi_machine).unwrap();

        Ok(())
    }
}
