use crate::services::CartesiMachine;
use std::process::{Command, Stdio};

pub struct HostCartesiMachine;

impl CartesiMachine for HostCartesiMachine {
    fn build(&self, target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>) {
        let ram_length = "128Mi";
        let rootfs = "rootfs.ext2";
        let ram_image = "linux-5.5.19-ctsi-6.bin";
        let rom_image = "rom.bin";
        let machine_dir = "machine";

        Command::new("cartesi-machine")
            .arg("--rollup")
            .arg(format!("--ram-length={}", ram_length))
            .arg(format!("--flash-drive=label:dapp,filename:{}", dapp_fs.as_ref()))
            .arg(format!("--flash-drive=label:root,filename:{}", rootfs))
            .arg(format!("--ram-image={}", ram_image))
            .arg(format!("--rom-image={}", rom_image))
            .arg(format!("--store={}", machine_dir))
            .arg("--")
            .arg(format!("cd /mnt/dapp; ./{}", target_binary.as_ref()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `cartesi-machine`");
    }

    fn run(
        &self,
        target_binary: impl AsRef<str>,
        rootfs: impl AsRef<str>,
        ram_image: impl AsRef<str>,
        rom_image: impl AsRef<str>,
        dapp_fs: impl AsRef<str>,
    ) {
        let ram_length = "128Mi";

        Command::new("cartesi-machine")
            .arg(format!("--ram-length={}", ram_length))
            .arg(format!("--flash-drive=label:dapp,filename:{}", dapp_fs.as_ref()))
            .arg(format!("--flash-drive=label:root,filename:{}", rootfs.as_ref()))
            .arg(format!("--ram-image={}", ram_image.as_ref()))
            .arg(format!("--rom-image={}", rom_image.as_ref()))
            .arg("--")
            .arg(format!("cd /mnt/dapp; ./{}", target_binary.as_ref()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `cartesi-machine`");
    }
}
