use std::io::Write;
use std::process::Command;
use std::{env, io};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {}

#[derive(Debug)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle() -> Result<(), BuildCommandError> {
        let cargo_path = env::var("CARGO").expect("The `CARGO` environment variable was not set. This is unexpected: it should always be provided by `cargo` when invoking a custom sub-command, allowing `cargo-cartesi` to correctly detect which toolchain should be used. Please file a bug.");
        let mut command = Command::new(cargo_path);
        command
            .arg("build")
            .arg("-Z")
            .arg("build-std=std,core,alloc,panic_abort,proc_macro")
            .arg("--color")
            .arg("always")
            .arg("--target")
            .arg(format!("{}.json", Self::target_name()))
            .arg("--release");

        let output = command.output().expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
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
