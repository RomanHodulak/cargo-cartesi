use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunCommandError {}

#[derive(Debug)]
pub struct RunCommand;

impl RunCommand {
    pub fn handle() -> Result<(), RunCommandError> {
        Ok(())
    }
}
