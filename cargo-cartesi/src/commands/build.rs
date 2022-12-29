use crate::services::Cargo;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {}

#[derive(Debug)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle(cargo: &impl Cargo) -> Result<(), BuildCommandError> {
        cargo.build_binary();

        Ok(())
    }
}
