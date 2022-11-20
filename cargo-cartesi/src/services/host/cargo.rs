use std::{env, io};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub struct HostCargo;

impl HostCargo {
    pub fn create_new_binary_source() {
        let mut command = Self::cargo();
        command
            .arg("new");

        let output = command.output().expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    pub fn build_binary() {
        let mut command = Self::cargo();
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
    }

    fn create_path(binary_name: &str) -> PathBuf {
        let target_name = Self::target_name();

        PathBuf::new()
            .join("target")
            .join(target_name)
            .join("release")
            .join(binary_name)
    }

    fn target_name() -> &'static str {
        "riscv64ima-cartesi-linux-gnu"
    }

    fn cargo() -> Command {
        Command::new(Self::cargo_path())
    }

    fn cargo_path() -> String {
        env::var("CARGO").expect("The `CARGO` environment variable was not set. This is unexpected: it should always be provided by `cargo` when invoking a custom sub-command, allowing `cargo-cartesi` to correctly detect which toolchain should be used. Please file a bug.")
    }
}
