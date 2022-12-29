use crate::commands;
use crate::services::HostCargo;
use clap::Args;
use std::iter;
use std::path::PathBuf;
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
        commands::BuildCommand::handle().unwrap();

        let target_bin = self.target_bin.unwrap_or(HostCargo::package_name().unwrap());
        let target_dir = PathBuf::from(HostCargo::target_dir().unwrap()).join(&target_bin);
        commands::CreateFsCommand::handle(iter::once(target_dir), None, self.output_fs).unwrap();

        Ok(())
    }
}
