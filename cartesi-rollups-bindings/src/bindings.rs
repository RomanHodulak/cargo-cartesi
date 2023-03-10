//! Items in this module communicate with the rollup device using IOCTL functions.
//!
//! All the IOCTL wiring is implemented in this crate and exposed as an abstraction over the native functions.
use nix::ioctl_readwrite;
use nix::libc::{c_int, realloc, size_t};
use std::error::Error;
use std::os::raw::c_void;

pub const CARTESI_ROLLUP_ADVANCE_STATE: u32 = 0;
pub const CARTESI_ROLLUP_INSPECT_STATE: u32 = 1;
pub const CARTESI_ROLLUP_ADDRESS_SIZE: usize = 20;

#[allow(non_camel_case_types)]
pub type rollup_address = [u8; CARTESI_ROLLUP_ADDRESS_SIZE];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_finish {
    pub accept_previous_request: bool,
    pub next_request_type: std::os::raw::c_int,
    pub next_request_payload_length: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_bytes {
    pub data: *mut std::os::raw::c_uchar,
    pub length: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_input_metadata {
    pub msg_sender: rollup_address,
    pub block_number: u64,
    pub timestamp: u64,
    pub epoch_index: u64,
    pub input_index: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_advance_state {
    pub metadata: rollup_input_metadata,
    pub payload: rollup_bytes,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_inspect_state {
    pub payload: rollup_bytes,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_voucher {
    pub address: rollup_address,
    pub payload: rollup_bytes,
    pub index: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_notice {
    pub payload: rollup_bytes,
    pub index: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_report {
    pub payload: rollup_bytes,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_exception {
    pub payload: rollup_bytes,
}

ioctl_readwrite!(finish_request, 0xd3, 0, rollup_finish);
ioctl_readwrite!(read_advance_state_request, 0xd3, 0, rollup_advance_state);
ioctl_readwrite!(read_inspect_state_request, 0xd3, 0, rollup_inspect_state);
ioctl_readwrite!(write_voucher, 0xd3, 1, rollup_voucher);
ioctl_readwrite!(write_notice, 0xd3, 2, rollup_notice);
ioctl_readwrite!(write_report, 0xd3, 3, rollup_report);
ioctl_readwrite!(throw_exception, 0xd3, 4, rollup_exception);

pub fn rollup_finish_request(fd: c_int, accept: bool) -> Result<rollup_finish, Box<dyn Error>> {
    let mut data = rollup_finish {
        accept_previous_request: accept,
        next_request_type: 0,
        next_request_payload_length: 0,
    };

    unsafe {
        finish_request(fd, &mut data)?;
    }

    Ok(data)
}

pub fn rollup_read_advance_state_request(
    fd: c_int,
    finish: &rollup_finish,
    payload: &mut Box<rollup_bytes>,
) -> Result<rollup_input_metadata, Box<dyn Error>> {
    let metadata = rollup_input_metadata {
        msg_sender: [0; CARTESI_ROLLUP_ADDRESS_SIZE],
        block_number: 0,
        timestamp: 0,
        epoch_index: 0,
        input_index: 0,
    };

    unsafe {
        resize_bytes(payload.as_mut(), finish.next_request_payload_length)?;

        let mut data = rollup_advance_state {
            metadata,
            payload: *payload.as_mut(),
        };

        read_advance_state_request(fd, &mut data)?;
    }

    Ok(metadata)
}

pub fn rollup_read_inspect_state_request(
    fd: c_int,
    finish: &rollup_finish,
    payload: &mut Box<rollup_bytes>,
) -> Result<(), Box<dyn Error>> {
    unsafe {
        resize_bytes(payload.as_mut(), finish.next_request_payload_length)?;

        let mut data = rollup_inspect_state {
            payload: *payload.as_mut(),
        };

        read_inspect_state_request(fd, &mut data)?;
    }

    Ok(())
}

pub fn rollup_write_voucher(
    fd: c_int,
    address: rollup_address,
    payload: &mut Box<rollup_bytes>,
) -> Result<u64, Box<dyn Error>> {
    unsafe {
        let mut data = rollup_voucher {
            address,
            payload: *payload.as_mut(),
            index: 0,
        };

        write_voucher(fd, &mut data)?;

        Ok(data.index)
    }
}

pub fn rollup_write_notice(fd: c_int, payload: &mut Box<rollup_bytes>) -> Result<u64, Box<dyn Error>> {
    unsafe {
        let mut data = rollup_notice {
            payload: *payload.as_mut(),
            index: 0,
        };

        write_notice(fd, &mut data)?;

        Ok(data.index)
    }
}

pub fn rollup_write_report(fd: c_int, payload: &mut Box<rollup_bytes>) -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut data = rollup_report {
            payload: *payload.as_mut(),
        };

        write_report(fd, &mut data)?;
    }

    Ok(())
}

pub fn rollup_throw_exception(fd: c_int, payload: &mut Box<rollup_bytes>) -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut data = rollup_exception {
            payload: *payload.as_mut(),
        };

        throw_exception(fd, &mut data)?;
    }

    Ok(())
}

unsafe fn resize_bytes(bytes: *mut rollup_bytes, size: c_int) -> Result<(), Box<dyn Error>> {
    if (*bytes).length >= size.try_into()? {
        return Ok(());
    }

    let new_data = realloc((*bytes).data as *mut c_void, size as size_t) as *mut u8;

    if new_data.is_null() {
        return Err("Failed growing payload buffer".into());
    }

    (*bytes).length = size.try_into()?;
    (*bytes).data = new_data;

    Ok(())
}
