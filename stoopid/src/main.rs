#![feature(async_closure)]

use std::error::Error;
use cartesi_rollups_dapp::{Rollups, RollupsBuilder, RollupsMessage};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut rollups = RollupsBuilder::new()
        .set_server_url("http://127.0.0.1:5004")
        .build()
        .unwrap();

    let poop = RollupsBuilder::new()
        .set_server_url("http://127.0.0.1:5004")
        .build()
        .unwrap();
    let poop = &poop;

    rollups.run(
        async move |request: RollupsMessage| -> Result<bool, Box<dyn Error>> {
            poop.add_notice(request.payload.as_bytes()).await.map(|_| true)
        },
        async move |request: RollupsMessage| -> Result<bool, Box<dyn Error>> {
            poop.add_report(request.payload.as_bytes()).await.map(|_| true)
        }
    ).await.unwrap();
}
