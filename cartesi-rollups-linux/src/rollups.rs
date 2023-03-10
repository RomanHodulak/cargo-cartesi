//! Implements Rust api to use Linux rollup device
use cartesi_rollups_bindings as bindings;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::ErrorKind;
use std::os::unix::prelude::RawFd;
use thiserror::Error;

pub use bindings::CARTESI_ROLLUP_ADDRESS_SIZE;
pub use bindings::CARTESI_ROLLUP_ADVANCE_STATE;
pub use bindings::CARTESI_ROLLUP_INSPECT_STATE;

/// Rollup device driver path
pub const ROLLUP_DEVICE_NAME: &str = "/dev/rollup";

#[derive(Debug, Default, Error)]
#[error("rollup error: {}", message)]
pub struct RollupError {
    message: String,
}

impl RollupError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Copy)]
pub struct RollupFinish {
    pub accept_previous_request: bool,
    pub next_request_type: i32,
    pub next_request_payload_length: i32,
}

impl From<RollupFinish> for bindings::rollup_finish {
    fn from(mut other: RollupFinish) -> Self {
        Self::from(&mut other)
    }
}

impl From<&mut RollupFinish> for bindings::rollup_finish {
    fn from(other: &mut RollupFinish) -> Self {
        bindings::rollup_finish {
            next_request_type: other.next_request_type,
            accept_previous_request: other.accept_previous_request,
            next_request_payload_length: other.next_request_payload_length,
        }
    }
}

