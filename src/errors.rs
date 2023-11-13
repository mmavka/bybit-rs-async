use std::collections::HashMap;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
#[error("retCode: {ret_code}, retMsg: {ret_msg}")]
pub struct BybitContentError {
    pub ret_code: i32,
    pub ret_msg: String,
    pub time: i64,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Qs(#[from] serde_qs::Error),
    #[error(transparent)]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    #[error(transparent)]
    TimestampError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    UTF8Err(#[from] std::str::Utf8Error),
    #[error("{response}")]
    BinanceError {
        #[from]
        response: BybitContentError,
    },
    #[error("invalid listen key : {0}")]
    InvalidListenKey(String),
    #[error("unknown symbol {0}")]
    UnknownSymbol(String),
    #[error("{msg}")]
    InvalidOrderError { msg: String },
    #[error("invalid price")]
    InvalidPrice,
    #[error("invalid period {0}")]
    InvalidPeriod(String),
    #[error("internal server error")]
    InternalServerError,
    #[error("service unavailable")]
    ServiceUnavailable,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Msg(String),
}

/// Custom error messages
pub mod error_messages {
    pub const INVALID_PRICE: &str = "Invalid price.";
}

pub type Result<T> = core::result::Result<T, Error>;