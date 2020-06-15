#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub symbol: String,
    pub tabs: ::serde_json::Value,
    pub income_statement_table: BalTable,
    pub balance_sheet_table: BalTable,
    pub cash_flow_table: BalTable,
    pub financial_ratios_table: BalTable,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalTable{
    pub headers: Headers,
    pub rows: Vec<Row>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetVals {
    pub value1: String,
    pub value2: String,
    pub value3: String,
    pub value4: String,
    pub value5: String,
}



