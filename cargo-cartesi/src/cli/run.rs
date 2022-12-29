use crate::commands;
use crate::services::HostCargo;
use clap::Args;
use std::iter;
use std::path::PathBuf;
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
        commands::BuildCommand::handle().unwrap();

        let target_bin = self.target_bin.unwrap_or(HostCargo::package_name().unwrap());
        let target_dir = PathBuf::from(HostCargo::target_dir().unwrap()).join(&target_bin);
        commands::CreateFsCommand::handle(iter::once(target_dir), None, &self.output_fs).unwrap();

        commands::RunCommand::handle(target_bin, self.output_fs).unwrap();

        Ok(())
    }
}
