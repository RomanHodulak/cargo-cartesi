mod cli;
mod commands;
pub(crate) mod services;

use clap::Parser;
use cli::Cli;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli: Cli = Figment::new()
        .merge(Toml::file("CargoCartesi.toml"))
        .merge(Env::prefixed("CARGO_CARTESI_"))
        .merge(Serialized::defaults(Cli::parse()))
        .extract()
        .unwrap();

    cli.run()
}
