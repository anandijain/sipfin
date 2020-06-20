use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl crate::HasRecs for EarningsRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.data
            .earnings_per_share
            .iter()
            .map(|x| crate::HasRec::to_rec(x))
            .collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub symbol: String,
    pub earnings_per_share: Vec<EarningsPerShare>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsPerShare {
    #[serde(rename = "type")]
    pub type_field: String,
    pub period: String,
    pub consensus: f64,
    pub earnings: f64,
}

impl crate::HasRec for EarningsPerShare {
    fn to_rec(&self) -> Vec<String> {
        vec![
            self.period.to_string(),
            self.earnings.to_string(),
            self.consensus.to_string(),
        ]
    }
}

pub const NDAQ_EARNINGS_HEADER: [&'static str; 3] = ["t", "earnings", "consensus"];
