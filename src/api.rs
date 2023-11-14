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

// impl Bybit for Account {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Account {
//         Account {
//             client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
//             recv_window: config.recv_window,
//         }
//     }
// }

// #[cfg(feature = "savings_api")]
// impl Bybit for crate::savings::Savings {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
//         Self {
//             client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
//             recv_window: config.recv_window,
//         }
//     }
// }

//
// impl Bybit for UserStream {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> UserStream {
//         UserStream {
//             client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
//             recv_window: config.recv_window,
//         }
//     }
// }
//
// #[cfg(feature = "futures_api")]
// impl Bybit for crate::futures::general::FuturesGeneral {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
//         Self {
//             client: Client::new(
//                 api_key,
//                 secret_key,
//                 config.futures_rest_api_endpoint.clone(),
//                 config.timeout,
//             ),
//         }
//     }
// }
//
// #[cfg(feature = "futures_api")]
// impl Bybit for crate::futures::market::FuturesMarket {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
//         Self {
//             client: Client::new(
//                 api_key,
//                 secret_key,
//                 config.futures_rest_api_endpoint.clone(),
//                 config.timeout,
//             ),
//             recv_window: config.recv_window,
//         }
//     }
// }
//
// #[cfg(feature = "futures_api")]
// impl Bybit for crate::futures::account::FuturesAccount {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
//         Self {
//             client: Client::new(
//                 api_key,
//                 secret_key,
//                 config.futures_rest_api_endpoint.clone(),
//                 config.timeout,
//             ),
//             recv_window: config.recv_window,
//         }
//     }
// }
//
// #[cfg(feature = "margin_api")]
// impl Bybit for crate::margin::Margin {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
//         Self {
//             client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
//             recv_window: config.recv_window,
//         }
//     }
// }
//
// #[cfg(feature = "wallet_api")]
// impl Bybit for crate::wallet::Wallet {
//     fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
//         Self {
//             client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
//             recv_window: config.recv_window,
//             binance_us_api: config.binance_us_api,
//         }
//     }
// }
