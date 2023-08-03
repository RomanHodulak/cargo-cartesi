use crate::rollups::{Exception, Notice, Report, RollupRequest, Voucher};
use cartesi_rollups::{RollupsMetadata, RollupsRequest};
use std::string::FromUtf8Error;

impl TryFrom<&[u8]> for Notice {
    type Error = FromUtf8Error;
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        String::from_utf8(payload.to_vec()).map(|payload| Self { payload })
    }
}

impl TryFrom<(&[u8; 20], &[u8])> for Voucher {
    type Error = FromUtf8Error;
    fn try_from(voucher: (&[u8; 20], &[u8])) -> Result<Self, Self::Error> {
        let destination = String::from_utf8(voucher.0.to_vec())?;
        let payload = String::from_utf8(voucher.1.to_vec())?;

        Ok(Self { destination, payload })
    }
}

impl TryFrom<&[u8]> for Report {
    type Error = FromUtf8Error;
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        String::from_utf8(payload.to_vec()).map(|payload| Self { payload })
    }
}

impl TryFrom<&[u8]> for Exception {
    type Error = FromUtf8Error;
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        String::from_utf8(payload.to_vec()).map(|payload| Self { payload })
    }
}

impl From<RollupRequest> for RollupsRequest {
    fn from(request: RollupRequest) -> Self {
        match request {
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
        }
    }
}
