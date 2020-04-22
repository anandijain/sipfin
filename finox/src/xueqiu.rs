// MSFT,AAPL,AMZN,GOOGL,BABA,FB,BRKA,JNJ,WMT,V,PG,JPM,TSM,UNH,MA,INTC,VZ,HD,T,MRK,KO,PFE,NVS,BAC,DIS,PEP,NFLX,XOM,CSCO,NVDA,TM,CMCSA,ORCL,ABT,ADBE,CVX,CHL,LLY,SAP,NKE,TSLA,MDT,MCD,BMY,RDSA.AS,AZN,PYPL,TMO,PM,NEE
// MSFT%2CAAPL%2CAMZN%2CGOOGL%2CBABA%2CFB%2CBRKA%2CJNJ%2CWMT%2CV%2CPG%2CJPM%2CTSM%2CUNH%2CMA%2CINTC%2CVZ%2CHD%2CT%2CMRK%2CKO%2CPFE%2CNVS%2CBAC%2CDIS%2CPEP%2CNFLX%2CXOM%2CCSCO%2CNVDA%2CTM%2CCMCSA%2CORCL%2CABT%2CADBE%2CCVX%2CCHL%2CLLY%2CSAP%2CNKE%2CTSLA%2CMDT%2CMCD%2CBMY%2CRDSA.AS%2CAZN%2CPYPL%2CTMO%2CPM%2CNEE


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    #[serde(rename = "error_code")]
    pub error_code: i64,
    #[serde(rename = "error_description")]
    pub error_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub count: i64,
    pub list: Vec<List>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub symbol: String,
    #[serde(rename = "net_profit_cagr")]
    pub net_profit_cagr: ::serde_json::Value,
    pub ps: ::serde_json::Value,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub percent: f64,
    #[serde(rename = "has_follow")]
    pub has_follow: bool,
    #[serde(rename = "tick_size")]
    pub tick_size: f64,
    #[serde(rename = "pb_ttm")]
    pub pb_ttm: ::serde_json::Value,
    #[serde(rename = "float_shares")]
    pub float_shares: ::serde_json::Value,
    pub current: f64,
    pub amplitude: f64,
    pub pcf: ::serde_json::Value,
    #[serde(rename = "current_year_percent")]
    pub current_year_percent: f64,
    #[serde(rename = "float_market_capital")]
    pub float_market_capital: ::serde_json::Value,
    #[serde(rename = "market_capital")]
    pub market_capital: i64,
    #[serde(rename = "dividend_yield")]
    pub dividend_yield: f64,
    #[serde(rename = "lot_size")]
    pub lot_size: i64,
    #[serde(rename = "roe_ttm")]
    pub roe_ttm: ::serde_json::Value,
    #[serde(rename = "total_percent")]
    pub total_percent: ::serde_json::Value,
    #[serde(rename = "percent5m")]
    pub percent5_m: f64,
    #[serde(rename = "income_cagr")]
    pub income_cagr: ::serde_json::Value,
    pub amount: i64,
    pub chg: f64,
    #[serde(rename = "issue_date_ts")]
    pub issue_date_ts: i64,
    #[serde(rename = "main_net_inflows")]
    pub main_net_inflows: f64,
    pub volume: i64,
    #[serde(rename = "volume_ratio")]
    pub volume_ratio: f64,
    pub pb: ::serde_json::Value,
    pub followers: i64,
    #[serde(rename = "turnover_rate")]
    pub turnover_rate: f64,
    #[serde(rename = "first_percent")]
    pub first_percent: ::serde_json::Value,
    pub name: String,
    #[serde(rename = "pe_ttm")]
    pub pe_ttm: i64,
    #[serde(rename = "total_shares")]
    pub total_shares: i64,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    #[serde(rename = "error_code")]
    pub error_code: i64,
    #[serde(rename = "error_description")]
    pub error_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub symbol: String,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub symbol: String,
    pub timestamp: i64,
    pub current: f64,
    pub chg: f64,
    pub percent: f64,
    #[serde(rename = "trade_volume")]
    pub trade_volume: i64,
    pub side: i64,
    pub level: i64,
    #[serde(rename = "trade_session")]
    pub trade_session: i64,
    #[serde(rename = "trade_type")]
    pub trade_type: Option<String>,
    #[serde(rename = "trade_unique_id")]
    pub trade_unique_id: String,
    #[serde(rename = "bid_appl_seq_num")]
    pub bid_appl_seq_num: ::serde_json::Value,
    #[serde(rename = "offer_appl_seq_num")]
    pub offer_appl_seq_num: ::serde_json::Value,
}
