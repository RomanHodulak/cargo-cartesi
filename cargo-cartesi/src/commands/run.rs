use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug)]
pub struct RunCommand;

impl RunCommand {
    pub fn handle(target_bin: String) -> Result<(), RunCommandError> {
        // rollup-init ./target/riscv64ima-cartesi-linux-gnu/release/echo-backend

        Ok(())
    }
}
