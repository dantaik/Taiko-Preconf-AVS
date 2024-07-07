use anyhow::Error;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use serde_json::Value;
use std::time::Duration;

pub struct RpcClient {
    client: HttpClient,
}

impl RpcClient {
    pub fn new(url: &str) -> Self {
        // let client = HttpClientBuilder::default().build(url).unwrap();

        let client = HttpClientBuilder::default()
            .request_timeout(Duration::from_secs(1))
            .build(url)
            .unwrap();
        RpcClient { client }
    }

    pub async fn call_method(&self, method: &str, params: Vec<Value>) -> Result<Value, Error> {
        self.client
            .request(method, params)
            .await
            .map_err(Error::from)
    }
}
