use crate::services::{CartesiMachine, DockerCartesiMachine, HostCartesiMachine};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug)]
pub struct RunCommand;

impl RunCommand {
    pub fn handle(target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>) -> Result<(), RunCommandError> {
        let target_binary = Path::new(target_binary.as_ref()).file_name().unwrap().to_str().unwrap();

        DockerCartesiMachine.run_one_shot(target_binary, dapp_fs);

        Ok(())
    }
}
