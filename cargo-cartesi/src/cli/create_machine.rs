use crate::commands;
use clap::Args;
use std::iter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateMachineCommandError {}

#[derive(Debug, Args)]
pub struct CreateMachineCommand {
    #[clap(value_parser)]
    target_bin: String,
    #[clap(value_parser)]
    output_fs: String,
}

impl CreateMachineCommand {
    pub fn handle(self) -> Result<(), CreateMachineCommandError> {
        commands::BuildCommand::handle().unwrap();
        commands::CreateFsCommand::handle(
            iter::once(self.target_bin.as_str()),
            None,
            self.output_fs
        ).unwrap();
        commands::CreateMachineCommand::handle(
            &self.target_bin
        ).unwrap();

        Ok(())
    }
}
