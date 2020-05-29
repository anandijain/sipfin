use crate::nasdaq::gen;
// use crate::nasdaq::gen::HasRecs;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl crate::HasRecs for OptionChainRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        let mut recs = vec![];
        for row in self.data.option_chain_list.rows.iter() {
            recs.append(&mut row.to_recs())
        }
        return recs;
    }
    //pub fn get_id(&self) -> String {
    //    return self.data.option_chain_list.rows[0]
    //        .call
    //        .symbol
    //        .to_string()
    //        .split_whitespace()
    //        .next()
    //        .expect("wtf option ticker")
    //        .to_string();
    //}
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub total_record: i64,
    pub last_trade: String,
    pub option_chain_list: OptionChainList,
    pub month_filter: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainList {
    pub headers: ::serde_json::Value,
    pub rows: Vec<OptionRow>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionRow {
    pub call: Option<OptionData2>,
    pub put: Option<OptionData2>,
}

impl OptionRow {
    pub fn to_recs(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = vec![];

        if let Some(c) = &self.call {
            let call: Vec<String> = OptionData2::to_rec(&c);
            recs.push(call);
        }
        if let Some(p) = &self.put{

        let put: Vec<String> = OptionData2::to_rec(&p);
        recs.push(put);
        }
        return recs;
    }
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

impl OptionData2 {
    pub fn to_rec(&self) -> Vec<String> {
        return vec![
            self.symbol.to_string(),
            self.last.to_string().replace("--", ""),
            self.change.to_string().replace("--", ""),
            self.bid.to_string().replace("--", ""),
            self.ask.to_string().replace("--", ""),
            self.volume.to_string().replace("--", ""),
            self.openinterest.to_string().replace("--", ""),
            self.strike.to_string(),
            self.expiry_date.to_string(),
            self.colour.to_string(),
        ];
    }
}

pub const NDAQ_OPTION_HEADER: [&'static str; 10] = [
    "symbol",
    "last",
    "change",
    "bid",
    "ask",
    "volume",
    "openinterest",
    "strike",
    "expiry_date",
    "colour",
];
