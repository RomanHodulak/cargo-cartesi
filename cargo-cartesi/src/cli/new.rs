use crate::commands;
use crate::services::{HostCargo, HostDependencyDownloader, HostResourceCreator};
use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewCommandError {}

#[derive(Debug, Args)]
pub struct NewCommand {
    #[clap(value_parser)]
    target_dir: String,
}

impl NewCommand {
    pub fn handle(self) -> Result<(), NewCommandError> {
        let cargo = HostCargo;
        let deps = HostDependencyDownloader;
        let res = HostResourceCreator;

        commands::NewCommand::handle(self.target_dir, &deps, &res, &cargo).unwrap();

        Ok(())
    }
}
