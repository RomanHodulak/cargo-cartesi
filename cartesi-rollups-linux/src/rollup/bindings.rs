use nix::ioctl_readwrite;
use nix::libc::{c_int, realloc, size_t};
use std::error::Error;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;

pub const CARTESI_ROLLUP_ADVANCE_STATE: u32 = 0;
pub const CARTESI_ROLLUP_INSPECT_STATE: u32 = 1;
pub const CARTESI_ROLLUP_ADDRESS_SIZE: usize = 20;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_finish {
    pub accept_previous_request: bool,
    pub next_request_type: ::std::os::raw::c_int,
    pub next_request_payload_length: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_bytes {
    pub data: *mut ::std::os::raw::c_uchar,
    pub length: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rollup_input_metadata {
    pub msg_sender: [u8; CARTESI_ROLLUP_ADDRESS_SIZE],
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
    pub address: [u8; CARTESI_ROLLUP_ADDRESS_SIZE],
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
    unsafe {
        let mut e = {
            let mut uninit = MaybeUninit::<rollup_finish>::zeroed();
            let data = uninit.as_mut_ptr();
            (*data).accept_previous_request = accept;
            uninit.assume_init()
        };

        finish_request(fd, &mut e)?;

        Ok(e.clone())
    }
}

pub fn rollup_read_advance_state_request(
    fd: c_int,
    finish: *mut rollup_finish,
    bytes: *mut rollup_bytes,
) -> Result<rollup_input_metadata, Box<dyn Error>> {
    unsafe {
        resize_bytes(bytes, (*finish).next_request_payload_length)?;

        let mut e = {
            let mut uninit = MaybeUninit::<rollup_advance_state>::zeroed();
            let data = uninit.as_mut_ptr();
            (*data).payload = *bytes;
            uninit.assume_init()
        };

        read_advance_state_request(fd, &mut e)?;

        Ok(e.metadata.clone())
    }
}

pub fn rollup_read_inspect_state_request(
    fd: c_int,
    finish: *mut rollup_finish,
    query: *mut rollup_bytes,
) -> Result<(), Box<dyn Error>> {
    unsafe {
        resize_bytes(query, (*finish).next_request_payload_length)?;

        let mut e = {
            let mut uninit = MaybeUninit::<rollup_inspect_state>::zeroed();
            let data = uninit.as_mut_ptr();
            (*data).payload = *query;
            uninit.assume_init()
        };

        read_inspect_state_request(fd, &mut e)?;

        Ok(())
    }
}

pub fn rollup_write_voucher(fd: c_int, address: [u8; CARTESI_ROLLUP_ADDRESS_SIZE], bytes: *mut rollup_bytes) -> Result<u64, Box<dyn Error>> {
    unsafe {
        let mut e = {
            let mut uninit = MaybeUninit::<rollup_voucher>::zeroed();
            let data = uninit.as_mut_ptr();
            (*data).payload = *bytes;
            addr_of_mut!((*data).address).write(address);
            uninit.assume_init()
        };

        write_voucher(fd, &mut e)?;

        Ok(e.index)
    }
}

pub fn rollup_write_notice(fd: c_int, bytes: *mut rollup_bytes) -> Result<u64, Box<dyn Error>> {
    unsafe {
        let mut e = {
            let mut uninit = MaybeUninit::<rollup_notice>::zeroed();
            let data = uninit.as_mut_ptr();
            (*data).payload = *bytes;
            uninit.assume_init()
        };

        write_notice(fd, &mut e)?;

        Ok(e.index)
    }
}

pub fn rollup_write_report(fd: c_int, bytes: *mut rollup_bytes) -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut e = {
            let mut uninit = MaybeUninit::<rollup_report>::zeroed();
            let data = uninit.as_mut_ptr();
            (*data).payload = *bytes;
            uninit.assume_init()
        };

        write_report(fd, &mut e)?;

        Ok(())
    }
}

pub fn rollup_throw_exception(fd: c_int, bytes: *mut rollup_bytes) -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut e = {
            let mut uninit = MaybeUninit::<rollup_exception>::zeroed();
            let data = uninit.as_mut_ptr();
            (*data).payload = *bytes;
            uninit.assume_init()
        };

        throw_exception(fd, &mut e)?;

        Ok(())
    }
}

unsafe fn resize_bytes(bytes: *mut rollup_bytes, size: i32) -> Result<(), Box<dyn Error>> {
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
