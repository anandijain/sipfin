extern crate reqwest;
mod keys;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

//use std::collections::HashMap;

pub fn get_headermap() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let key_id = HeaderName::from_lowercase(b"apca-api-key-id").unwrap();
    let sec_key = HeaderName::from_lowercase(b"apca-api-secret-key").unwrap();
    headers.insert(
        key_id,
        HeaderValue::from_str(keys::APCA_API_KEY_ID).unwrap(),
    );
    headers.insert(
        sec_key,
        HeaderValue::from_str(keys::APCA_API_SECRET_KEY).unwrap(),
    );

    headers
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let req_ep = "account";
    let post_ep = "orders";
    let pos_ep = "positions";
    let request_url = format!("https://paper-api.alpaca.markets/v2/{}", req_ep);
    let post_url = format!("https://paper-api.alpaca.markets/v2/{}", post_ep);
    let positions_url = format!("https://paper-api.alpaca.markets/v2/{}", pos_ep);

    let headers = get_headermap();
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client
        .get(&request_url)
        .send()
        .await?
        //.json::<Vec<ApcaAsset>>()
        .json::<ApcaAccount>()
        .await?;

    println!("{:#?}", res);

    let order = ApcaPostOrder {
        symbol: "AAPL".to_string(),
        qty: 1,
        side: "buy".to_string(),
        type_field: "market".to_string(),
        time_in_force: "day".to_string(),
        //limit_price: None,
        //stop_price: None,
        //extended_hours: None,
    };

    println!("order struct{:#?}", order);

    let order_res = client
        .post(&post_url)
        .json(&order)
        .send()
        .await?
        .json::<ApcaOrderTmp>()
        .await?;

    println!("{:#?}", order_res);
    let positions = client
        .get(&positions_url)
        .send()
        .await?
        .json::<Vec<ApcaPosition>>()
        .await?;

    println!("{:#?}", positions);

    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FixRoot {
    pub side: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub qty: String,
    pub time_in_force: String,
    pub order_class: String,
    pub take_profit: TakeProfit,
    pub stop_loss: StopLoss,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TakeProfit {
    pub limit_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct StopLoss {
    pub stop_price: String,
    pub limit_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApcaPostOrder {
    pub symbol: String,
    pub qty: u64,
    pub side: String, // buy, sell
    #[serde(rename = "type")]
    pub type_field: String, // market, limit, stop, stop_limit
    pub time_in_force: String, // day, gtc, opg, cls, ioc, fok
                      //pub limit_price: Option<f64>, // if type == limit or stop_limit
                      //pub stop_price: Option<f64>, //if type == stop or stop_limit
                      //pub extended_hours: Option<bool>,
                      // todo take_profit and stop_loss arms
                      // pub order_class = Stiring, // simple, bracket, oco, oto
                      //pub : bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApcaAsset {
    pub id: String,
    pub class: String,
    pub exchange: String,
    pub symbol: String,
    pub status: String,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    pub easy_to_borrow: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApcaAccount {
    pub account_blocked: bool,
    pub account_number: String,
    pub buying_power: String,
    pub cash: String,
    pub created_at: String,
    pub currency: String,
    pub daytrade_count: i64,
    pub daytrading_buying_power: String,
    pub equity: String,
    pub id: String,
    pub initial_margin: String,
    pub last_equity: String,
    pub last_maintenance_margin: String,
    pub long_market_value: String,
    pub maintenance_margin: String,
    pub multiplier: String,
    pub pattern_day_trader: bool,
    pub portfolio_value: String,
    pub regt_buying_power: String,
    pub short_market_value: String,
    pub shorting_enabled: bool,
    pub sma: String,
    pub status: String,
    pub trade_suspended_by_user: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApcaOrder {
    pub id: String,
    pub client_order_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub submitted_at: String,
    pub filled_at: String,
    pub expired_at: String,
    pub canceled_at: String,
    pub failed_at: String,
    pub replaced_at: String,
    pub replaced_by: String,
    pub replaces: ::serde_json::Value,
    pub asset_id: String,
    pub symbol: String,
    pub asset_class: String,
    pub qty: String,
    pub filled_qty: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub side: String,
    pub time_in_force: String,
    pub limit_price: String,
    pub stop_price: String,
    pub filled_avg_price: String,
    pub status: String,
    pub extended_hours: bool,
    pub legs: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApcaOrderTmp {
    pub id: String,
    pub client_order_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub submitted_at: String,
    pub filled_at: ::serde_json::Value,
    pub expired_at: ::serde_json::Value,
    pub canceled_at: ::serde_json::Value,
    pub failed_at: ::serde_json::Value,
    pub replaced_at: ::serde_json::Value,
    pub replaced_by: ::serde_json::Value,
    pub replaces: ::serde_json::Value,
    pub asset_id: String,
    pub symbol: String,
    pub asset_class: String,
    pub qty: String,
    pub filled_qty: String,
    pub filled_avg_price: ::serde_json::Value,
    pub order_class: String,
    pub order_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub side: String,
    pub time_in_force: String,
    pub limit_price: ::serde_json::Value,
    pub stop_price: ::serde_json::Value,
    pub status: String,
    pub extended_hours: bool,
    pub legs: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApcaPosition {
    pub asset_id: String,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub avg_entry_price: String,
    pub qty: String,
    pub side: String,
    pub market_value: String,
    pub cost_basis: String,
    pub unrealized_pl: String,
    pub unrealized_plpc: String,
    pub unrealized_intraday_pl: String,
    pub unrealized_intraday_plpc: String,
    pub current_price: String,
    pub lastday_price: String,
    pub change_today: String,
}
