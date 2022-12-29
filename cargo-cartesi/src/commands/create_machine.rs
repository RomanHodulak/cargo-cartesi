use crate::services::{CartesiMachine, DockerCartesiMachine, HostCartesiMachine};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateMachineCommandError {}

#[derive(Debug)]
pub struct CreateMachineCommand;

impl CreateMachineCommand {
    pub fn handle(target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>) -> Result<(), CreateMachineCommandError> {
        DockerCartesiMachine.build(target_binary, dapp_fs);

        Ok(())
    }
}
