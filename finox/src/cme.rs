#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CMERoot {
    pub quote_delayed: bool,
    pub quote_delay: String,
    pub trade_date: String,
    pub quotes: Vec<Quote>,
    pub empty: bool,
}

impl crate::HasRecs for CMERoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.quotes.iter().map(|x| x.to_rec()).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub last: String,
    pub change: String,
    pub prior_settle: String,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub high_limit: String,
    pub low_limit: String,
    pub volume: String,
    pub md_key: String,
    pub quote_code: String,
    pub escaped_quote_code: String,
    pub code: String,
    pub updated: String,
    pub percentage_change: String,
    pub expiration_month: String,
    pub expiration_code: String,
    pub expiration_date: String,
    pub product_name: String,
    pub product_code: String,
    pub uri: String,
    pub product_id: i64,
    pub exchange_code: String,
    pub option_uri: String,
    pub has_option: bool,
    pub last_trade_date: LastTradeDate,
    pub price_chart: PriceChart,
    pub net_change_status: String,
    pub high_low_limits: String,
}

impl Quote {
    pub fn to_rec(&self) -> Vec<String> {
        return vec![
            self.last_trade_date.timestamp.to_string(),
            self.last.to_string(),
            self.change.to_string(),
            self.prior_settle.to_string(),
            self.open.to_string(),
            self.close.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.high_limit.to_string(),
            self.low_limit.to_string(),
            self.volume.to_string(),
            self.md_key.to_string(),
            self.quote_code.to_string(),
            self.expiration_month.to_string(),
            self.expiration_date.to_string(),
            self.product_name.to_string(),
            self.product_code.to_string(),
            self.uri.to_string(),
            self.product_id.to_string(),
            self.exchange_code.to_string(),
            self.option_uri.to_string(),
            self.has_option.to_string(),
        ];
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastTradeDate {
    pub timestamp: i64,
    pub date_only_long_format: String,
    pub default24: String,
    pub default12: String,
    pub verbose: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceChart {
    pub enabled: bool,
    pub code: String,
    pub month_year: String,
    pub venue: i64,
    pub title: String,
    pub year: i64,
}
