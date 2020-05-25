use crate::nasdaq::gen;
use chrono::{DateTime, FixedOffset, Utc};
//use crate::nasdaq::gen::HasRecs;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

//impl HasRecs for RealtimeRoot {
//    fn to_recs(&self) -> Vec<Vec<String>> {
//        return self.data.to_recs();
//    }
//}

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
        return self
            .rows
            .iter()
            .flat_map(|x| x.to_rec(self.symbol.clone()))
            .collect();
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
    pub fn to_rec(&self, symbol: String) -> Option<Vec<String>> {
        if let Ok(t) = nls_to_dt(&self.nls_time) {
            return Some(vec![
                symbol,
                t.to_rfc3339(),
                self.nls_price.to_string().replace("$ ", ""),
                self.nls_share_volume.to_string().replace(",", ""),
            ]);
        }
        return None;
    }
}
pub fn nls_to_dt(s: &str) -> Result<DateTime<FixedOffset>, chrono::ParseError> {
    let t = format!("{} {} +05:00", Utc::now().format("%Y-%m-%d"), s);
    return DateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S %z");
}

pub const NDAQ_REALTIME_HEADER: [&'static str; 4] = ["symbol", "t", "x", "v"];
