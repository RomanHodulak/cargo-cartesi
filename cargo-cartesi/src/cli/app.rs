use crate::cli::{BuildCommand, CreateFsCommand, CreateMachineCommand, NewCommand, RunCommand};
use crate::services::{
    CartesiMachine, DockerCartesiMachine, HostCargo, HostCartesiMachine, HostDependencyDownloader, HostFileSystem,
    HostResourceCreator,
};
use clap::clap_derive::ArgEnum;
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum Commands {
    New(NewCommand),
    Build(BuildCommand),
    CreateFs(CreateFsCommand),
    CreateMachine(CreateMachineCommand),
    Run(RunCommand),
}

impl Commands {
    pub fn execute<CM: CartesiMachine>(self, services: impl ServiceFactory<CM>) -> ExitCode {
        match self {
            Commands::New(cmd) => cmd
                .handle(
                    services.create_cargo(),
                    services.create_dependencies_downloader(),
                    services.create_resource_creator(),
                )
                .expect("failed new"),
            Commands::Build(cmd) => cmd.handle(services.create_cargo()).expect("failed build"),
            Commands::CreateFs(cmd) => cmd
                .handle(services.create_cargo(), services.create_file_system())
                .expect("failed create-fs"),
            Commands::CreateMachine(cmd) => cmd
                .handle(
                    services.create_cargo(),
                    services.create_file_system(),
                    services.create_cartesi_machine(),
                )
                .expect("failed create-machine"),
            Commands::Run(cmd) => cmd
                .handle(
                    services.create_cargo(),
                    services.create_file_system(),
                    services.create_cartesi_machine(),
                )
                .expect("failed run"),
        }

        ExitCode::SUCCESS
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, arg_enum, default_value = "host")]
    pub executor: Executor,
    #[clap(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let services = AppServiceFactory;

        match self.executor {
            Executor::Host => self.command.execute::<HostCartesiMachine>(services),
            Executor::Docker => self.command.execute::<DockerCartesiMachine>(services),
        }
    }
}

#[derive(Debug, Clone, ArgEnum)]
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
