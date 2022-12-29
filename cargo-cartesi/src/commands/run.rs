use crate::services::{Cargo, CartesiMachine, FileSystem};
use std::iter::once;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug)]
pub struct RunCommand;

impl RunCommand {
    pub fn handle(
        target_binary: Option<String>,
        dapp_fs: impl AsRef<str>,
        cargo: &impl Cargo,
        file_system: &impl FileSystem,
        cartesi_machine: &impl CartesiMachine,
    ) -> Result<(), RunCommandError> {
        let target_bin = target_binary.unwrap_or(cargo.package_name().unwrap());
        let target_dir = PathBuf::from(cargo.target_dir().unwrap()).join(&target_bin);

        cargo.build_binary();
        file_system.create(once(target_dir), None, dapp_fs.as_ref()).unwrap();
        cartesi_machine.run_one_shot(target_bin, dapp_fs);

        Ok(())
    }
}
