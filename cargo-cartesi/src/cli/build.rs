use crate::commands;
use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {}

#[derive(Debug, Args)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle(&self) -> Result<(), BuildCommandError> {
        commands::BuildCommand::handle().unwrap();

        Ok(())
    }
}
