use std::error::Error;

/// Request sent from the rollups server.
///
/// For example, the rollups server received some inputs and now wants the dapp to advance state.
#[derive(Clone, Debug)]
pub enum RollupsRequest {
    AdvanceState {
        metadata: RollupsMetadata,
        payload: Vec<u8>,
    },
    InspectState {
        payload: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub struct RollupsMetadata {
    pub msg_sender: String,
    pub epoch_index: u64,
    pub input_index: u64,
    pub block_number: u64,
    pub timestamp: u64,
}

pub trait MachineIo {
    fn write_notice(&self, payload: &[u8]) -> Result<usize, Box<dyn Error>>;

    fn write_voucher(&self, address: &[u8; 20], payload: &[u8]) -> Result<usize, Box<dyn Error>>;

    fn write_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;

    /// Submits all writes since the last submit.
    fn submit(&self) -> Result<RollupsRequest, Box<dyn Error>>;

    /// Rolls-back the entire machine.
    fn throw_exception(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;
}
