use std::io::Write;
use std::process::Command;
use std::{env, io};
use std::path::PathBuf;
use thiserror::Error;
use crate::services::HostCargo;

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
