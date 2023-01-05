use crate::services::DependenciesDownloader;
use hex_literal::hex;
use sha1::{Digest, Sha1};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};

pub struct HostDependencyDownloader;

impl DependenciesDownloader for HostDependencyDownloader {
    fn download_if_not_present_and_verify(&self, target_dir: impl AsRef<str>) -> Result<(), Box<dyn Error>> {
        let items = Self::DEPENDENCIES
            .into_iter()
            .collect::<Vec<(&'static str, &'static str, [u8; 20])>>();

        Command::new("wget")
            .args(items.iter().map(|v| v.0))
            .arg("-P")
            .arg(target_dir.as_ref())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `wget`");

        for (_, path, hash) in items {
            Self::verify(&format!("{}/{}", target_dir.as_ref(), path), hash);
        }

        Ok(())
    }
}

impl HostDependencyDownloader {
    const DEPENDENCIES: [(&'static str, &'static str, [u8; 20]); 3] = [
        (
            "https://github.com/cartesi/image-rootfs/releases/download/v0.14.1/rootfs.ext2",
            "rootfs.ext2",
            hex!("7c4ee44b48e821f66036dcb6a7bb49ddf8c88623"),
        ),
        (
            "https://github.com/cartesi/machine-emulator-rom/releases/download/v0.12.0/rom.bin",
            "rom.bin",
            hex!("804644b4123c3402a579e7c52aa24e86805c302f"),
        ),
        (
            "https://github.com/cartesi/image-kernel/releases/download/v0.13.0/linux-5.5.19-ctsi-6.bin",
            "linux-5.5.19-ctsi-6.bin",
            hex!("6e85569297f751e44568114e443b01995df24a63"),
        ),
    ];

    fn verify(path: impl AsRef<str>, hash: [u8; 20]) {
        let mut file = File::open(path.as_ref()).unwrap();
        let mut buffer = vec![];
        let mut hasher = Sha1::new();
        file.read_to_end(&mut buffer).unwrap();
        hasher.update(buffer);
        let result = hasher.finalize();
        assert_eq!(result[..], hash);
        println!("{}: Verified OK", path.as_ref());
    }
}
