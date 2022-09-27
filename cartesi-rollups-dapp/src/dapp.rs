use std::env;
use hyper::{Client, Uri};
use cartesi_rollups_http::HttpRollups;
use thiserror::Error;

pub use cartesi_rollups::{Rollups, RollupsMessage};

#[derive(Debug, Error)]
pub enum RollupsBuilderError {
    #[error("Missing parameter {param:?}, define env var {env_var:?} or set it explicitly by calling {method_name:?}")]
    MissingParameter {
        param: String,
        env_var: String,
        method_name: String,
    }
}

impl RollupsBuilderError {
    pub fn missing_parameter(param: &str, env_var: &str, method_name: &str) -> Self {
        Self::MissingParameter {
            param: param.to_owned(),
            env_var: env_var.to_owned(),
            method_name: method_name.to_owned(),
        }
    }
}

pub struct RollupsBuilder {
    server_address: Option<Uri>,
}

impl RollupsBuilder {
    pub fn new() -> Self {
        Self {
            server_address: env::var("ROLLUP_HTTP_SERVER_URL")
                .ok()
                .and_then(|v| v.try_into().ok())
        }
    }

    pub fn set_server_url(mut self, server_url: impl TryInto<Uri>) -> Self {
        self.server_address = server_url.try_into().ok();
        self
    }

    pub fn build(self) -> Result<impl Rollups, RollupsBuilderError> {
        Ok(HttpRollups::new(
            Client::new(),
            self.server_address.ok_or(RollupsBuilderError::missing_parameter(
                "server_address",
                "ROLLUP_HTTP_SERVER_URL",
                "set_server_url"
            ))?
        ))
    }
}
