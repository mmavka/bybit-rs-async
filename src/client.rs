use std::time::Duration;

use boolinator::Boolinator;
use hex::encode as hex_encode;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use reqwest::Response;
use reqwest::StatusCode;
use ring::hmac;
use serde::de;
use serde::de::DeserializeOwned;

use crate::Category;
use crate::errors::{BybitContentError, Error, error_messages, Result};
use crate::util::{build_request_p, build_signed_request_p};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CategoryQuery {
    category: String,
}

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    inner: reqwest::Client,
    host: String,
    category: Option<Category>,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String, timeout: Option<u64>, category: Option<Category>) -> Self {
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
            category,
        }
    }

    pub async fn get_category<T: DeserializeOwned>(&self, endpoint: &str, request: Option<&str>) -> Result<T> {
        let url = self.category_query(endpoint, request);
        let response = self.inner.get(&url).headers(self.build_headers(true)?).send().await?;

        self.handler(response).await
    }

    pub async fn get_signed<T: DeserializeOwned>(&self, endpoint: &str, request: &str) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let response = self.inner.get(&url).headers(self.build_headers(true)?).send().await?;

        self.handler(response).await
    }

    pub async fn get_signed_d<T: de::DeserializeOwned>(&self, endpoint: &str, request: &str) -> Result<T> {
        self.get_signed(endpoint, request).await
    }

    pub async fn get_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: Option<P>,
        recv_window: u64,
    ) -> Result<T> {
        let req = build_signed_request_p(payload, recv_window)?;
        self.get_signed(endpoint, &req).await
    }

    pub async fn post_signed<T: DeserializeOwned>(&self, endpoint: &str, request: &str) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let response = self.inner.post(&url).headers(self.build_headers(true)?).send().await?;

        self.handler(response).await
    }

    pub async fn post_signed_d<T: de::DeserializeOwned>(&self, endpoint: &str, request: &str) -> Result<T> {
        self.post_signed(endpoint, request).await
    }

    pub async fn post_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: P,
        recv_window: u64,
    ) -> Result<T> {
        let request = build_signed_request_p(payload, recv_window)?;
        self.post_signed(endpoint, &request).await
    }

    pub async fn delete_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: P,
        recv_window: u64,
    ) -> Result<T> {
        let request = build_signed_request_p(payload, recv_window)?;
        self.delete_signed(endpoint, &request).await
    }

    pub async fn delete_signed<T: DeserializeOwned>(&self, endpoint: &str, request: &str) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let response = self
            .inner
            .delete(&url)
            .headers(self.build_headers(true)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str, request: Option<&str>) -> Result<T> {
        let url = request
            .map(|r| format!("{}{}?{}", self.host, endpoint, r))
            .unwrap_or_else(|| format!("{}{}", self.host, endpoint));

        let response = self.inner.get(&url).send().await?;

        self.handler(response).await
    }

    pub async fn get_p<T: DeserializeOwned>(&self, endpoint: &str, request: Option<&str>) -> Result<T> {
        self.get(endpoint, request).await
    }

    pub async fn get_d<T: DeserializeOwned, S: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: Option<S>,
    ) -> Result<T> {
        let req = if let Some(p) = payload {
            Some(build_request_p(p)?)
        } else {
            None
        };
        self.get_p(endpoint, req.as_deref()).await
    }

    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str, symbol: Option<&str>) -> Result<T> {
        let url = symbol
            .map(|s| format!("{}{}?symbol={}", self.host, endpoint, s))
            .unwrap_or_else(|| format!("{}{}", self.host, endpoint));

        let response = self.inner.post(url).headers(self.build_headers(false)?).send().await?;

        self.handler(response).await
    }

    pub async fn put<T: DeserializeOwned>(&self, endpoint: &str, listen_key: &str, symbol: Option<&str>) -> Result<T> {
        let data = symbol
            .map(|s| format!("listenKey={listen_key}&symbol={s}"))
            .unwrap_or_else(|| format!("listenKey={listen_key}"));
        let headers = self.build_headers(false)?;
        let url = format!("{}{}?{}", self.host, endpoint, data);
        let response = self.inner.put(&url).headers(headers).send().await?;

        self.handler(response).await
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        listen_key: &str,
        symbol: Option<&str>,
    ) -> Result<T> {
        let data = symbol
            .map(|s| format!("listenKey={listen_key}&symbol={s}"))
            .unwrap_or_else(|| format!("listenKey={listen_key}"));
        let url = format!("{}{}?{}", self.host, endpoint, data);
        let response = self
            .inner
            .delete(url)
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        self.handler(response).await
    }

    fn category_query(&self, endpoint: &str, request: Option<&str>) -> String {
        let url = match &self.category {
            Some(c) => {
                let cat = match c {
                    Category::Spot => String::from("spot"),
                    Category::Linear => String::from("linear"),
                    Category::Inverse => String::from("inverse"),
                    Category::Option => String::from("option"),
                };
                request
                    .map(|r| format!("{}{}?{}&category={}", self.host, endpoint, r, cat))
                    .unwrap_or_else(|| format!("{}{}?category={}", self.host, endpoint, cat))
            }
            None => {
                request
                    .map(|r| format!("{}{}?{}", self.host, endpoint, r))
                    .unwrap_or_else(|| format!("{}{}", self.host, endpoint))
            }
        };

        url
    }

    // fn category_query(&self, endpoint: &str, request: Option<&str>) -> String {
    //     let category = self.category.map(|c| {
    //         match c {
    //             Category::Linear => "linear",
    //             Category::Inverse => "inverse",
    //             Category::Option => "option",
    //             Category::Spot => "spot",
    //         }
    //     }).unwrap_or_else(|| "");
    //     let url = request
    //         .map(|r| format!("{}{}?{}&category={}", self.host, endpoint, r, category))
    //         .unwrap_or_else(|| format!("{}{}?category={}", self.host, endpoint, category));
    //
    //     url
    // }

    // Request must be signed
    fn sign_request(&self, endpoint: &str, request: &str) -> String {
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
        let signature = hex_encode(hmac::sign(&signed_key, request.as_bytes()).as_ref());
        let url = format!("{}{}?{}&signature={}", self.host, endpoint, request, signature);

        url
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let header = IntoIterator::into_iter([
            // Always include user agent
            Some((USER_AGENT, HeaderValue::from_static("bybit-rs-async"))),
            // Always include API key
            Some((
                HeaderName::from_static("x-mbx-apikey"),
                HeaderValue::from_str(&self.api_key)?,
            )),
            // Include content type if needed
            content_type.as_option().map(|_| {
                (
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/x-www-form-urlencoded"),
                )
            }),
        ])
            .flatten()
            .collect();

        Ok(header)
    }

    async fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
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