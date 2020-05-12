use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub total_record: i64,
    pub last_trade: String,
    pub option_chain_list: OptionChainList,
    pub month_filter: Vec<MonthFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainList {
    pub headers: Headers,
    pub rows: Vec<OptionRow>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    pub call: OptionData,
    pub put: OptionData,
    pub other_or_common: OtherOrCommon,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionData {
    pub last: String,
    pub change: String,
    pub bid: String,
    pub ask: String,
    pub volume: String,
    pub openinterest: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherOrCommon {
    pub strike: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionRow {
    pub call: OptionData2,
    pub put: OptionData2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionData2 {
    pub symbol: String,
    pub last: String,
    pub change: String,
    pub bid: String,
    pub ask: String,
    pub volume: String,
    pub openinterest: String,
    pub strike: String,
    pub expiry_date: String,
    pub colour: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthFilter {
    pub month: String,
    pub dates: Vec<gen::LabelValue>,
}

