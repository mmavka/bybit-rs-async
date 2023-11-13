use crate::client::Client;
use crate::errors::{Result};
use crate::rest_model::{ExchangeInformation, ServerTime};

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    pub async fn get_server_time(&self) -> Result<ServerTime> { self.client.get("/v5/market/time", None).await }

    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get("/api/v3/exchangeInfo", None).await
    }
}
