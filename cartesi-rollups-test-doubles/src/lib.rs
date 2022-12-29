use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use cartesi_rollups::{MachineIo, RollupsRequest};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum FakeCartesiMachineError {
    #[error("Reached end of request queue.")]
    EmptyRequests,
}

#[derive(Clone, Debug, Default)]
pub struct FakeCartesiMachine {
    requests: RefCell<VecDeque<RollupsRequest>>,
    notices: RefCell<Vec<Vec<u8>>>,
    vouchers: RefCell<Vec<(Vec<u8>, Vec<u8>)>>,
    reports: RefCell<Vec<Vec<u8>>>,
    exceptions: RefCell<Vec<Vec<u8>>>,
}

impl FakeCartesiMachine {
    pub fn new(requests: VecDeque<RollupsRequest>) -> Self {
        Self { requests: RefCell::new(requests), ..Default::default() }
    }
}

impl MachineIo for FakeCartesiMachine {
    fn write_notice(&self, payload: &[u8]) -> Result<usize, Box<dyn Error>> {
        self.notices.borrow_mut().push(payload.to_vec());
        Ok(payload.len())
    }

    fn write_voucher(&self, address: &[u8; 20], payload: &[u8]) -> Result<usize, Box<dyn Error>> {
        self.vouchers.borrow_mut().push((address.to_vec(), payload.to_vec()));
        Ok(payload.len())
    }

    fn write_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        self.reports.borrow_mut().push(payload.to_vec());
        Ok(())
    }

    fn submit(&self) -> Result<RollupsRequest, Box<dyn Error>> {
        Ok(self.requests
            .borrow_mut()
            .pop_front()
            .ok_or_else(|| FakeCartesiMachineError::EmptyRequests)?)
    }

    fn throw_exception(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        self.exceptions.borrow_mut().push(payload.to_vec());
        Ok(())
    }
}
