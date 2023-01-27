use crate::services::{Cargo, ResourceCreator};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewCommandError {}

#[derive(Debug)]
pub struct NewCommand;

impl NewCommand {
    pub fn handle(
        crate_name: impl AsRef<str>,
        res: &impl ResourceCreator,
        cargo: &impl Cargo,
    ) -> Result<(), NewCommandError> {
        cargo.create_new_binary_source(&crate_name);
        res.create(&crate_name).unwrap();

        Ok(())
    }
}
