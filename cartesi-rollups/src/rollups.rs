use async_trait::async_trait;
use thiserror::Error;
use crate::http::HttpRollups;

#[derive(Debug, Error)]
pub enum RollupsError {
}

#[async_trait]
pub trait Rollups {
    async fn add_notice(&mut self, payload: &[u8]) -> Result<(), RollupsError>;

    async fn finish(&mut self) -> Result<(), RollupsError>;
}
