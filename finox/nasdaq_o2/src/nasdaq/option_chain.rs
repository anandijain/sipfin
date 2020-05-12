use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainRoot {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: gen::Status,
}

impl OptionChainRoot {
    pub fn to_recs(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = vec![];
        for row in self.data.option_chain_list.rows.iter() {
            recs.append(&mut row.to_recs())
        }
        return recs;
    }

    pub fn get_id(&self) -> String {
        return self.data.option_chain_list.rows[0]
            .call
            .symbol
            .to_string()
            .split_whitespace()
            .next()
            .expect("wtf option ticker")
            .to_string();
    }

    pub fn gen_header(&self) -> Vec<String> {
        return NDAQ_OPTION_HEADER.iter().map(|x| x.clone().to_string()).collect();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub total_record: i64,
    pub last_trade: String,
    pub option_chain_list: OptionChainList,
    pub month_filter: Vec<MonthFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainList {
    pub headers: ::serde_json::Value,
    pub rows: Vec<OptionRow>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherOrCommon {
    pub strike: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionRow {
    pub call: OptionData2,
    pub put: OptionData2,
}

impl OptionRow {
    pub fn to_recs(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = vec![];
        let put: Vec<String> = OptionData2::to_rec(&self.put);
        let call: Vec<String> = OptionData2::to_rec(&self.call);

        recs.push(put);
        recs.push(call);
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
            self.last.to_string(),
            self.change.to_string(),
            self.bid.to_string(),
            self.ask.to_string(),
            self.volume.to_string(),
            self.openinterest.to_string(),
            self.strike.to_string(),
            self.expiry_date.to_string(),
            self.colour.to_string(),
        ];
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthFilter {
    pub month: String,
    pub dates: Vec<gen::LabelValue>,
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
