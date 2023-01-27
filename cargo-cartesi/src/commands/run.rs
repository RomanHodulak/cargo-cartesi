use crate::services::{Cargo, CartesiMachine, DependenciesDownloader, FileSystem};
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
        deps: &impl DependenciesDownloader,
        cargo: &impl Cargo,
        file_system: &impl FileSystem,
        cartesi_machine: &impl CartesiMachine,
    ) -> Result<(), RunCommandError> {
        cargo.build_binary();

        let target_bin = target_binary.unwrap_or(cargo.package_name().unwrap());
        let target_dir = PathBuf::from(cargo.target_dir().unwrap()).join(&target_bin);
        let target_cartesi_dir = PathBuf::from(cargo.target_dir().unwrap()).join("cartesi");

        std::fs::create_dir_all(&target_cartesi_dir).unwrap();

        let rootfs = target_cartesi_dir.join("rootfs.ext2");
        let ram_image = target_cartesi_dir.join("linux-5.5.19-ctsi-6.bin");
        let rom_image = target_cartesi_dir.join("rom.bin");
        let output = target_cartesi_dir.join(dapp_fs.as_ref());

        let rootfs = rootfs.to_str().unwrap();
        let ram_image = ram_image.to_str().unwrap();
        let rom_image = rom_image.to_str().unwrap();
        let output = output.to_str().unwrap();

        file_system.create(once(target_dir), None, output).unwrap();
        deps.download_if_not_present_and_verify(target_cartesi_dir).unwrap();
        cartesi_machine.run(target_bin, rootfs, ram_image, rom_image, output);

        Ok(())
    }
}
