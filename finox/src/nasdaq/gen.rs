#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub r_code: i64,
    pub b_code_message: ::serde_json::Value,
    pub developer_message: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelValue {
    pub label: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateVal {
    pub date_time: String,
    pub value: String,
}

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub trait HasRecs {
    fn to_recs(&self) -> Vec<Vec<String>>;
}

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub trait HasRec {
    fn to_rec(&self) -> Vec<String>;
}
