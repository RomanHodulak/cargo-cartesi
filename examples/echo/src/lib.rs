use cartesi_rollups_linux::{MachineIo, RollupsRequest};
use std::error::Error;

pub fn run(machine: impl MachineIo) -> Result<(), Box<dyn Error>> {
    loop {
        let request = machine.submit()?;

        match request {
            RollupsRequest::AdvanceState { payload, .. } => {
                machine.write_notice(payload.as_slice())?;
            }
            RollupsRequest::InspectState { payload } => {
                machine.write_report(payload.as_slice())?;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cartesi_rollups_linux::RollupsMetadata;
    use cartesi_rollups_test_doubles::{Data, FakeCartesiMachine};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_advancing_state_echoes_notice() {
        let request = RollupsRequest::AdvanceState {
            metadata: RollupsMetadata {
                msg_sender: "test".to_owned(),
                epoch_index: 0,
                input_index: 0,
                block_number: 0,
                timestamp: 0,
            },
            payload: vec![1, 2, 3],
        };

        let actual_data = Rc::new(RefCell::new(Data::default()));
        let machine = FakeCartesiMachine::new([request], actual_data.clone());

        run(machine).unwrap_err();

        let expected_data = Rc::new(RefCell::new(Data {
            notices: vec![vec![1, 2, 3]],
            vouchers: vec![],
            reports: vec![],
            exceptions: vec![],
        }));

        assert_eq!(expected_data, actual_data);
    }

    #[test]
    fn test_inspecting_state_echoes_report() {
        let request = RollupsRequest::InspectState { payload: vec![1, 2, 3] };

        let actual_data = Rc::new(RefCell::new(Data::default()));
        let machine = FakeCartesiMachine::new([request], actual_data.clone());

        run(machine).unwrap_err();

        let expected_data = Rc::new(RefCell::new(Data {
            notices: vec![],
            vouchers: vec![],
            reports: vec![vec![1, 2, 3]],
            exceptions: vec![],
        }));

        assert_eq!(expected_data, actual_data);
    }
}
