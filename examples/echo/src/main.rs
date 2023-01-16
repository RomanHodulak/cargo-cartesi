use cartesi_rollups_linux::LinuxMachine;

fn main() {
    env_logger::init();

    let machine = LinuxMachine::open_default_device().unwrap();

    echo::run(machine).unwrap();
}
