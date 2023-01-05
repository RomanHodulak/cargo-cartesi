use crate::services::{Cargo, DependenciesDownloader, ResourceCreator};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewCommandError {}

#[derive(Debug)]
pub struct NewCommand;

impl NewCommand {
    pub fn handle(
        crate_name: impl AsRef<str>,
        deps: &impl DependenciesDownloader,
        res: &impl ResourceCreator,
        cargo: &impl Cargo,
    ) -> Result<(), NewCommandError> {
        cargo.create_new_binary_source(&crate_name);
        res.create(&crate_name).unwrap();
        deps.download_if_not_present_and_verify(crate_name).unwrap();

        Ok(())
    }
}
