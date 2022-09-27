use clap::Args;
use std::io::{self, Write};
use std::process::{Command, ExitCode, ExitStatus};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewCommandError {}

#[derive(Debug, Args)]
pub struct NewCommand {
    #[clap(value_parser)]
    pub target_dir: Option<String>,
}

impl NewCommand {
    pub fn handle(target_bin: &str) -> Result<ExitCode, NewCommandError> {
        let cargo_path = std::env::var("CARGO").expect("The `CARGO` environment variable was not set. This is unexpected: it should always be provided by `cargo` when invoking a custom sub-command, allowing `cargo-cartesi` to correctly detect which toolchain should be used. Please file a bug.");
        let mut command = Command::new(cargo_path);
        command.arg("new");
        command.arg("--color");
        command.arg("always");
        command.arg(target_bin);

        println!("     Running `{:?}`", command);
        let output = command.output().expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(u8::try_from(output.status.code().unwrap_or_default())
            .unwrap_or_default()
            .into())
    }
}
