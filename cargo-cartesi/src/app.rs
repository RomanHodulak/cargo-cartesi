use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Builds release target binary for Cartesi machine.
    Build {
        #[clap(value_parser)]
        target_bin: String,
    },
    /// Builds release target binary for Cartesi machine and generates file system of the appropriate size.
    CreateFs {
        #[clap(value_parser)]
        target_bin: String,
    },
    /// Creates Cartesi machine from target.
    CreateMachine {
        #[clap(value_parser)]
        target_bin: String,
    },
    /// Builds and runs release target binary for Cartesi machine.
    Run {
        #[clap(value_parser)]
        target_bin: String,
    },
    /// Runs rollups integration tests. Functions marked with a specific macro will be considered test scripts.
    /// These functions produce inputs, which will be sent synchronously to <target-bin> dapp inside a Cartesi machine.
    /// The test scripts can assert the machine's state after each advance input.
    RollupsTest {
        #[clap(value_parser)]
        target_bin: String,
    },
    /// Runs Cargo unit tests inside Cartesi machine and builds unit-test binary.
    UnitTest,
}
