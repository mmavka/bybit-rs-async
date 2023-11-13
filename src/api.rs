use crate::Category;
use crate::client::Client;
use crate::config::Config;
use crate::market::Market;

pub trait Bybit: Sized {
    fn spot(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self;
}

impl Bybit for Market {
    fn spot(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout, Category::Spot),
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
