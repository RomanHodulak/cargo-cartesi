use crate::rollups;
use crate::rollups::{Exception, Notice, Report, Voucher};
use cartesi_rollups::{MachineIo, RollupsRequest};
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
        Self::open(rollups::ROLLUP_DEVICE_NAME)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let rollup_file = File::open(path)?;
        let fd = rollup_file.into_raw_fd();

        Ok(Self::new(fd))
    }
}

impl MachineIo for LinuxMachine {
    fn write_notice(&self, payload: &[u8]) -> Result<usize, Box<dyn Error>> {
        let mut notice = Notice::try_from(payload)?;

        rollups::write_notice(self.fd, &mut notice).map(|v| v as usize)
    }

    fn write_voucher(&self, address: &[u8; 20], payload: &[u8]) -> Result<usize, Box<dyn Error>> {
        let mut voucher = Voucher::try_from((address, payload))?;

        rollups::write_voucher(self.fd, &mut voucher).map(|v| v as usize)
    }

    fn write_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        let report = Report::try_from(payload)?;

        rollups::write_report(self.fd, &report)
    }

    fn submit(&self) -> Result<RollupsRequest, Box<dyn Error>> {
        let finish = rollups::perform_rollup_finish_request(self.fd, true)?;

        Ok(rollups::handle_rollup_requests(self.fd, finish).map(Into::into)?)
    }

    fn throw_exception(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        let exception = Exception::try_from(payload)?;

        rollups::throw_exception(self.fd, &exception)
    }
}
