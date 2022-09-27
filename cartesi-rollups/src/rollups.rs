use std::error::Error;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RollupsError {
}

#[async_trait]
pub trait Rollups {
    async fn add_notice(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;

    async fn add_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;

    async fn run(&mut self) -> Result<(), RollupsError>;
}
