// #[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_qs as qs;

mod client;
pub mod errors;
pub mod config;
pub mod api;
pub mod util;
pub mod spot;
pub mod linear;
mod general;

static API_V5_SERVER_TIME: &str = "/v5/market/time";
static API_V5_KLINES: &str = "/v5/market/kline";
static API_V5_MARK_PRICE_KLINE: &str = "/v5/market/mark-price-kline";
static API_V5_INDEX_PRICE_KLINE: &str = "/v5/market/index-price-kline";
static API_V5_PREMIUM_INDEX_PRICE_KLINE: &str = "/v5/market/premium-index-price-kline";
static API_V5_INSTRUMENTS_INFO: &str = "/v5/market/instruments-info";
static API_V5_ORDERBOOK: &str = "/v5/market/orderbook";
static API_V5_TICKERS: &str = "/v5/market/tickers";
static API_V5_FUNDING_RATE_HISTORY: &str = "/v5/market/funding/history";
static API_V5_PUBLIC_RECENT_TRADING_HISTORY: &str = "/v5/market/recent-trade";
static API_V5_OPEN_INTEREST: &str = "/v5/market/open-interest";
static API_V5_HISTORICAL_VOLATILITY: &str = "/v5/market/historical-volatility";
static API_V5_INSURANCE: &str = "/v5/market/insurance";
static API_V5_RISK_LIMIT: &str = "/v5/market/risk-limit";
static API_V5_DELIVERY_PRICE: &str = "/v5/market/delivery-price";
static API_V5_LONG_SHORT_RATIO: &str = "/v5/market/account-ratio";

#[derive(Clone)]
pub enum Category {
    Spot,
    Linear,
    Inverse,
    Option
}
