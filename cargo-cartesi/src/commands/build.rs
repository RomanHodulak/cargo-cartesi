use crate::services::HostCargo;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {}

#[derive(Debug)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle() -> Result<(), BuildCommandError> {
        HostCargo::build_binary();

        Ok(())
    }
}
