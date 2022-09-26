use std::error::Error;
use async_trait::async_trait;
use cartesi_rollups::{Rollups, RollupsError};
use hyper::{
    body::{HttpBody, to_bytes},
    client::HttpConnector,
    header,
    Client, Request, Method, Body, Uri
};
use serde_json::{Value as JsonValue, Value};

#[derive(Debug)]
pub struct HttpRollups {
    client: Client<HttpConnector>,
    server_address: Uri,
}

impl HttpRollups {
    pub fn new(client: Client<HttpConnector>, server_address: impl Into<Uri>) -> Self {
        Self {
            client,
            server_address: server_address.into(),
        }
    }

    async fn handle_advance(&self, request: JsonValue) -> Result<(), Box<dyn Error>> {
        log::debug!("Received advance request data {}", &request);

        let payload = request["data"]["payload"]
            .as_str()
            .ok_or("Missing payload")?;

        log::debug!("Adding notice");

        let notice = object! {"payload" => format!("{}", payload)};
        let req = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/notice", self.server_address))
            .body(Body::from(notice.dump()))?;

        let response = self.client.request(req).await?;
        Self::log_response(response).await?;

        Ok(())
    }

    async fn handle_inspect(&self, request: JsonValue) -> Result<(), Box<dyn Error>> {
        log::debug!("Received inspect request data {}", &request);

        let payload = request["data"]["payload"]
            .as_str()
            .ok_or("Missing payload")?;

        log::debug!("Adding report");

        let report = object! {"payload" => format!("{}", payload)};
        let req = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/report", self.server_address))
            .body(Body::from(report.dump()))?;

        let response = self.client.request(req).await?;
        Self::log_response(response).await?;

        Ok(())
    }

    async fn handle_finish(&self) -> Result<(), Box<dyn Error>> {
        log::debug!("Sending finish");

        let response = object! {"status" => "accept"};
        let req = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/finish", self.server_address))
            .body(Body::from(response.dump()))?;

        let response = self.client.request(req).await?;
        Self::log_response(response).await?;

        Ok(())
    }

    async fn log_response<T: HttpBody>(response: hyper::Response<T>) -> Result<(), Box<dyn Error>>
        where
            <T as HttpBody>::Error: 'static,
            <T as HttpBody>::Error: Error,
    {
        let response_status = response.status().as_u16();
        let response_body = to_bytes(response).await?;
        let response_body = std::str::from_utf8(&response_body)?;

        log::debug!("Response HTTP code {} body {}", response_status, response_body);

        Ok(())
    }
}

#[async_trait]
impl Rollups for HttpRollups {
    async fn add_notice(&mut self, payload: &[u8]) -> Result<(), RollupsError> {
        Ok(self.handle_advance(Value::from(payload)).unwrap())
    }

    async fn finish(&mut self) -> Result<(), RollupsError> {
        Ok(self.handle_finish().unwrap())
    }
}
