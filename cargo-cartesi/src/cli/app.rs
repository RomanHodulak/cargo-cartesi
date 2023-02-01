use crate::cli::{BuildCommand, CreateFsCommand, CreateMachineCommand, NewCommand, RunCommand};
use crate::services::{
    CartesiMachine, DockerCartesiMachine, HostCargo, HostCartesiMachine, HostDependencyDownloader, HostFileSystem,
    HostResourceCreator,
};
use clap::clap_derive::ValueEnum;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::process::ExitCode;

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum Command {
    New(NewCommand),
    Build(BuildCommand),
    CreateFs(CreateFsCommand),
    CreateMachine(CreateMachineCommand),
    Run(RunCommand),
}

impl Command {
    pub fn execute<CM: CartesiMachine>(self, services: impl ServiceFactory<CM>) -> ExitCode {
        match self {
            Command::New(cmd) => cmd
                .handle(services.create_cargo(), services.create_resource_creator())
                .expect("failed new"),
            Command::Build(cmd) => cmd.handle(services.create_cargo()).expect("failed build"),
            Command::CreateFs(cmd) => cmd
                .handle(services.create_cargo(), services.create_file_system())
                .expect("failed create-fs"),
            Command::CreateMachine(cmd) => cmd
                .handle(
                    services.create_cargo(),
                    services.create_file_system(),
                    services.create_dependencies_downloader(),
                    services.create_cartesi_machine(),
                )
                .expect("failed create-machine"),
            Command::Run(cmd) => cmd
                .handle(
                    services.create_cargo(),
                    services.create_file_system(),
                    services.create_dependencies_downloader(),
                    services.create_cartesi_machine(),
                )
                .expect("failed run"),
        }

        ExitCode::SUCCESS
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(
    bin_name = "cargo",
    version = clap::crate_version!(),
)]
pub struct Cli {
    #[clap(subcommand)]
    pub cartesi: CargoInvocation,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub enum CargoInvocation {
    Cartesi {
        #[clap(short, long, value_enum, default_value = "host")]
        executor: Executor,
        #[clap(subcommand)]
        command: Command,
    },
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let services = AppServiceFactory;

        match self.cartesi {
            CargoInvocation::Cartesi { command, executor } => match executor {
                Executor::Host => command.execute::<HostCartesiMachine>(services),
                Executor::Docker => command.execute::<DockerCartesiMachine>(services),
            },
        }
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum Executor {
    Host,
    Docker,
}

pub trait ServiceFactory<CM: CartesiMachine> {
    fn create_cartesi_machine(&self) -> CM;
    fn create_cargo(&self) -> HostCargo;
    fn create_file_system(&self) -> HostFileSystem;
    fn create_dependencies_downloader(&self) -> HostDependencyDownloader;
    fn create_resource_creator(&self) -> HostResourceCreator;
}

pub struct AppServiceFactory;

impl ServiceFactory<DockerCartesiMachine> for AppServiceFactory {
    fn create_cartesi_machine(&self) -> DockerCartesiMachine {
        DockerCartesiMachine
    }

    fn create_cargo(&self) -> HostCargo {
        HostCargo
    }

    fn create_file_system(&self) -> HostFileSystem {
        HostFileSystem
    }

    fn create_dependencies_downloader(&self) -> HostDependencyDownloader {
        HostDependencyDownloader
    }

    fn create_resource_creator(&self) -> HostResourceCreator {
        HostResourceCreator
    }
}

impl ServiceFactory<HostCartesiMachine> for AppServiceFactory {
    fn create_cartesi_machine(&self) -> HostCartesiMachine {
        HostCartesiMachine
    }

    fn create_cargo(&self) -> HostCargo {
        HostCargo
    }

    fn create_file_system(&self) -> HostFileSystem {
        HostFileSystem
    }

    fn create_dependencies_downloader(&self) -> HostDependencyDownloader {
        HostDependencyDownloader
    }

    fn create_resource_creator(&self) -> HostResourceCreator {
        HostResourceCreator
    }
}
