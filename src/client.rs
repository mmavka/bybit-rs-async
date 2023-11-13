use std::time::Duration;

use boolinator::Boolinator;
use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Response;
use reqwest::StatusCode;
use ring::hmac;
use serde::de;
use serde::de::DeserializeOwned;

use crate::errors::{BybitContentError, Error, error_messages, Result};

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    inner: reqwest::Client,
    host: String,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String, timeout: Option<u64>) -> Self {
        let mut builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        if let Some(timeout_secs) = timeout {
            builder = builder.timeout(Duration::from_secs(timeout_secs))
        }
        Client {
            // Does it ever make sense for api_key and secret_key to be ""?
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            inner: builder.build().unwrap(),
            host,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str, request: Option<&str>) -> Result<T> {
        let url = request
            .map(|r| format!("{}{}?{}", self.host, endpoint, r))
            .unwrap_or_else(|| format!("{}{}", self.host, endpoint));

        let response = self.inner.get(&url).send().await?;

        self.handler(response).await
    }

    async fn handler<T: de::DeserializeOwned>(&self, response: Response) -> Result<T> {
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::INTERNAL_SERVER_ERROR => Err(Error::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(Error::ServiceUnavailable),
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::BAD_REQUEST => {
                let error: BybitContentError = response.json().await?;
                Err(handle_content_error(error))
            }
            s => Err(Error::Msg(format!("Received response: {s:?}"))),
        }
    }
}

fn handle_content_error(error: BybitContentError) -> crate::errors::Error {
    match (error.ret_code, error.ret_msg.as_ref()) {
        (-1013, error_messages::INVALID_PRICE) => Error::InvalidPrice,
        (-1125, msg) => Error::InvalidListenKey(msg.to_string()),
        _ => Error::BinanceError { response: error },
    }
}