impl From<bindings::rollup_finish> for RollupFinish {
    fn from(other: bindings::rollup_finish) -> Self {
        RollupFinish {
            next_request_type: other.next_request_type,
            accept_previous_request: other.accept_previous_request,
            next_request_payload_length: other.next_request_payload_length,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceMetadata {
    pub msg_sender: String,
    pub epoch_index: u64,
    pub input_index: u64,
    pub block_number: u64,
    pub timestamp: u64,
}

impl AdvanceMetadata {
    /// Returns the address with the `Ox` prefix stripped if present.
    pub fn msg_sender_without_prefix(&self) -> &str {
        match self.msg_sender.starts_with("0x") {
            true => &self.msg_sender[2..],
            false => &self.msg_sender,
        }
    }
}

impl From<bindings::rollup_input_metadata> for AdvanceMetadata {
    fn from(other: bindings::rollup_input_metadata) -> Self {
        let address = format!("0x{}", hex::encode(&other.msg_sender));

        Self {
            input_index: other.input_index,
            epoch_index: other.epoch_index,
            timestamp: other.timestamp,
            block_number: other.block_number,
            msg_sender: address,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceRequest {
    pub metadata: AdvanceMetadata,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectRequest {
    pub payload: String,
}

pub enum RollupRequest {
    Inspect(InspectRequest),
    Advance(AdvanceRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectReport {
    pub reports: Vec<Report>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notice {
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voucher {
    pub address: String,
    pub payload: String,
}

impl Voucher {
    /// Returns the address with the `Ox` prefix stripped if present.
    pub fn address_without_prefix(&self) -> &str {
        match self.address.starts_with("0x") {
            true => &self.address[2..],
            false => &self.address,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exception {
    pub payload: String,
}

pub fn finish_request(fd: RawFd, finish: &mut RollupFinish, accept: bool) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("writing rollup finish request, yielding");

    let finish_c = bindings::rollup_finish_request(fd as i32, accept)?;

    *finish = RollupFinish::from(finish_c);

    log::debug!("finish request written to rollup device: {:#?}", &finish);

    Ok(())
}

pub fn read_advance_state_request(
    fd: RawFd,
    finish: &mut RollupFinish,
) -> Result<AdvanceRequest, Box<dyn std::error::Error>> {
    let finish_c = bindings::rollup_finish::from(&mut *finish);
    let mut bytes_c = Box::new(bindings::rollup_bytes {
        data: std::ptr::null::<std::os::raw::c_uchar>() as *mut std::os::raw::c_uchar,
        length: 0,
    });

    let input_metadata_c = bindings::rollup_read_advance_state_request(fd as i32, &finish_c, bytes_c.as_mut())?;

    if bytes_c.length == 0 {
        log::info!("read zero size payload from advance state request");
    }

    let mut payload: Vec<u8> = Vec::with_capacity(bytes_c.length as usize);

    if bytes_c.length > 0 {
        unsafe {
            std::ptr::copy(bytes_c.data, payload.as_mut_ptr(), bytes_c.length as usize);
            payload.set_len(bytes_c.length as usize);
        }
    }

    let result = AdvanceRequest {
        metadata: AdvanceMetadata::from(input_metadata_c),
        payload: "0x".to_string() + &hex::encode(&payload),
    };
    *finish = RollupFinish::from(finish_c);

    Ok(result)
}

pub fn read_inspect_state_request(
    fd: RawFd,
    finish: &mut RollupFinish,
) -> Result<InspectRequest, Box<dyn std::error::Error>> {
    let finish_c = bindings::rollup_finish::from(&mut *finish);
    let mut bytes_c = Box::new(bindings::rollup_bytes {
        data: std::ptr::null::<std::os::raw::c_uchar>() as *mut std::os::raw::c_uchar,
        length: 0,
    });

    bindings::rollup_read_inspect_state_request(fd as i32, &finish_c, bytes_c.as_mut())?;

    let mut payload: Vec<u8> = Vec::with_capacity(bytes_c.length as usize);
    unsafe {
        std::ptr::copy(bytes_c.data, payload.as_mut_ptr(), bytes_c.length as usize);
        payload.set_len(bytes_c.length as usize);
    }
    let result = InspectRequest {
        payload: format!("0x{}", hex::encode(&payload)),
    };
    *finish = RollupFinish::from(finish_c);

    Ok(result)
}

pub fn write_notice(fd: RawFd, notice: &mut Notice) -> Result<u64, Box<dyn std::error::Error>> {
    log::debug!(
        "Notice: {{ length: {} payload: {} }}",
        notice.payload.len(),
        notice.payload
    );

    let binary_payload = hex::decode(&notice.payload[2..]).map_err(|_| {
        Box::new(RollupError::new(
            "Error decoding notice payload, payload must be in Ethereum hex binary format",
        ))
    })?;
    let mut buffer: Vec<u8> = Vec::with_capacity(binary_payload.len());
    let mut bytes_c = Box::new(bindings::rollup_bytes {
        data: buffer.as_mut_ptr() as *mut std::os::raw::c_uchar,
        length: binary_payload.len() as u64,
    });

    let notice_index = unsafe {
        std::ptr::copy(binary_payload.as_ptr(), buffer.as_mut_ptr(), binary_payload.len());
        bindings::rollup_write_notice(fd as i32, bytes_c.as_mut())
    }?;

    log::debug!("notice with id {} successfully written!", notice_index);

    Ok(notice_index)
}

pub fn write_voucher(fd: RawFd, voucher: &mut Voucher) -> Result<u64, Box<dyn std::error::Error>> {
    log::debug!(
        "voucher: {{ address: 0x{} length: {} payload: {} }}",
        voucher.address_without_prefix(),
        voucher.payload.len(),
        voucher.payload
    );

    let binary_payload = hex::decode(&voucher.payload[2..]).map_err(|_| {
        Box::new(RollupError::new(
            "Error decoding voucher payload, payload must be in Ethereum hex binary format",
        ))
    })?;
    let mut buffer: Vec<u8> = Vec::with_capacity(binary_payload.len());
    let mut bytes_c = Box::new(bindings::rollup_bytes {
        data: buffer.as_mut_ptr() as *mut std::os::raw::c_uchar,
        length: binary_payload.len() as u64,
    });
    let address_c = hex::decode(&voucher.address[2..])
        .map_err(|e| Box::new(RollupError::new(&format!("address not valid: {}", e))))?;

    let voucher_index = unsafe {
        std::ptr::copy(binary_payload.as_ptr(), buffer.as_mut_ptr(), binary_payload.len());
        bindings::rollup_write_voucher(fd as i32, address_c.try_into().unwrap(), bytes_c.as_mut())
    }?;

    log::debug!("voucher with id {} successfully written!", voucher_index);

    Ok(voucher_index)
}

pub fn write_report(fd: RawFd, report: &Report) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!(
        "report: {{ length: {} payload: {}}}",
        report.payload.len(),
        report.payload
    );

    let binary_payload = match hex::decode(&report.payload[2..]) {
        Ok(payload) => payload,
        Err(_err) => {
            return Err(Box::new(RollupError::new(&format!(
                "Error decoding report payload, payload must be in Ethereum hex binary format"
            ))));
        }
    };
    let mut buffer: Vec<u8> = Vec::with_capacity(binary_payload.len());
    let mut bytes_c = Box::new(bindings::rollup_bytes {
        data: buffer.as_mut_ptr() as *mut std::os::raw::c_uchar,
        length: binary_payload.len() as u64,
    });

    unsafe {
        std::ptr::copy(binary_payload.as_ptr(), buffer.as_mut_ptr(), binary_payload.len());
        bindings::rollup_write_report(fd as i32, bytes_c.as_mut())
    }?;

    log::debug!("report successfully written!");

    Ok(())
}

pub fn throw_exception(fd: RawFd, exception: &Exception) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!(
        "exception: {{ length: {} payload: {}}}",
        exception.payload.len(),
        exception.payload
    );

    let binary_payload = hex::decode(&exception.payload[2..]).map_err(|_| {
        Box::new(RollupError::new(
            "Error decoding report payload, payload must be in Ethereum hex binary format",
        ))
    })?;
    let mut buffer: Vec<u8> = Vec::with_capacity(binary_payload.len());
    let mut bytes_c = Box::new(bindings::rollup_bytes {
        data: buffer.as_mut_ptr() as *mut std::os::raw::c_uchar,
        length: binary_payload.len() as u64,
    });

    unsafe {
        std::ptr::copy(binary_payload.as_ptr(), buffer.as_mut_ptr(), binary_payload.len());
        bindings::rollup_throw_exception(fd as i32, bytes_c.as_mut())
    }?;

    log::debug!("exception successfully thrown!");

    Ok(())
}

pub fn perform_rollup_finish_request(fd: RawFd, accept: bool) -> io::Result<RollupFinish> {
    let mut finish = RollupFinish::default();

    finish_request(fd, &mut finish, accept).map(|_| finish).map_err(|e| {
        log::error!("error inserting finish request, details: {}", e.to_string());

        io::Error::new(ErrorKind::Other, e.to_string())
    })
}

/// Read advance/inspect request from rollup device
pub fn handle_rollup_requests(fd: RawFd, mut finish_request: RollupFinish) -> Result<RollupRequest, io::Error> {
    let next_request_type = finish_request.next_request_type as u32;

    match next_request_type {
        CARTESI_ROLLUP_ADVANCE_STATE => {
            log::debug!("handle advance state request...");

            // Read advance request from rollup device
            let advance_request = read_advance_state_request(fd, &mut finish_request)
                .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

            log::info!(
                "advance: {{ msg_sender: {} block_number: {} timestamp: {} epoch_index: {} input_index: {} }}",
                advance_request.metadata.msg_sender_without_prefix(),
                advance_request.metadata.block_number,
                advance_request.metadata.timestamp,
                advance_request.metadata.epoch_index,
                advance_request.metadata.input_index
            );

            // Send newly read advance request to http service
            Ok(RollupRequest::Advance(advance_request))
        }
        CARTESI_ROLLUP_INSPECT_STATE => {
            log::debug!("handle inspect state request...");

            // Read inspect request from rollup device
            let inspect_request = read_inspect_state_request(fd, &mut finish_request)
                .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

            log::info!(
                "inspect: {{ length: {} payload: {}}}",
                inspect_request.payload.len(),
                inspect_request.payload
            );

            // Send newly read inspect request to http service
            Ok(RollupRequest::Inspect(inspect_request))
        }
        _ => Err(io::Error::new(ErrorKind::Unsupported, "request type unsupported")),
    }
}
