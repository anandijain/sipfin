extern crate serde;
extern crate serde_derive;
extern crate serde_json;

/*
https://query1.finance.yahoo.com/v7/finance/spark?symbols=%5EGSPC&range=1d
https://query1.finance.yahoo.com/v7/finance/spark?symbols=BTCUSD%3DX&range=1d
https://query1.finance.yahoo.com/v1/finance/screener/instrument/earnings/fields?lang=en-US&region=US&category=keystats%2Cfinancials
https://query1.finance.yahoo.com/v8/finance/chart/AAPL?region=US&range=1d
https://query1.finance.yahoo.com/v8/finance/chart/USDEUR=X?symbol=USDEUR%3DX&range=1d&interval=1m
https://query2.finance.yahoo.com/ws/insights/v2/finance/insights?region=US&symbol=MSFT

https://finance.yahoo.com/_finance_doubledown/api/resource/YFinLists;count=3;listIds=%5B%22commodities%22%2C%22currencies%22%2C%22bonds%22%5D
*/

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YFRoot {
    pub chart: Chart,
}

impl crate::HasRecs for YFRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        let ts = &self.chart.result[0].timestamp;
        let meta = &self.chart.result[0].meta;
        if let Some(quote) = &self.chart.result[0].indicators.quote[0] {
            for i in 0..ts.len() {
                let mut rec: Vec<String> = vec![meta.symbol.to_string()]; //Vec::new();
                if let Some(ohlcv) = Quote::to_rec(quote, i) {
                    //rec.push(self.chart.result[0].meta.symbol.to_string());
                    rec.push(ts[i].to_string());
                    rec.append(&mut ohlcv.clone());
                    ret.push(rec);
                }
            }
        }
        return ret;
    }
}

impl YFRoot {
    pub fn meta_record(&self) -> Vec<String> {
        let rec = Meta::to_rec(&self.chart.result[0].meta);
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
    pub current_trading_period: ::serde_json::Value,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

impl Meta {
    pub fn to_rec(&self) -> Vec<String> {
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
//??
impl crate::HasRecs for Quote {
    fn to_recs(&self) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        for i in 0..self.high.len() {
            if let Some(rec) = Quote::to_rec(self, i) {
                ret.push(rec);
            }
        }
        return ret;
    }
}

// TODO refac
impl Quote {
    pub fn to_rec(&self, i: usize) -> Option<Vec<String>> {
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
