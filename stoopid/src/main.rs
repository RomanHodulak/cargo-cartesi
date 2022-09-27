use cartesi_rollups_dapp::{Rollups, RollupsBuilder};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut builder = RollupsBuilder::new();
    builder.set_server_url("http://127.0.0.1:5004");
    let mut rollups = builder.build().unwrap();

    rollups.run().await.unwrap();
}
