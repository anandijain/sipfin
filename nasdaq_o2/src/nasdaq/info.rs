use crate::nasdaq::gen;

// https://api.nasdaq.com/api/quote/AAPL/info?assetclass=stocks
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoRoot {
    pub data: InfoData,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl InfoRoot {
    pub fn to_rec(&self) -> Vec<String> {
        return InfoData::to_rec(&self.data);
    }

    pub fn get_id(&self) -> String {
        let mut id: String = self.data.symbol.to_string();
        id.push('i');
        return id;
    }

    pub fn gen_header(&self) -> Vec<String> {
        return NDAQ_QUOTE_HEADER
            .iter()
            .map(|x| x.clone().to_string())
            .collect();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoData {
    pub symbol: String,
    pub company_name: String,
    pub stock_type: ::serde_json::Value,
    pub exchange: String,
    pub is_nasdaq_listed: bool,
    pub is_nasdaq100: bool,
    pub is_held: bool,
    pub primary_data: PrimaryData,
    pub secondary_data: ::serde_json::Value,
    pub key_stats: ::serde_json::Value,
    pub market_status: String,
    pub asset_class: String,
}

impl InfoData {
    pub fn to_rec(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec![
            self.symbol.to_string(),
            self.company_name.to_string(),
            self.stock_type.to_string(),
            self.exchange.to_string(),
            self.is_nasdaq_listed.to_string(),
            self.is_nasdaq100.to_string(),
            self.is_held.to_string(),
        ];
        rec.append(&mut PrimaryData::to_rec(&self.primary_data));
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryData {
    pub last_sale_price: String,
    pub net_change: String,
    pub percentage_change: String,
    pub delta_indicator: String,
    pub last_trade_timestamp: String,
    pub is_real_time: bool,
}

impl PrimaryData {
    pub fn to_rec(&self) -> Vec<String> {
        // let ts = self.last_trade_timestamp.split("OF ").collect::<Vec<&str>>();
        // println!("{:#?}", ts);
        return vec![
            self.last_trade_timestamp.to_string(), //.split_at(mid: usize),
            // ts[1].to_string(),
            self.last_sale_price.to_string(),
            self.net_change.to_string(),
            self.percentage_change.to_string(),
            self.is_real_time.to_string(),
            self.delta_indicator.to_string(),
        ];
    }
}

// commodities diff than stocks, serializing to Value
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct KeyStats {
//    #[serde(rename = "Volume")]
//    pub volume: gen::LabelValue,
//    #[serde(rename = "PreviousClose")]
//    pub previous_close: gen::LabelValue,
//    #[serde(rename = "OpenPrice")]
//    pub open_price: gen::LabelValue,
//    #[serde(rename = "MarketCap")]
//    pub market_cap: gen::LabelValue,
//}

pub const NDAQ_QUOTE_HEADER: [&'static str; 13] = [
    "symbol",
    "company_name",
    "stock_type",
    "exchange",
    "is_nasdaq_listed",
    "is_nasdaq100",
    "is_held",
    "last_trade_timestamp",
    "last_sale_price",
    "net_change",
    "percentage_change",
    "is_real_time",
    "delta_indicator",
];
