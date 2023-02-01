use crate::services::CartesiMachine;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

pub struct DockerCartesiMachine;

impl CartesiMachine for DockerCartesiMachine {
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

impl DockerCartesiMachine {
    fn command(
        rootfs: impl AsRef<str>,
        ram_image: impl AsRef<str>,
        rom_image: impl AsRef<str>,
        dapp_fs: impl AsRef<str>,
    ) -> Command {
        let path = PathBuf::from(rootfs.as_ref());
        let path = path.ancestors().nth(1).unwrap().to_str().unwrap();
        let rootfs = rootfs.as_ref().strip_prefix(path).unwrap();
        let ram_image = ram_image.as_ref().strip_prefix(path).unwrap();
        let rom_image = rom_image.as_ref().strip_prefix(path).unwrap();
        let dapp_fs = dapp_fs.as_ref().strip_prefix(path).unwrap();
        let mut command = Command::new("docker");

        command
            .arg("run")
            .arg("--volume")
            .arg(format!("{}:/opt/cartesi/bin/dapp", path))
            .arg("--volume")
            .arg(format!(
                "{}:/opt/cartesi/bin/pwd",
                env::current_dir().unwrap().to_str().unwrap()
            ))
            .arg("-u")
            .arg(format!("{}:{}", users::get_current_uid(), users::get_current_gid()))
            .arg("-t")
            .arg("cartesi/server-manager:0.4.0")
            .arg("cartesi-machine")
            .arg(format!("--flash-drive=label:root,filename:dapp/{}", rootfs))
            .arg(format!("--flash-drive=label:dapp,filename:dapp/{}", dapp_fs))
            .arg(format!("--ram-image=dapp/{}", ram_image))
            .arg(format!("--rom-image=dapp/{}", rom_image));

        command
    }

    fn run_command_for_binary(command: &mut Command, target_binary: impl AsRef<str>) {
        command
            .arg("--")
            .arg(format!("cd /mnt/dapp; ./{}", target_binary.as_ref()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `docker`");
    }
}
