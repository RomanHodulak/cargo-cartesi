use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {}

#[derive(Debug)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle(target_bin: String) -> Result<(), BuildCommandError> {
        // cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target riscv64ima-cartesi-linux-gnu.json --release
        let cargo_path = std::env::var("CARGO").expect("The `CARGO` environment variable was not set. This is unexpected: it should always be provided by `cargo` when invoking a custom sub-command, allowing `cargo-cartesi` to correctly detect which toolchain should be used. Please file a bug.");
        let mut command = Command::new(cargo_path);
        command.arg("build");
        command.arg("-Z");
        command.arg("build-std=std,core,alloc,panic_abort,proc_macro");
        command.arg("--color");
        command.arg("always");
        command.arg("--target");
        command.arg("riscv64ima-cartesi-linux-gnu.json");
        command.arg("--release");

        Ok(())
    }
}
