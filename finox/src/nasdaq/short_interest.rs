use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortInterestRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}
// todo impl hasrecs

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub symbol: String,
    pub short_interest_table: ShortInterestTable,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortInterestTable {
    pub headers: Headers,
    pub rows: Vec<Row>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub settlement_date: String,
    pub interest: String,
    pub avg_daily_share_volume: String,
    pub days_to_cover: f64,
}

pub const NDAQ_SHORT_HEADER: [&'static str; 4] = [
    "settlement_date",
    "interest",
    "avg_daily_share_volume",
    "days_to_cover",
];
