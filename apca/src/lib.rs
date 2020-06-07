mod keys;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MarketOrder {
    pub symbol: String,
    pub qty: u64,
    pub side: String, // buy, sell
    #[serde(rename = "type")]
    pub type_field: String, // market, limit, stop, stop_limit
    pub time_in_force: String, // day, gtc, opg, cls, ioc, fok
                      // TODO
                      // pub limit_price: Option<f64>, // if type == limit or stop_limit
                      // pub stop_price: Option<f64>, //if type == stop or stop_limit
                      // pub extended_hours: Option<bool>,
                      // todo take_profit and stop_loss arms
                      // pub order_class = Stiring, // simple, bracket, oco, oto
                      // pub : bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BracketOrder {
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
pub struct ApcaAsset {
    pub id: ::serde_json::Value,
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
    pub asset_class: Option<String>,
    pub asset_id: Option<String>,
    pub canceled_at: ::serde_json::Value,
    pub client_order_id: String,
    pub created_at: String,
    pub expired_at: ::serde_json::Value,
    pub extended_hours: bool,
    pub failed_at: ::serde_json::Value,
    pub filled_at: ::serde_json::Value,
    pub filled_avg_price: ::serde_json::Value,
    pub filled_qty: String,
    pub id: String,
    pub legs: ::serde_json::Value,
    pub limit_price: ::serde_json::Value,
    pub order_class: String,
    pub order_type: String,
    pub qty: String,
    pub replaced_at: ::serde_json::Value,
    pub replaced_by: ::serde_json::Value,
    pub replaces: ::serde_json::Value,
    pub side: String,
    pub status: String,
    pub stop_price: ::serde_json::Value,
    pub submitted_at: String,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApcaPosition {
    pub asset_id: String,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: ::serde_json::Value,
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Clock {
    pub timestamp: String,
    pub is_open: bool,
    pub next_open: String,
    pub next_close: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PortfolioHistory {
    pub timestamp: Vec<i64>,
    pub equity: Vec<f64>,
    pub profit_loss: Vec<f64>,
    pub profit_loss_pct: Vec<f64>,
    pub base_value: f64,
    pub timeframe: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
