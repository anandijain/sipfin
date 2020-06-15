use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl crate::HasRecs for HolderRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.data
            .holdings_transactions
            .table
            .rows
            .iter()
            .map(|x| crate::HasRec::to_rec(x))
            .collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub ownership_summary: ::serde_json::Value,
    pub active_positions: ::serde_json::Value,
    pub new_sold_out_positions: ::serde_json::Value,
    pub holdings_transactions: HoldingsTransactions,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldingsTransactions {
    pub total_records: String,
    pub institutional_holders: String,
    pub shares_held: String,
    pub table: Table,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub headers: ::serde_json::Value,
    pub rows: Vec<HolderRow>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderRow {
    pub owner_name: String,
    pub date: String,
    pub shares_held: String,
    pub shares_change: String,
    #[serde(rename = "sharesChangePCT")]
    pub shares_change_pct: String,
    pub market_value: String,
    pub url: String,
}

impl crate::HasRec for HolderRow {
    fn to_rec(&self) -> Vec<String> {
        vec![
            self.owner_name.to_string(),
            self.date.to_string(),
            self.shares_held.to_string(),
            self.shares_change.to_string(),
            self.shares_change_pct.to_string(),
            self.market_value.to_string(),
            self.url.to_string(),
        ]
    }
}

pub const NDAQ_HOLDER_HEADER: [&'static str; 7] = [
    "owner",
    "t",
    "held",
    "change",
    "change_pct",
    "mkt_val",
    "url",
];

//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct OwnershipSummary {
//    #[serde(rename = "SharesOutstandingPCT")]
//    pub shares_outstanding_pct: gen::LabelValue,
//    #[serde(rename = "ShareoutstandingTotal")]
//    pub shareoutstanding_total: gen::LabelValue,
//    #[serde(rename = "TotalHoldingsValue")]
//    pub total_holdings_value: gen::LabelValue,
//}
//
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct {
//    pub headers: ::serde_json::Value,
//    pub rows: Vec<Row>,
//}
//
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct Row {
//    pub positions: String,
//    pub holders: String,
//    pub shares: String,
//}
//
