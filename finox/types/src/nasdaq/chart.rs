use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartRoot {
    pub data: ChartData,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    pub symbol: String,
    pub company: String,
    pub time_as_of: String,
    pub is_nasdaq100: bool,
    pub last_sale_price: String,
    pub net_change: String,
    pub percentage_change: String,
    pub delta_indicator: String,
    pub previous_close: String,
    pub chart: Vec<Chart>,
    pub events: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub z: Z,
    pub x: i64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Z {
    pub date_time: String,
    pub value: String,
}

