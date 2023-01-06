use crate::commands;
use crate::services::Cargo;
use clap::Args;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle(&self, cargo: impl Cargo) -> Result<(), BuildCommandError> {
        commands::BuildCommand::handle(&cargo).unwrap();

        Ok(())
    }
}
