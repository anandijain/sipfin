extern crate serde;
extern crate serde_derive;
extern crate serde_json;

//https://query1.finance.yahoo.com/v7/finance/spark?symbols=%5EGSPC&range=1d
//https://query1.finance.yahoo.com/v7/finance/spark?symbols=BTCUSD%3DX&range=1d
//https://query1.finance.yahoo.com/v1/finance/screener/instrument/earnings/fields?lang=en-US&region=US&category=keystats%2Cfinancials
//https://finance.yahoo.com/_finance_doubledown/api/resource/finance.market-time?
//https://query1.finance.yahoo.com/v8/finance/chart/AAPL?region=US&range=1d
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub spark: Spark,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spark {
    pub result: Vec<Result>,
    pub error: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub symbol: String,
    pub response: Vec<Response>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub meta: Meta,
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub currency: String,
    pub symbol: String,
    pub exchange_name: String,
    pub instrument_type: String,
    pub first_trade_date: i64,
    pub regular_market_time: i64,
    pub gmtoffset: i64,
    pub timezone: String,
    pub exchange_timezone_name: String,
    pub regular_market_price: f64,
    pub chart_previous_close: f64,
    pub previous_close: f64,
    pub scale: i64,
    pub price_hint: i64,
    pub current_trading_period: CurrentTradingPeriod,
    pub trading_periods: Vec<Vec<TradingPeriod>>,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTradingPeriod {
    pub pre: Pre,
    pub regular: Regular,
    pub post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pre {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regular {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPeriod {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indicators {
    pub quote: Vec<Quote>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub high: Vec<Option<f64>>,
    pub close: Vec<Option<f64>>,
    pub open: Vec<Option<f64>>,
    pub volume: Vec<Option<i64>>,
    pub low: Vec<Option<f64>>,
}