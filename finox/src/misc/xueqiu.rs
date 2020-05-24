extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::utils;
// MSFT,AAPL,AMZN,GOOGL,BABA,FB,BRKA,JNJ,WMT,V,PG,JPM,TSM,UNH,MA,INTC,VZ,HD,T,MRK,KO,PFE,NVS,BAC,DIS,PEP,NFLX,XOM,CSCO,NVDA,TM,CMCSA,ORCL,ABT,ADBE,CVX,CHL,LLY,SAP,NKE,TSLA,MDT,MCD,BMY,RDSA.AS,AZN,PYPL,TMO,PM,NEE
// MSFT%2CAAPL%2CAMZN%2CGOOGL%2CBABA%2CFB%2CBRKA%2CJNJ%2CWMT%2CV%2CPG%2CJPM%2CTSM%2CUNH%2CMA%2CINTC%2CVZ%2CHD%2CT%2CMRK%2CKO%2CPFE%2CNVS%2CBAC%2CDIS%2CPEP%2CNFLX%2CXOM%2CCSCO%2CNVDA%2CTM%2CCMCSA%2CORCL%2CABT%2CADBE%2CCVX%2CCHL%2CLLY%2CSAP%2CNKE%2CTSLA%2CMDT%2CMCD%2CBMY%2CRDSA.AS%2CAZN%2CPYPL%2CTMO%2CPM%2CNEE
// https://stock.xueqiu.com/v5/stock/realtime/quotec.json?
// https://stock.xueqiu.com/v5/stock/history/trade.json?symbol=AAPL&count=20
// https://stock.xueqiu.com/v5/stock/chart/minute.json?symbol=.DJI&period=1d

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeQuotec {
    pub data: Vec<Quote>,
    #[serde(rename = "error_code")]
    pub error_code: i64,
    #[serde(rename = "error_description")]
    pub error_description: ::serde_json::Value,
}


impl RealtimeQuotec {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for t in self.data.iter() {
            // println!("{:#?}", t);
            recs.push(Quote::to_record(t));
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
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
    pub trade_volume: Option<i64>,
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
    pub trade_unique_id: ::serde_json::Value,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "bid_appl_seq_num")]
    pub bid_appl_seq_num: ::serde_json::Value,
    #[serde(rename = "offer_appl_seq_num")]
    pub offer_appl_seq_num: ::serde_json::Value,
}

impl Quote {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            self.symbol.to_string(),
            self.timestamp.to_string(),
            self.current.to_string(),
            utils::lilmatcher_i64(self.trade_volume),
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

pub const snowballQuoteHeader: [&'static str; 19] = [
    "symbol",
    "timestamp",
    "current",
    "trade_volume",
    "volume",
    "open",
    "high",
    "low",
    "last_close",
    "avg_price",
    "amount",
    "percent",
    "chg",
    "market_capital",
    "turnover_rate",
    "amplitude",
    "current_year_percent",
    "level",
    "trade_session",
];


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub data: Trades,
    #[serde(rename = "error_code")]
    pub error_code: i64,
    #[serde(rename = "error_description")]
    pub error_description: String,
}


impl TradeHistory {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for t in self.data.items.iter() {
            println!("{:#?}", t);
            recs.push(Trade::to_record(t));
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trades {
    pub symbol: String,
    pub items: Vec<Trade>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub symbol: String,
    pub timestamp: i64,
    pub current: f64,
    pub chg: f64,
    pub percent: f64,
    #[serde(rename = "trade_volume")]
    pub trade_volume: i64,
    pub side: i64,
    pub level: i64,
    #[serde(rename = "trade_session")]
    pub trade_session: i64,
    #[serde(rename = "trade_type")]
    pub trade_type: Option<String>,
    #[serde(rename = "trade_unique_id")]
    pub trade_unique_id: String,
    #[serde(rename = "bid_appl_seq_num")]
    pub bid_appl_seq_num: ::serde_json::Value,
    #[serde(rename = "offer_appl_seq_num")]
    pub offer_appl_seq_num: ::serde_json::Value,
}


impl Trade {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            self.symbol.to_string(),
            self.timestamp.to_string(),
            self.current.to_string(),
            self.chg.to_string(),
            self.percent.to_string(),
            self.trade_volume.to_string(),
            self.side.to_string(),
            self.level.to_string(),
            utils::lilmatcher(self.trade_type.clone()),
            self.trade_unique_id.to_string(),
        );
        return rec;
    }
}

pub const snowballTradeHeader: [&'static str; 10] = [
    "symbol",
    "timestamp",
    "current",
    "chg",
    "percent",
    "trade_volume",
    "side",
    "level",
    "trade_type",
    "trade_unique_id",
];

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnowballMinute {
    pub data: MinuteData,
    #[serde(rename = "error_code")]
    pub error_code: i64,
    #[serde(rename = "error_description")]
    pub error_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinuteData {
    #[serde(rename = "last_close")]
    pub last_close: f64,
    pub items: Vec<MinuteQuote>,
    #[serde(rename = "items_size")]
    pub items_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinuteQuote {
    pub current: f64,
    pub volume: i64,
    #[serde(rename = "avg_price")]
    pub avg_price: f64,
    pub chg: f64,
    pub percent: f64,
    pub timestamp: i64,
    pub amount: f64,
    pub high: f64,
    pub low: f64,
    pub macd: ::serde_json::Value,
    pub kdj: ::serde_json::Value,
    pub ratio: ::serde_json::Value,
    pub capital: ::serde_json::Value,
    #[serde(rename = "volume_compare")]
    pub volume_compare: VolumeCompare,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeCompare {
    #[serde(rename = "volume_sum")]
    pub volume_sum: i64,
    #[serde(rename = "volume_sum_last")]
    pub volume_sum_last: i64,
}
