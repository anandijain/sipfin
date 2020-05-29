use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividendsRoot {
    pub data: DividendData,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl crate::HasRecs for DividendsRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        return self
            .data
            .dividends
            .rows
            .iter()
            .map(|c| c.to_rec())
            .collect();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividendData {
    pub ex_dividend_date: String,
    pub dividend_payment_date: String,
    #[serde(rename = "yield")]
    pub yield_field: String,
    pub annualized_dividend: String,
    pub payout_ratio: String,
    pub dividends: HeadersRows,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadersRows {
    pub headers: ::serde_json::Value,
    pub rows: Vec<Row>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub ex_or_eff_date: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub amount: String,
    pub declaration_date: String,
    pub record_date: String,
    pub payment_date: String,
}

impl Row {
    pub fn to_rec(&self) -> Vec<String> {
        return vec![
            self.ex_or_eff_date.to_string(),
            self.type_field.to_string(),
            self.amount.to_string(),
            self.declaration_date.to_string(),
            self.record_date.to_string(),
            self.payment_date.to_string(),
        ];
    }
}

pub const NDAQ_DIVIDEND_HEADER: [&'static str; 6] = [
    "ex_or_eff_date",
    "type_field",
    "amount",
    "declaration_date",
    "record_date",
    "payment_date",
];
