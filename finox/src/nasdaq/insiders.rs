use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsidersRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl crate::HasRecs for InsidersRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        return self.data
            .transaction_table.rows
            .iter()
            .map(|x| x.to_rec())
            .collect();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub title: ::serde_json::Value,
    pub number_of_trades: NumberOfTrades,
    pub number_of_shares_traded: NumberOfSharesTraded,
    pub transaction_table: TransactionTable,
    pub filer_transaction_table: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberOfTrades {
    pub headers: ::serde_json::Value,
    pub rows: Vec<Row>,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub insider_trade: String,
    pub months3: String,
    pub months12: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberOfSharesTraded {
    pub headers: ::serde_json::Value,
    pub rows: Vec<Row>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTable {
    pub headers: ::serde_json::Value,
    pub rows: Vec<TransactionRow>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRow {
    pub insider: String,
    pub relation: String,
    pub last_date: String,
    pub transaction_type: String,
    pub own_type: String,
    pub shares_traded: String,
    pub last_price: String,
    pub shares_held: String,
    pub url: String,
}

impl TransactionRow {
    pub fn to_rec(&self) -> Vec<String> {
        return vec![
            self.insider.to_string(),
            self.relation.to_string(),
            self.last_date.to_string(),
            self.transaction_type.to_string(),
            self.own_type.to_string(),
            self.shares_traded.to_string(),
            self.last_price.to_string(),
            self.shares_held.to_string(),
            self.url.to_string(),
        ];
    }
}

pub const NDAQ_INSIDER_HEADER: [&'static str; 9] = [
    "insider",
    "relation",
    "last_date",
    "transaction_type",
    "own_type",
    "shares_traded",
    "last_price",
    "shares_held",
    "url",
];
