use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateFsCommandError {}

#[derive(Debug)]
pub struct CreateFsCommand;

impl CreateFsCommand {
    /// Creates a file-system suitable to mount in Cartesi machine.
    ///
    /// This command takes the `files`, creates a file-system image of `fs_size` blocks and stores it in
    /// `output_fs`.
    pub fn handle(
        files: impl IntoIterator<Item = impl Into<PathBuf>>,
        fs_size: Option<usize>,
        output_fs: impl Into<PathBuf>,
    ) -> Result<(), CreateFsCommandError> {
        let temp_dir = Self::temp_dir().into();
        let tar = Self::temp_file().into();

        let mut command = Command::new("rsync");
        command.arg("-r");

        for file in files {
            let path = file.into();
            command.arg(&path);
        }

        command
            .arg(&temp_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run process `rsync`");

        Self::tar(temp_dir, &tar);

        let size_in_blocks = fs_size.map(|v| v.to_string()).unwrap_or_else(|| "4096".to_owned());
        let output_fs = output_fs.into();

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

    fn tar(input: impl Into<PathBuf>, output: impl Into<PathBuf>) {
        Command::new("tar")
            .arg("-cf")
            .arg(output.into())
            .arg("-C")
            .arg(input.into())
            .arg(".")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process `tar`");
    }

    fn temp_dir() -> impl Into<PathBuf> {
        let mut command = Command::new("mktemp");
        command.arg("-d");
        let output = command.output().expect("failed to execute process `mktemp`");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn temp_file() -> impl Into<PathBuf> {
        let mut command = Command::new("mktemp");
        let output = command.output().expect("failed to execute process `mktemp`");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn create_path(binary_name: &str) -> PathBuf {
        let target_name = Self::target_name();
        let path = PathBuf::new()
            .join("target")
            .join(target_name)
            .join("release")
            .join(binary_name);

        path
    }

    fn target_name() -> &'static str {
        "riscv64ima-cartesi-linux-gnu"
    }
}
