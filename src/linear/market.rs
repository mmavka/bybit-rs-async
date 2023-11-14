use crate::{API_V5_INSTRUMENTS_INFO, API_V5_SERVER_TIME};
use crate::client::*;
use crate::errors::*;
use crate::general::rest_model::{ResponseBybit, ServerTime};
use crate::linear::rest_model::{InstrumentsInfoLinear, SymbolLinear};
use crate::util::*;

#[derive(Clone)]
pub struct MarketLinear {
    pub client: Client,
    pub recv_window: u64,
}

// Market Data endpoints
impl MarketLinear {
    pub async fn get_instruments_info(&self) -> Result<ResponseBybit<InstrumentsInfoLinear<SymbolLinear>>> {
        self.client.get_category(API_V5_INSTRUMENTS_INFO, None).await
    }

    fn symbol_request<S>(&self, symbol: S) -> String
        where
            S: AsRef<str>,
    {
        build_request([("symbol", symbol)])
    }
}
