use crate::commands;
use crate::services::{Cargo, CartesiMachine, DependenciesDownloader, FileSystem};
use clap::Args;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateMachineCommandError {}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct CreateMachineCommand {
    #[clap(value_parser)]
    target_bin: Option<String>,
    #[clap(value_parser, default_value = "dapp.ext2")]
    output_fs: String,
}

impl CreateMachineCommand {
    pub fn handle(
        self,
        cargo: impl Cargo,
        file_system: impl FileSystem,
        deps: impl DependenciesDownloader,
        cartesi_machine: impl CartesiMachine,
    ) -> Result<(), CreateMachineCommandError> {
        commands::CreateMachineCommand::handle(
            self.target_bin,
            self.output_fs,
            &deps,
            &cargo,
            &file_system,
            &cartesi_machine,
        )
        .unwrap();

        Ok(())
    }
}
