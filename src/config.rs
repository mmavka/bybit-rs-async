
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub recv_window: u64,

    pub timeout: Option<u64>,
}

impl Config {
    pub fn testnet() -> Config {
        Config::default()
            .set_rest_api_endpoint("https://api-testnet.bybit.com")
            .set_ws_endpoint("wss://stream-testnet.bybit.com")
    }

    pub fn set_rest_api_endpoint<T: Into<String>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_ws_endpoint<T: Into<String>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }

    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub fn set_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

impl Default for Config {
    /// Configure binance with default production endpoints
    /// # Examples
    /// ```
    /// use bybit_rs_async::config::Config;
    /// let config = Config::default();
    /// ```
    fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.bybit.com".into(),
            ws_endpoint: "wss://stream.bybit.com".into(),

            recv_window: 5000,

            timeout: None,
        }
    }
}