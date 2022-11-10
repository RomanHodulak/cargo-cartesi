use std::io;
use std::io::Write;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug)]
pub struct RunCommand;

impl RunCommand {
    pub fn handle(target_binary: &str) -> Result<(), RunCommandError> {
        let machine_dir = "machine-store";
        let ram_length = "128Mi";
        let dappfs = "dapp.ext2";
        let rootfs = "rootfs.ext2";
        let ram_image = "linux-5.5.19-ctsi-6.bin";
        let rom_image = "rom.bin";

        let mut command = Command::new("cartesi-machine");
        command
            .arg("--rollup")
            .arg(format!("--ram-length={}", ram_length))
            .arg(format!("--flash-drive=label:dapp,filename:{}", dappfs))
            .arg(format!("--flash-drive=label:root,filename:{}", rootfs))
            .arg(format!("--ram-image={}", ram_image))
            .arg(format!("--rom-image={}", rom_image))
            .arg(format!("--store={}", machine_dir))
            .arg("--")
            .arg(format!("cd /mnt/dapp; ./{}", target_binary));

        let output = command.output().expect("failed to run process `cartesi-machine`");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }
}
