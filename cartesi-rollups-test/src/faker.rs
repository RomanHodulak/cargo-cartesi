//! Items in this module implement [`MachineIo`] in-memory with pre-programmed set of requests.
//!
//! This kind of implementation is useful for unit tests with a set of requests triggering your target scenario while
//! also being free of side-effects and external dependencies.
//!
//! # Examples
//!
//! ```
//! # use std::error::Error;
//! # use cartesi_rollups::{MachineIo, RollupsRequest};
//! # pub fn run(machine: impl MachineIo) -> Result<(), Box<dyn Error>> {
//! # let address = [0u8; 20];
//! let request = machine.submit()?;
//!
//! match request {
//!     RollupsRequest::AdvanceState { payload, .. } => {
//!         machine.write_notice(&payload)?;
//!         machine.write_voucher(&address, &payload)?;
//!     }
//!     RollupsRequest::InspectState { payload } => {
//!         machine.write_report(&payload)?;
//!     }
//! }
//! # Ok(())
//! # }
//! ```
use cartesi_rollups::{MachineIo, RollupsRequest};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Defines errors of in-memory Cartesi rollup device.
///
/// This variant [`FakeCartesiMachineError::EmptyRequests`] is thrown after [`submitting`] past the last popped request.
///
/// [`submitting`]: MachineIo::submit
#[derive(Error, Debug)]
pub enum FakeCartesiMachineError {
    #[error("Reached end of request queue.")]
    EmptyRequests,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Data {
    pub notices: Vec<Vec<u8>>,
    pub vouchers: Vec<(Vec<u8>, Vec<u8>)>,
    pub reports: Vec<Vec<u8>>,
    pub exceptions: Vec<Vec<u8>>,
}

/// See the [module-level documentation](./index.html) for more details.
#[derive(Clone, Debug, Default)]
pub struct FakeCartesiMachine {
    requests: RefCell<VecDeque<RollupsRequest>>,
    data: Rc<RefCell<Data>>,
}

impl FakeCartesiMachine {
    /// Creates new in-memory Cartesi rollup device popping `requests` per [`submit`].
    ///
    /// [`submit`]: MachineIo::submit
    pub fn new(requests: impl IntoIterator<Item = RollupsRequest>, data: Rc<RefCell<Data>>) -> Self {
        Self {
            requests: RefCell::new(requests.into_iter().collect()),
            data,
        }
    }
}

impl MachineIo for FakeCartesiMachine {
    fn write_notice(&self, payload: &[u8]) -> Result<usize> {
        self.data.borrow_mut().notices.push(payload.to_vec());
        Ok(payload.len())
    }

    fn write_voucher(&self, address: &[u8; 20], payload: &[u8]) -> Result<usize> {
        self.data
            .borrow_mut()
            .vouchers
            .push((address.to_vec(), payload.to_vec()));
        Ok(payload.len())
    }

    fn write_report(&self, payload: &[u8]) -> Result<()> {
        self.data.borrow_mut().reports.push(payload.to_vec());
        Ok(())
    }

    fn submit(&self) -> Result<RollupsRequest> {
        Ok(self
            .requests
            .borrow_mut()
            .pop_front()
            .ok_or_else(|| FakeCartesiMachineError::EmptyRequests)?)
    }

    fn throw_exception(&self, payload: &[u8]) -> Result<()> {
        self.data.borrow_mut().exceptions.push(payload.to_vec());
        Ok(())
    }
}
