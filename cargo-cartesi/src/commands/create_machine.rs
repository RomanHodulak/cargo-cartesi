use crate::services::{Cargo, CartesiMachine, DependenciesDownloader, FileSystem};
use std::iter::once;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateMachineCommandError {}

#[derive(Debug)]
pub struct CreateMachineCommand;

impl CreateMachineCommand {
    pub fn handle(
        target_binary: Option<String>,
        dapp_fs: impl AsRef<str>,
        deps: &impl DependenciesDownloader,
        cargo: &impl Cargo,
        file_system: &impl FileSystem,
        cartesi_machine: &impl CartesiMachine,
    ) -> Result<(), CreateMachineCommandError> {
        cargo.build_binary();

        let target_bin = target_binary.unwrap_or_else(|| cargo.package_name().unwrap());
        let target_dir = PathBuf::from(cargo.target_dir().unwrap()).join(&target_bin);
        let target_cartesi_dir = PathBuf::from(cargo.target_dir().unwrap()).join("cartesi");

        std::fs::create_dir_all(&target_cartesi_dir).unwrap();

        let output = target_cartesi_dir.join(dapp_fs.as_ref());
        let output = output.to_str().unwrap();

        file_system.create(once(target_dir), None, output).unwrap();
        deps.download_if_not_present_and_verify(target_cartesi_dir).unwrap();
        cartesi_machine.build(target_bin, output);

        Ok(())
    }
}
