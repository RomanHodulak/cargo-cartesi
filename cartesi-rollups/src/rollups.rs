use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::future::Future;
use thiserror::Error;

/// Request sent from the rollups server.
///
/// For example, the rollups server received some inputs and now wants the dapp to advance state.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RollupsRequest {
    pub request_type: RequestType,
    pub data: RollupsMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RequestType {
    #[serde(rename = "advance_state")]
    AdvanceState,
    #[serde(rename = "inspect_state")]
    InspectState,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RollupsMessage {
    pub metadata: RollupsMetadata,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RollupsMetadata {
    pub msg_sender: String,
    pub epoch_index: u64,
    pub input_index: u64,
    pub block_number: u64,
    pub timestamp: u64,
}

#[derive(Debug, Error)]
pub enum RollupsError {}

#[async_trait]
pub trait Rollups {
    async fn add_notice(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;

    async fn add_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;

    async fn run<F1, F2, Fut1, Fut2>(
        &mut self,
        advance_state_handler: F1,
        inspect_state_handler: F2,
    ) -> Result<(), RollupsError>
    where
        F1: Fn(RollupsMessage) -> Fut1 + Send,
        F2: Fn(RollupsMessage) -> Fut2 + Send,
        Fut1: Future<Output = Result<bool, Box<dyn Error + 'static>>> + Send,
        Fut2: Future<Output = Result<bool, Box<dyn Error + 'static>>> + Send;
}
