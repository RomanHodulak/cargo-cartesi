use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use thiserror::Error;
use crate::services::HostCartesiMachine;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug)]
pub struct RunCommand;

impl RunCommand {
    pub fn handle(target_binary: &str) -> Result<(), RunCommandError> {
        let target_binary = Path::new(target_binary)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        HostCartesiMachine::run_one_shot(target_binary);

        Ok(())
    }
}
