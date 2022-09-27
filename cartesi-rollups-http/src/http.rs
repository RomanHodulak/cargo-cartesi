use std::error::Error;
use std::future::Future;
use async_trait::async_trait;
use cartesi_rollups::{RequestType, Rollups, RollupsError, RollupsMessage, RollupsRequest};
use hyper::{body::{HttpBody, to_bytes}, header, Client, Request, Method, Body, Uri, StatusCode};
use hyper::client::connect::Connect;
use serde_json::json;

#[derive(Debug)]
pub struct HttpRollups<C> {
    client: Client<C>,
    server_address: Uri,
    rollups_address: Option<String>,
}

impl<C: Connect + Clone + Send + Sync + 'static> HttpRollups<C> {
    pub fn new(client: Client<C>, server_address: impl Into<Uri>) -> Self {
        Self {
            client,
            server_address: server_address.into(),
            rollups_address: None,
        }
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

    async fn finish(&self, status: bool) -> Result<Option<RollupsRequest>, Box<dyn Error>> {
        log::debug!("Sending finish");

        let status = match status {
            true => "accept",
            false => "reject",
        };

        let response = json!({"status": status});
        let request = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("{}finish", self.server_address))
            .body(Body::from(response.to_string()))?;

        let response = self.client.request(request).await?;
        log::debug!("Received finish status {}", response.status());

        match response.status() {
            StatusCode::ACCEPTED => Ok(None),
            StatusCode::OK => Ok(Some(serde_json::from_slice(
                &to_bytes(response).await.unwrap()
            )?)),
            status_code => Err(format!("Unexpected status code {}", status_code))?,
        }
    }
}

#[async_trait]
impl<C: Connect + Clone + Send + Sync + 'static> Rollups for HttpRollups<C>
    where Self: Send
{
    async fn add_notice(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        log::debug!("Adding notice");

        let payload: &str = std::str::from_utf8(payload)?;
        let notice = json!({"payload": payload});
        let request = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("{}notice", self.server_address))
            .body(Body::from(notice.to_string()))?;

        let response = self.client.request(request).await?;
        Self::log_response(response).await?;

        Ok(())
    }

    async fn add_report(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        log::debug!("Adding report");

        let payload: &str = std::str::from_utf8(payload)?;
        let report = json!({"payload": payload});
        let request = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(format!("{}report", self.server_address))
            .body(Body::from(report.to_string()))?;

        let response = self.client.request(request).await?;
        Self::log_response(response).await?;

        Ok(())
    }

    async fn run<F1, F2, Fut1, Fut2>(
        &mut self,
        advance_state_handler: F1,
        inspect_state_handler: F2,
    ) -> Result<(), RollupsError>
        where
            F1: Fn(RollupsMessage) -> Fut1 + Send,
            F2: Fn(RollupsMessage) -> Fut2 + Send,
            Fut1: Future<Output = Result<bool, Box<dyn Error>>> + Send,
            Fut2: Future<Output = Result<bool, Box<dyn Error>>> + Send,
    {
        let mut status = true;

        loop {
            let request = self.finish(status).await.unwrap();

            match request {
                None => log::debug!("No pending rollup request, trying again"),
                Some(request) if request.data.metadata.epoch_index == 0 && request.data.metadata.input_index == 0 => {
                    log::debug!("Captured rollups address: {}", request.data.metadata.msg_sender);

                    self.rollups_address = Some(request.data.metadata.msg_sender);
                }
                Some(request) => {
                    log::debug!("Received rollups request {:?}", &request);

                    status = match request.request_type {
                        RequestType::AdvanceState => {
                            let fut = advance_state_handler(request.data);

                            fut.await.unwrap()
                        },
                        RequestType::InspectState => {
                            let fut = inspect_state_handler(request.data);

                            fut.await.unwrap()
                        },
                    };
                }
            }
        }
    }
}
