use crate::http::HttpRollups;

pub trait Rollups {

}

pub struct RollupsFactory;

impl RollupsFactory {
    pub fn create() -> impl Rollups {
        HttpRollups
    }
}
