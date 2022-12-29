use crate::commands;
use crate::services::{DockerCartesiMachine, HostCargo, HostFileSystem};
use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateMachineCommandError {}

#[derive(Debug, Args)]
pub struct CreateMachineCommand {
    #[clap(value_parser)]
    target_bin: Option<String>,
    #[clap(value_parser, default_value = "dapp.ext2")]
    output_fs: String,
}

impl CreateMachineCommand {
    pub fn handle(self) -> Result<(), CreateMachineCommandError> {
        let cargo = HostCargo;
        let file_system = HostFileSystem;
        let cartesi_machine = DockerCartesiMachine;

        commands::CreateMachineCommand::handle(self.target_bin, self.output_fs, &cargo, &file_system, &cartesi_machine)
            .unwrap();

        Ok(())
    }
}
