extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// use crate::utils;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Daum>,
    #[serde(rename = "error_code")]
    pub error_code: i64,
    #[serde(rename = "error_description")]
    pub error_description: ::serde_json::Value,
}

impl Root {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for t in self.data.iter() {
            // println!("{:#?}", t);
            recs.push(Daum::to_record(t));
        }
        return recs;
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub symbol: String,
    pub current: f64,
    pub percent: f64,
    pub chg: f64,
    pub timestamp: i64,
    pub volume: i64,
    pub amount: f64,
    #[serde(rename = "market_capital")]
    pub market_capital: f64,
    #[serde(rename = "float_market_capital")]
    pub float_market_capital: ::serde_json::Value,
    #[serde(rename = "turnover_rate")]
    pub turnover_rate: f64,
    pub amplitude: f64,
    pub open: f64,
    #[serde(rename = "last_close")]
    pub last_close: f64,
    pub high: f64,
    pub low: f64,
    #[serde(rename = "avg_price")]
    pub avg_price: f64,
    #[serde(rename = "trade_volume")]
    pub trade_volume: i64,
    pub side: i64,
    #[serde(rename = "is_trade")]
    pub is_trade: bool,
    pub level: i64,
    #[serde(rename = "trade_session")]
    pub trade_session: i64,
    #[serde(rename = "trade_type")]
    pub trade_type: ::serde_json::Value,
    #[serde(rename = "current_year_percent")]
    pub current_year_percent: f64,
    #[serde(rename = "trade_unique_id")]
    pub trade_unique_id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "bid_appl_seq_num")]
    pub bid_appl_seq_num: ::serde_json::Value,
    #[serde(rename = "offer_appl_seq_num")]
    pub offer_appl_seq_num: ::serde_json::Value,
}


impl Daum {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            self.symbol.to_string(),
            self.timestamp.to_string(),
            self.current.to_string(),
            self.trade_volume.to_string(),
            self.volume.to_string(),
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.last_close.to_string(),
            self.avg_price.to_string(),
            self.amount.to_string(),
            self.percent.to_string(),
            self.chg.to_string(),
            self.market_capital.to_string(),
            self.turnover_rate.to_string(),
            self.amplitude.to_string(),
            self.current_year_percent.to_string(),
            self.level.to_string(),
            self.trade_session.to_string(),
        );

        return rec;
    }
}