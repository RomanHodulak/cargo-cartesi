use crate::services::CartesiMachine;
use std::process::{Command, Stdio};

pub struct DockerCartesiMachine;

impl CartesiMachine for DockerCartesiMachine {
    fn build(&self, target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>) {
        let machine_dir = "machine";

        Command::new("docker")
            .env("DOCKER_BUILDKIT", "1")
            .arg("build")
            .arg("--build-arg")
            .arg(format!("BINARY={}", target_binary.as_ref()))
            .arg("--build-arg")
            .arg(format!("DAPP_FS={}", dapp_fs.as_ref()))
            .arg(format!("--output={}", machine_dir))
            .arg(".")
            .arg("-t")
            .arg(format!("cartesi/{}:server", target_binary.as_ref()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `docker`");
    }

    fn run(
        &self,
        target_binary: impl AsRef<str>,
        rootfs: impl AsRef<str>,
        ram_image: impl AsRef<str>,
        rom_image: impl AsRef<str>,
        dapp_fs: impl AsRef<str>,
    ) {
        Command::new("docker")
            .arg("run")
            .arg("--volume")
            .arg(format!(
                "{}:/opt/cartesi/bin/dapp",
                std::env::current_dir().unwrap().to_str().unwrap()
            ))
            .arg("-t")
            .arg("cartesi/server-manager:0.4.0")
            .arg("cartesi-machine")
            .arg(format!("--flash-drive=label:root,filename:dapp/{}", rootfs.as_ref()))
            .arg(format!("--flash-drive=label:dapp,filename:dapp/{}", dapp_fs.as_ref()))
            .arg(format!("--ram-image=dapp/{}", ram_image.as_ref()))
            .arg(format!("--rom-image=dapp/{}", rom_image.as_ref()))
            .arg("--")
            .arg(format!("cd /mnt/dapp; ./{}", target_binary.as_ref()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `docker`");
    }
}
