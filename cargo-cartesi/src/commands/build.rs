use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildCommandError {
}

#[derive(Debug)]
pub struct BuildCommand;

impl BuildCommand {
    pub fn handle(target_bin: String) -> Result<(), BuildCommandError> {
        // cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target riscv64ima-cartesi-linux-gnu.json --release

        Ok(())
    }
}
