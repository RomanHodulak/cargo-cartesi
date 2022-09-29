use cartesi_rollups_linux::{LinuxMachine, MachineIo, RollupsRequest};

fn main() {
    env_logger::init();

    let machine = LinuxMachine::open_default_device().unwrap();

    run(machine);
}

fn run(machine: impl MachineIo) {
    loop {
        let request = machine.submit().unwrap();

        match request {
            RollupsRequest::AdvanceState { payload, .. } => {
                machine.write_notice(payload.as_slice()).unwrap();
            }
            RollupsRequest::InspectState { payload } => {
                machine.write_report(payload.as_slice()).unwrap();
            }
        }
    }
}
