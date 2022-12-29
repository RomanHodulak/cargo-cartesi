use crate::services::FileSystem;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct HostFileSystem;

impl FileSystem for HostFileSystem {
    fn create(
        &self,
        files: impl IntoIterator<Item = impl Into<PathBuf>>,
        size: Option<usize>,
        output: impl Into<PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        let temp_dir = Self::temp_dir().into();
        let tar = Self::temp_file().into();

        Command::new("rsync")
            .arg("-r")
            .args(files.into_iter().map(|file| file.into()))
            .arg(&temp_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `rsync`");

        Command::new("tar")
            .arg("-cf")
            .arg(&tar)
            .arg("-C")
            .arg(&temp_dir)
            .arg(".")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process `tar`");

        let size_in_blocks = size.map(|v| v.to_string()).unwrap_or_else(|| "4096".to_owned());
        let output_fs = output.into();

        Command::new("genext2fs")
            .arg("-f")
            .arg("-i")
            .arg("512")
            .arg("-b")
            .arg(size_in_blocks)
            .arg("-a")
            .arg(tar)
            .arg(&output_fs)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `genext2fs`");

        Command::new("truncate")
            .arg("-s")
            .arg("%4096")
            .arg(output_fs)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `truncate`");

        Ok(())
    }
}

impl HostFileSystem {
    fn temp_dir() -> impl Into<PathBuf> {
        let output = Command::new("mktemp")
            .arg("-d")
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process `mktemp`");

        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn temp_file() -> impl Into<PathBuf> {
        let output = Command::new("mktemp")
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process `mktemp`");

        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }
}
