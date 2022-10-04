use crate::commands;
use clap::Args;
use std::iter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateFsCommandError {}

#[derive(Debug, Args)]
pub struct CreateFsCommand {
    #[clap(value_parser)]
    target_bin: String,
    #[clap(value_parser)]
    output_fs: String,
}

impl CreateFsCommand {
    pub fn handle(self) -> Result<(), CreateFsCommandError> {
        commands::BuildCommand::handle().unwrap();
        commands::CreateFsCommand::handle(iter::once(self.target_bin), None, self.output_fs).unwrap();

        Ok(())
    }
}
