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
        .merge(Serialized::defaults(Cli::parse()))
        .merge(Toml::file("Dapp.toml"))
        .merge(Env::prefixed("DAPP_"))
        .extract()
        .unwrap();

    cli.run()
}
