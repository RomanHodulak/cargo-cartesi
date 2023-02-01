use crate::services::CartesiMachine;
use std::fs;
use std::process::{Command, Stdio};

pub struct HostCartesiMachine;

impl CartesiMachine for HostCartesiMachine {
    fn build(
        &self,
        target_binary: impl AsRef<str>,
        rootfs: impl AsRef<str>,
        ram_image: impl AsRef<str>,
        rom_image: impl AsRef<str>,
        dapp_fs: impl AsRef<str>,
    ) {
        let machine_dir = "machine";
        let _ = fs::remove_dir_all(machine_dir);

        let mut command = Self::command(rootfs, ram_image, rom_image, dapp_fs);
        Self::run_command_for_binary(
            command.arg("--rollup").arg(format!("--store=pwd/{}", machine_dir)),
            target_binary,
        );
    }

    fn run(
        &self,
        target_binary: impl AsRef<str>,
        rootfs: impl AsRef<str>,
        ram_image: impl AsRef<str>,
        rom_image: impl AsRef<str>,
        dapp_fs: impl AsRef<str>,
    ) {
        let mut command = Self::command(rootfs, ram_image, rom_image, dapp_fs);
        Self::run_command_for_binary(&mut command, target_binary);
    }
}

impl HostCartesiMachine {
    fn command(
        rootfs: impl AsRef<str>,
        ram_image: impl AsRef<str>,
        rom_image: impl AsRef<str>,
        dapp_fs: impl AsRef<str>,
    ) -> Command {
        let ram_length = "128Mi";
        let mut command = Command::new("cartesi-machine");

        command
            .arg(format!("--ram-length={}", ram_length))
            .arg(format!("--flash-drive=label:dapp,filename:{}", dapp_fs.as_ref()))
            .arg(format!("--flash-drive=label:root,filename:{}", rootfs.as_ref()))
            .arg(format!("--ram-image={}", ram_image.as_ref()))
            .arg(format!("--rom-image={}", rom_image.as_ref()));

        command
    }

    fn run_command_for_binary(command: &mut Command, target_binary: impl AsRef<str>) {
        command
            .arg("--")
            .arg(format!("cd /mnt/dapp; ./{}", target_binary.as_ref()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `cartesi-machine`");
    }
}
