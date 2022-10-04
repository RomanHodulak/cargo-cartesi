use std::io::Write;
use std::process::Command;
use std::{env, io};
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
            .arg("riscv64ima-cartesi-linux-gnu.json")
            .arg("--release");

        let output = command.output().expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }
}
