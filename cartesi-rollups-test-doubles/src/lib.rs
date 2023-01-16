use cartesi_rollups::{MachineIo, RollupsRequest};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

#[derive(Clone, Debug, Default)]
pub struct FakeCartesiMachine {
    requests: RefCell<VecDeque<RollupsRequest>>,
    data: Rc<RefCell<Data>>,
}

impl FakeCartesiMachine {
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
