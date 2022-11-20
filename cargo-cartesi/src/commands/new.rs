use std::io;
use std::io::Write;
use std::process::Command;
use thiserror::Error;
use crate::services::HostCargo;

#[derive(Debug, Error)]
pub enum NewCommandError {}

#[derive(Debug)]
pub struct NewCommand;

impl NewCommand {
    pub fn handle(target_binary: &str) -> Result<(), NewCommandError> {
        HostCargo::create_new_binary_source();

        Ok(())
    }
}
