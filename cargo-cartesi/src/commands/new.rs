use crate::services::{Cargo, HostCargo};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewCommandError {}

#[derive(Debug)]
pub struct NewCommand;

impl NewCommand {
    pub fn handle(_target_binary: &str) -> Result<(), NewCommandError> {
        HostCargo.create_new_binary_source();

        Ok(())
    }
}
