use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl RealtimeRoot {
    pub fn to_recs(&self) -> Vec<Vec<String>> {
        return self.data.to_recs(); 
    }

    pub fn get_id(&self) -> String {
        return format!("{}_rt", self.data.symbol.to_string());
    }

    pub fn gen_header(&self) -> Vec<String> {
        return NDAQ_REALTIME_HEADER
            .iter()
            .map(|x| x.clone().to_string())
            .collect();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub symbol: String,
    pub total_records: i64,
    pub offset: i64,
    pub limit: i64,
    pub headers: ::serde_json::Value,
    pub rows: Vec<Row>,
}

impl Data {
    pub fn to_recs(&self) -> Vec<Vec<String>> {
        return self.rows.iter().map(|x| x.to_rec(self.symbol.clone())).collect();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub nls_time: String,
    pub nls_price: String,
    pub nls_share_volume: String,
}

impl Row {
    pub fn to_rec(&self, symbol: String) -> Vec<String> {
        return vec![
            symbol,
            self.nls_time.to_string(),
            self.nls_price.to_string(),
            self.nls_share_volume.to_string(),
            ];
    }
}


pub const NDAQ_REALTIME_HEADER: [&'static str; 4] = [
    "symbol",
    "t",
    "x",
    "v",
];
