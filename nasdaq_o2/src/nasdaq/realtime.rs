use crate::nasdaq::gen;
use chrono::{DateTime, FixedOffset};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl RealtimeRoot {
    pub fn to_recs(&self, t: DateTime<FixedOffset>) -> (Option<Vec<Vec<String>>>, DateTime<FixedOffset>) {
        return self.data.to_recs(t);
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
    pub fn to_recs(&self, t: DateTime<FixedOffset>) -> (Option<Vec<Vec<String>>>, DateTime<FixedOffset>)
    {
        let mut recs = vec![];
        let mut newest = t;
        for r in self.rows.iter() {
            let tup = r.to_rec(&self.symbol, t);
            match tup {
                Some((v, new_t)) => {
                    if new_t > newest {
                        newest = new_t;
                    }
                    recs.push(v);
                }
                None => break,
            }
        }
        if newest == t {
            return (None, t);
        }
        //println!("new t: {:?}, old t: {:?}", newest, t);
        return (Some(recs), newest);
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
    pub fn to_rec(
        &self,
        symbol: &str,
        last_new: DateTime<FixedOffset>,
    ) -> Option<(Vec<String>, DateTime<FixedOffset>)> {
        /* if the current rec has a time newer than the previous newest time
         * then it must be new data
         */
        if let Ok(t) = crate::nls_to_dt(&self.nls_time) {
            if last_new <= t {
                return Some((
                    vec![
                        symbol.to_string(),
                        t.to_rfc3339(),
                        self.nls_price.to_string().replace("$ ", ""),
                        self.nls_share_volume.to_string().replace(",", ""),
                    ],
                    t,
                ));
            } else {
                return None;
            }
            // prob change, sending true because failed to parse
        }
        return None;
    }
}
pub const NDAQ_REALTIME_HEADER: [&'static str; 4] = ["symbol", "t", "x", "v"];
