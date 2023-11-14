use crate::Category;
use crate::client::Client;
use crate::config::Config;
use crate::general::market::General;
use crate::linear::market::MarketLinear;

pub trait Bybit: Sized {
    fn new(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self;
}

impl Bybit for General {
    fn new(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout, None),
            recv_window: config.recv_window,
        }
    }
}

impl Bybit for MarketLinear {
    fn new(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> MarketLinear {
        MarketLinear {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout, Option::from(Category::Linear)),
            recv_window: config.recv_window,
        }
    }
}