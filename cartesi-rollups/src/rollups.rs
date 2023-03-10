//! Items in this module define the Cartesi Rollup communication device abstraction.
use std::error::Error;

/// Request sent from the rollups server.
///
/// For example, the rollups server received some inputs and now wants the dapp to advance state.
#[derive(Clone, Debug)]
pub enum RollupsRequest {
    /// The handler should respond to this request by advancing the state of the dapp using the `payload` and `metadata`.
    AdvanceState {
        metadata: RollupsMetadata,
        payload: Vec<u8>,
    },
    /// The handler should respond to this request by inspecting current state of the dapp using the `payload` and not
    /// advance the state.
    InspectState { payload: Vec<u8> },
}

/// Metadata exactly describing the input order accompanying the [`RollupsRequest`].
#[derive(Clone, Debug)]
pub struct RollupsMetadata {
    pub msg_sender: String,
    pub epoch_index: u64,
    pub input_index: u64,
    pub block_number: u64,
    pub timestamp: u64,
}

/// The implementor of this trait handles communication with the rollup device.
pub trait MachineIo {
    /// Writes a notice with `payload`.
    fn write_notice(&self, payload: &[u8]) -> Result<usize, Box<dyn Error>>;

    /// Writes a voucher with `payload` for `address`.
    fn write_voucher(&self, address: &[u8; 20], payload: &[u8]) -> Result<usize, Box<dyn Error>>;

    /// Writes a report with `payload`.
    fn write_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;

    /// Submits all writes since the last submit and retrieves next [`RollupsRequest`] to handle.
    ///
    /// Blocks the current thread. Needs to be called before the first write call.
    fn submit(&self) -> Result<RollupsRequest, Box<dyn Error>>;

    /// Rolls-back the entire machine and writes exception with `payload`.
    fn throw_exception(&self, payload: &[u8]) -> Result<(), Box<dyn Error>>;
}
