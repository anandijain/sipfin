extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// https://query1.finance.yahoo.com/v7/finance/spark?symbols=%5EGSPC&range=1d
// https://query1.finance.yahoo.com/v7/finance/spark?symbols=BTCUSD%3DX&range=1d
// https://query1.finance.yahoo.com/v1/finance/screener/instrument/earnings/fields?lang=en-US&region=US&category=keystats%2Cfinancials
// https://finance.yahoo.com/_finance_doubledown/api/resource/finance.market-time?
// https://query1.finance.yahoo.com/v8/finance/chart/AAPL?region=US&range=1d
// https://query1.finance.yahoo.com/v8/finance/chart/USDEUR=X?symbol=USDEUR%3DX&range=1d&interval=1m

impl Root {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        let ts = &self.chart.result[0].timestamp;
        if let Some(quote) = &self.chart.result[0].indicators.quote[0] {

            for i in 0..ts.len(){
                let mut rec: Vec<String> = Vec::new();
                if let Some(ohlcv) = Quote::to_record(quote, i){
                    rec.push(ts[i].to_string());
                    for elt in ohlcv.iter(){
                        rec.push(elt.to_string());
                    }
                    ret.push(rec);
                }
            }
        }
        return ret;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub chart: Chart,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub result: Vec<Result>,
    pub error: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub meta: Meta,
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub currency: Option<String>,
    pub symbol: Option<String>,
    pub exchange_name: Option<String>,
    pub instrument_type: String,
    pub first_trade_date: Option<i64>,
    pub regular_market_time: Option<i64>,
    pub gmtoffset: Option<i64>,
    pub timezone: Option<String>,
    pub exchange_timezone_name: String,
    pub regular_market_price: Option<f64>,
    pub chart_previous_close: Option<f64>,
    pub previous_close: Option<f64>,
    pub scale: Option<i64>,
    pub price_hint: Option<i64>,
    pub current_trading_period: Option<TradingPeriod>,
    pub trading_periods: Option<Vec<Vec<TradingPeriod>>>,
    pub data_granularity: Option<String>,
    pub range: Option<String>,
    pub valid_ranges: Vec<String>,
}

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
