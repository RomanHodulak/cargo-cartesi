use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use thiserror::Error;
use crate::services::HostCartesiMachine;

#[derive(Debug, Error)]
pub enum CreateMachineCommandError {}

#[derive(Debug)]
pub struct CreateMachineCommand;

impl CreateMachineCommand {
    pub fn handle(target_binary: &str) -> Result<(), CreateMachineCommandError> {
        let target_binary = Path::new(target_binary)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        HostCartesiMachine::build(target_binary);

        Ok(())
    }
}
