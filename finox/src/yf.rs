extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;

/*
https://query1.finance.yahoo.com/v7/finance/spark?symbols=%5EGSPC&range=1d
https://query1.finance.yahoo.com/v7/finance/spark?symbols=BTCUSD%3DX&range=1d
https://query1.finance.yahoo.com/v1/finance/screener/instrument/earnings/fields?lang=en-US&region=US&category=keystats%2Cfinancials
https://query1.finance.yahoo.com/v8/finance/chart/AAPL?region=US&range=1d
https://query1.finance.yahoo.com/v8/finance/chart/USDEUR=X?symbol=USDEUR%3DX&range=1d&interval=1m
https://query2.finance.yahoo.com/ws/insights/v2/finance/insights?region=US&symbol=MSFT

https://finance.yahoo.com/_finance_doubledown/api/resource/YFinLists;count=3;listIds=%5B%22commodities%22%2C%22currencies%22%2C%22bonds%22%5D
https://www.marketwatch.com/DockingBar/Dock/Markets#
*/


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub chart: Chart,
}

impl Root {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        let ts = &self.chart.result[0].timestamp;
        let meta = &self.chart.result[0].meta;
        if let Some(quote) = &self.chart.result[0].indicators.quote[0] {
            for i in 0..ts.len() {
                let mut rec: Vec<String> = vec![meta.symbol.to_string()]; //Vec::new();
                if let Some(ohlcv) = Quote::to_record(quote, i) {
                    rec.push(ts[i].to_string());
                    rec.append(&mut ohlcv.clone());
                    ret.push(rec);
                }
            }
        }
        return ret;
    }

    pub fn meta_record(&self) -> Vec<String> {
        let rec = Meta::to_record(&self.chart.result[0].meta);
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub result: Vec<YFResult>,
    pub error: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YFResult {
    pub meta: Meta,
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
}

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Meta {
//     pub currency: String,
//     pub symbol: String,
//     pub exchange_name: String,
//     pub instrument_type: String,
//     pub first_trade_date: i64,
//     pub regular_market_time: i64,
//     pub gmtoffset: i64,
//     pub timezone: String,
//     pub exchange_timezone_name: String,
//     pub regular_market_price: f64,
//     pub chart_previous_close: f64,
//     pub previous_close: f64,
//     pub scale: Option<i64>,
//     pub price_hint: Option<i64>,
//     pub current_trading_period: Option<TradingPeriod>,
//     // pub trading_periods: Option<Vec<Vec<TradingPeriod>>>,
//     pub data_granularity: Option<String>,
//     pub range: Option<String>,
//     pub valid_ranges: Vec<String>,
// }
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
    pub price_hint: i64,
    pub current_trading_period: TradingPeriod,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

impl Meta {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec![
            self.symbol.to_string(),
            self.exchange_name.to_string(),
            self.instrument_type.to_string(),
            self.currency.to_string(),
            self.first_trade_date.to_string(),
            self.regular_market_time.to_string(),
            self.gmtoffset.to_string(),
            self.timezone.to_string(),
            self.exchange_timezone_name.to_string(),
        ];
        return rec;
    }
}



pub const YF_META_HEADER: [&'static str; 9] = [
"symbol",
"exchange",
"instrument",
"currency",
"first_trade_date",
"reg_mkt_time",
"gmtoffset",
"tz",
"exchange_tz",
];


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPeriod {
    pub timezone: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub gmtoffset: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indicators {
    pub quote: Vec<Option<Quote>>,
    pub adjclose: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    // todo option<vec<option<f64>>>
    pub open: Vec<Option<f64>>,
    pub close: Vec<Option<f64>>,
    pub volume: Vec<Option<i64>>,
    pub low: Vec<Option<f64>>,
    pub high: Vec<Option<f64>>,
}

impl Quote {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        for i in 0..self.high.len() {
            if let Some(rec) = Quote::to_record(self, i) {
                ret.push(rec);
            }
        }
        return ret;
    }

    pub fn to_record(&self, i: usize) -> Option<Vec<String>> {
        let mut rec: Vec<String> = Vec::new();

        if let Some(op) = self.open[i] {
            rec.push(op.to_string());
        } else {
            rec.push("".to_string());
        }

        if let Some(hi) = self.high[i] {
            rec.push(hi.to_string());
        } else {
            rec.push("".to_string());
        }

        if let Some(lo) = self.low[i] {
            rec.push(lo.to_string());
        } else {
            rec.push("".to_string());
        }

        if let Some(close) = self.close[i] {
            rec.push(close.to_string());
        } else {
            rec.push("".to_string());
        }

        if let Some(vol) = self.volume[i] {
            rec.push(vol.to_string());
        } else {
            rec.push("".to_string());
        }
        return serde::export::Some(rec);
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YFinList {
    pub primary_key: String,
    pub quotes: ListQuotes,
    #[serde(flatten)]
    index: ::serde_json::Value,
}

impl YFinList {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        for (k, quote) in &self.quotes.quotes {
            ret.push(YFinListSecurity::to_record(&quote));
        }
        return ret;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuotes {
    #[serde(flatten)]
    pub quotes: HashMap<String, YFinListSecurity>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YFinListSecurity {
    pub symbol: String,
    pub two_hundred_day_average_change_percent: Option<Rawf64>,
    // pub fifty_two_week_low_change_percent: Rawf64,
    pub language: String,
    pub head_symbol: Option<bool>,
    pub regular_market_day_range: Option<RawString>,
    pub regular_market_day_high: Option<Rawf64>,
    pub two_hundred_day_average_change: Option<Rawf64>,
    pub last_market: Option<String>,
    pub two_hundred_day_average: Option<Rawf64>,
    pub ask_size: Option<Rawi64>,
    pub fifty_two_week_high_change: Option<Rawf64>,
    pub expire_iso_date: Option<RawString>,
    pub fifty_two_week_range: Option<RawString>,
    pub fifty_day_average_change: Option<Rawf64>,
    pub average_daily_volume3_month: Option<Rawi64LongFmt>,
    pub exchange_data_delayed_by: i64,
    pub first_trade_date_milliseconds: Option<i64>,
    pub fifty_two_week_low: Option<Rawf64>,
    pub market: String,
    pub regular_market_volume: Option<Rawi64LongFmt>,
    pub price_hint: i64,
    pub source_interval: i64,
    pub regular_market_day_low: Option<Rawf64>,
    pub exchange: String,
    pub short_name: String,
    pub region: String,
    pub fifty_day_average_change_percent: Option<Rawf64>,
    pub full_exchange_name: Option<String>,
    pub underlying_exchange_symbol: Option<String>,
    pub open_interest: Option<Rawi64LongFmt>,
    pub gmt_off_set_milliseconds: i64,
    pub head_symbol_as_string: Option<String>,
    pub regular_market_open: Option<Rawf64>,
    pub regular_market_time: Rawi64,
    pub regular_market_change_percent: Option<Rawf64>,
    pub quote_type: String,
    pub average_daily_volume10_day: Option<Rawi64LongFmt>,
    pub fifty_two_week_low_change: Option<Rawf64>,
    pub underlying_symbol: Option<String>,
    pub fifty_two_week_high_change_percent: Option<Rawf64>,
    pub contract_symbol: Option<bool>,
    pub tradeable: bool,
    pub expire_date: Option<Rawi64>,
    pub currency: String,
    pub regular_market_previous_close: Option<Rawf64>,
    pub fifty_two_week_high: Option<Rawf64>,
    pub exchange_timezone_name: String,
    pub bid_size: Option<Rawi64LongFmt>,
    pub regular_market_change: Option<Rawf64>,
    pub fifty_day_average: Option<Rawf64>,
    pub exchange_timezone_short_name: String,
    pub regular_market_price: Option<Rawf64>,
    pub market_state: String,
    pub ask: Option<Rawf64>,
    pub bid: Option<Rawf64>,
    pub triggerable: bool,
}

impl YFinListSecurity {
    pub fn to_record(&self) -> Vec<String> {
        // self..to_string(),
        // lilmatcher_Rawf64(// lilmatcher(self).two_hundred_day_average.,
        // if let Some(p) = self.regular_market_price
        let rec: Vec<String> = vec![
            self.symbol.to_string(),
            self.exchange.to_string(),
            self.market_state.to_string(),
            lilmatcher_Rawf64(self.two_hundred_day_average_change_percent.clone()).to_string(),
            lilmatcher_Rawf64(self.regular_market_day_high.clone()).to_string(),
            lilmatcher_Rawf64(self.two_hundred_day_average_change.clone()).to_string(),
            lilmatcher_Rawf64(self.fifty_two_week_high_change.clone()).to_string(),
            lilmatcher_RawString(self.expire_iso_date.clone()).to_string(),
            lilmatcher_RawString(self.fifty_two_week_range.clone()).to_string(),
            lilmatcher_Rawf64(self.fifty_day_average_change.clone()).to_string(),
            lilmatcher_Rawi64L(self.average_daily_volume3_month.clone()).to_string(),
            self.exchange_data_delayed_by.to_string(),
            lilmatcher_Rawf64(self.fifty_two_week_low.clone()).to_string(),
            self.market.to_string(),
            lilmatcher_Rawi64L(self.regular_market_volume.clone()).to_string(),
            self.price_hint.to_string(),
            self.source_interval.to_string(),
            lilmatcher_Rawf64(self.regular_market_day_low.clone()).to_string(),
            lilmatcher_Rawi64(self.expire_date.clone()).to_string(),
            self.exchange.to_string(),
            self.region.to_string(),
            lilmatcher_Rawi64L(self.open_interest.clone()).to_string(),
            self.gmt_off_set_milliseconds.to_string(),
            lilmatcher_Rawf64(self.regular_market_open.clone()).to_string(),
            lilmatcher_Rawf64(self.regular_market_change_percent.clone()).to_string(),
            lilmatcher_Rawi64L(self.average_daily_volume10_day.clone()).to_string(),
            lilmatcher_Rawf64(self.fifty_two_week_low_change.clone()).to_string(),
            self.currency.to_string(),
            lilmatcher_Rawf64(self.regular_market_previous_close.clone()).to_string(),
            lilmatcher_Rawf64(self.fifty_two_week_high.clone()).to_string(),
            self.exchange_timezone_name.to_string(),
            lilmatcher_Rawf64(self.regular_market_change.clone()).to_string(),
            lilmatcher_Rawf64(self.fifty_day_average.clone()).to_string(),
            self.exchange_timezone_short_name.to_string(),
            lilmatcher_Rawf64(self.regular_market_price.clone()).to_string(),
            self.market_state.to_string(),
            lilmatcher_Rawf64(self.ask.clone()).to_string(),
            lilmatcher_Rawf64(self.bid.clone()).to_string(),
        ];
        // lilmatcher_Rawi64(self.regular_market_time).to_string(),
        // self.contract_symbol.to_string(),
        // lilmatcher_Rawf64(// self.bid_size).to_string(),
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawString {
    pub raw: String,
    pub fmt: String,
}

pub fn lilmatcher_RawString(s: Option<RawString>) -> String {
    match s {
        Some(s) => s.raw.to_string(),
        None => "".to_string(),
    }
}

pub fn lilmatcher_Rawf64(s: Option<Rawf64>) -> String {
    match s {
        Some(s) => s.raw.to_string(),
        None => "".to_string(),
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rawf64 {
    pub raw: f64,
    pub fmt: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rawi64LongFmt {
    pub raw: i64,
    pub fmt: String,
    pub long_fmt: String,
}

pub fn lilmatcher_Rawi64L(s: Option<Rawi64LongFmt>) -> String {
    match s {
        Some(s) => s.raw.to_string(),
        None => "".to_string(),
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rawi64 {
    pub raw: i64,
    pub fmt: String,
}

impl Rawi64 {
    pub fn to_record(&self) -> String {
        return self.raw.to_string();
    }
}

pub fn lilmatcher_Rawi64(s: Option<Rawi64>) -> String {
    match s {
        Some(s) => s.raw.to_string(),
        None => "".to_string(),
    }
}

pub const YF_LIST_HEADER: [&'static str; 38] = [
    "symbol",
    "exchange",
    "market_state",
    "two_hundred_day_average_change_percent",
    "regular_market_day_high",
    "two_hundred_day_average_change",
    "fifty_two_week_high_change",
    "expire_iso_date",
    "fifty_two_week_range",
    "fifty_day_average_change",
    "average_daily_volume3_month",
    "exchange_data_delayed_by",
    "fifty_two_week_low",
    "market",
    "regular_market_volume",
    "price_hint",
    "source_interval",
    "regular_market_day_low",
    "expire_date",
    "exchange",
    "region",
    "open_interest",
    "gmt_off_set_milliseconds",
    "regular_market_open",
    "regular_market_change_percent",
    "average_daily_volume10_day",
    "fifty_two_week_low_change",
    "currency",
    "regular_market_previous_close",
    "fifty_two_week_high",
    "exchange_timezone_name",
    "regular_market_change",
    "fifty_day_average",
    "exchange_timezone_short_name",
    "regular_market_price",
    "market_state",
    "ask",
    "bid",
];
