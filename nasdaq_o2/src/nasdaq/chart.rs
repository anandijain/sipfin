use crate::nasdaq::gen;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartRoot {
    pub data: ChartData,
    pub message: ::serde_json::Value,
    pub status: ::serde_json::Value,
}

impl ChartRoot {
    pub fn to_recs(&self) -> Vec<Vec<String>> {
        let symb = self.data.symbol.to_string();
        return self.data
            .chart
            .iter()
            .map(|c| vec![symb.to_string(), c.x.to_string(), c.y.to_string()])
            .collect();
    }

    pub fn get_id(&self) -> String {
        let mut id: String = self.data.symbol.to_string();
        id.push('c');
        return id;
    }

    //pub fn gen_header(&self) -> Vec<String> {
    //    //return vec!["t".to_string(), self.data.symbol.to_string()]; //chart header
    //    return vec!["t".to_string(), self.data.symbol.to_string()]; //chart header
    //}
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    pub symbol: String,
    pub company: String,
    pub time_as_of: String,
    pub is_nasdaq100: bool,
    pub last_sale_price: String,
    pub net_change: String,
    pub percentage_change: String,
    pub delta_indicator: String,
    pub previous_close: String,
    pub chart: Vec<Chart>,
    pub events: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub z: gen::DateVal,
    pub x: i64,
    pub y: f64,
}


pub const NDAQ_CHART_HEADER: [&'static str; 3] = [
    "symbol",
   "t",
   "x",
];

