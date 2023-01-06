use crate::commands;
use crate::services::{Cargo, FileSystem};
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
    pub fn handle(self, cargo: impl Cargo, file_system: impl FileSystem) -> Result<(), CreateFsCommandError> {
        commands::CreateFsCommand::handle(self.target_bin, self.output_fs, &cargo, &file_system).unwrap();

        Ok(())
    }
}
