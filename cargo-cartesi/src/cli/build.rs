use crate::commands;
use crate::services::HostCargo;
use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {}

#[derive(Debug, Args)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle(&self) -> Result<(), BuildCommandError> {
        let cargo = HostCargo;

        commands::BuildCommand::handle(&cargo).unwrap();

        Ok(())
    }
}
