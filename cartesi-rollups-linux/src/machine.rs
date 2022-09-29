use crate::rollup;
use crate::rollup::{Exception, Notice, Report, RollupRequest, Voucher};
use cartesi_rollups::{MachineIo, RollupsMetadata, RollupsRequest};
use std::error::Error;
use std::fs::File;
use std::io;
use std::os::unix::prelude::{IntoRawFd, RawFd};
use std::path::Path;

pub struct LinuxMachine {
    /// File descriptor pointing to the rollup device.
    fd: RawFd,
}

impl LinuxMachine {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }

    pub fn open_default_device() -> Result<Self, io::Error> {
        Self::open(rollup::ROLLUP_DEVICE_NAME)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let rollup_file = File::open(path).map_err(|e| {
            log::error!("error opening rollup device {}", e.to_string());

            e
        })?;
        let fd = rollup_file.into_raw_fd();

        Ok(Self::new(fd))
    }
}

impl MachineIo for LinuxMachine {
    fn write_notice(&self, payload: &[u8]) -> Result<usize, Box<dyn Error>> {
        let mut notice = Notice {
            payload: String::from_utf8(payload.to_vec())?,
        };

        rollup::write_notice(self.fd, &mut notice).map(|v| v as usize)
    }

    fn write_voucher(&self, address: &[u8; 20], payload: &[u8]) -> Result<usize, Box<dyn Error>> {
        let mut voucher = Voucher {
            address: String::from_utf8(address.to_vec())?,
            payload: String::from_utf8(payload.to_vec())?,
        };

        rollup::write_voucher(self.fd, &mut voucher).map(|v| v as usize)
    }

    fn write_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut report = Report {
            payload: String::from_utf8(payload.to_vec())?,
        };

        rollup::write_report(self.fd, &mut report)
    }

    fn submit(&self) -> Result<RollupsRequest, Box<dyn Error>> {
        let finish = rollup::perform_rollup_finish_request(self.fd, true)?;
        let request = rollup::handle_rollup_requests(self.fd, finish)?;

        Ok(match request {
            RollupRequest::Inspect(request) => RollupsRequest::InspectState {
                payload: request.payload.into_bytes(),
            },
            RollupRequest::Advance(request) => RollupsRequest::AdvanceState {
                metadata: RollupsMetadata {
                    msg_sender: request.metadata.msg_sender,
                    epoch_index: request.metadata.epoch_index,
                    input_index: request.metadata.input_index,
                    block_number: request.metadata.block_number,
                    timestamp: request.metadata.timestamp,
                },
                payload: request.payload.into_bytes(),
            },
        })
    }

    fn throw_exception(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        let exception = Exception {
            payload: String::from_utf8(payload.to_vec())?,
        };

        rollup::throw_exception(self.fd, &exception)
    }
}
