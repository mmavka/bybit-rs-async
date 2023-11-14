use crate::general::rest_model::string_or_float;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoLinear<T> {
    pub category: String,
    pub list: Vec<T>,
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SymbolLinear {
    pub symbol: String,
    pub contract_type: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_scale: String,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    #[serde(with = "string_or_float")]
    pub min_leverage: f64,
    #[serde(with = "string_or_float")]
    pub max_leverage: f64,
    #[serde(with = "string_or_float")]
    pub leverage_step: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    #[serde(with = "string_or_float")]
    pub min_price: f64,
    #[serde(with = "string_or_float")]
    pub max_price: f64,
    #[serde(with = "string_or_float")]
    pub tick_size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    #[serde(with = "string_or_float")]
    pub max_order_qty: f64,
    #[serde(with = "string_or_float")]
    pub min_order_qty: f64,
    #[serde(with = "string_or_float")]
    pub qty_step: f64,
    #[serde(with = "string_or_float")]
    pub post_only_max_order_qty: f64,
}