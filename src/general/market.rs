use crate::API_V5_SERVER_TIME;
use crate::client::{Client};
use crate::errors::{Result};
use crate::general::rest_model::{ResponseBybit, ServerTime};

#[derive(Clone)]
pub struct General {
    pub client: Client,
    pub recv_window: u64,
}

// Market Data endpoints
impl General {
    pub async fn get_server_time(&self) -> Result<ResponseBybit<ServerTime>> {
        self.client.get(API_V5_SERVER_TIME, None).await
    }
}
