use crate::commands;
use crate::services::{HostCargo, HostFileSystem};
use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateFsCommandError {}

#[derive(Debug, Args)]
pub struct CreateFsCommand {
    #[clap(value_parser)]
    target_bin: Option<String>,
    #[clap(value_parser, default_value = "dapp.ext2")]
    output_fs: String,
}

impl CreateFsCommand {
    pub fn handle(self) -> Result<(), CreateFsCommandError> {
        let cargo = HostCargo;
        let file_system = HostFileSystem;

        commands::CreateFsCommand::handle(self.target_bin, self.output_fs, &cargo, &file_system).unwrap();

        Ok(())
    }
}
