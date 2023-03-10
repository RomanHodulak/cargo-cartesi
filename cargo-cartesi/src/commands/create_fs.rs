use crate::services::{Cargo, FileSystem};
use std::iter::once;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateFsCommandError {}

#[derive(Debug)]
pub struct CreateFsCommand;

impl CreateFsCommand {
    /// Creates a file-system suitable to mount in Cartesi machine.
    ///
    /// This command takes the `target_binary`, creates a file-system image and stores it in `dapp_fs`.
    pub fn handle(
        target_binary: Option<String>,
        dapp_fs: impl AsRef<str>,
        cargo: &impl Cargo,
        file_system: &impl FileSystem,
    ) -> Result<(), CreateFsCommandError> {
        cargo.build_binary();

        let target_bin = target_binary.unwrap_or(cargo.package_name().unwrap());
        let target_dir = PathBuf::from(cargo.target_dir().unwrap()).join(target_bin);
        let target_cartesi_dir = PathBuf::from(cargo.target_dir().unwrap()).join("cartesi");

        std::fs::create_dir_all(&target_cartesi_dir).unwrap();

        let output = target_cartesi_dir.join(dapp_fs.as_ref());

        file_system.create(once(target_dir), None, output).unwrap();

        Ok(())
    }
}
