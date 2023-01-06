use crate::commands;
use crate::services::{Cargo, DependenciesDownloader, ResourceCreator};
use clap::Args;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewCommandError {}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct NewCommand {
    #[clap(value_parser)]
    target_dir: String,
}

impl NewCommand {
    pub fn handle(
        self,
        cargo: impl Cargo,
        deps: impl DependenciesDownloader,
        res: impl ResourceCreator,
    ) -> Result<(), NewCommandError> {
        commands::NewCommand::handle(self.target_dir, &deps, &res, &cargo).unwrap();

        Ok(())
    }
}
