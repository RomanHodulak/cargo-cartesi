use clap::Args;
use std::iter;
use thiserror::Error;
use crate::commands;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug, Args)]
pub struct RunCommand {
    #[clap(value_parser)]
    target_bin: String,
    #[clap(value_parser)]
    output_fs: String,
}

impl RunCommand {
    pub fn handle(self) -> Result<(), RunCommandError> {
        commands::BuildCommand::handle().unwrap();
        commands::CreateFsCommand::handle(
            iter::once(self.target_bin.as_str()),
            None,
            self.output_fs
        ).unwrap();
        commands::RunCommand::handle(
            &self.target_bin
        ).unwrap();

        Ok(())
    }
}
