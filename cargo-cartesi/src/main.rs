mod app;

use app::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
}
