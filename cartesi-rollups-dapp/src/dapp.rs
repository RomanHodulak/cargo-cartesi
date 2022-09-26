use hyper::Client;
use cartesi_rollups::Rollups;
use cartesi_rollups_http::HttpRollups;

pub struct RollupsFactory;

impl RollupsFactory {
    pub fn create() -> impl Rollups {
        HttpRollups::new(Client::new())
    }
}
