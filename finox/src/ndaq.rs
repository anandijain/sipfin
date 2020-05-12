/*
ftp://ftp.nasdaqtrader.com/SymbolDirectory/

ftp://ftp.nasdaqtrader.com/SymbolDirectory/bondslist.txt
ftp://ftp.nasdaqtrader.com/SymbolDirectory/bxoptions.txt

https://api.nasdaq.com/api/quote/EURUSD/summary?assetclass=currencies

*/

//https://api.nasdaq.com/api/calendar/upcoming

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarRoot {
    pub data: Vec<Daum>,
    pub message: ::serde_json::Value,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub splits_list: Option<Vec<SplitsList>>,
    pub name: String,
    pub event_count: i64,
    pub earnings_list: Option<Vec<EarningsList>>,
    pub dividends_list: Option<Vec<DividendsList>>,
    pub econs_list: Option<Vec<EconsList>>,
    pub ipos_list: Option<Vec<IposList>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitsList {
    pub company_name: String,
    pub execution_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsList {
    pub company_name: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividendsList {
    pub company_name: String,
    pub ex_div_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconsList {
    pub event_name: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IposList {
    pub company_name: String,
    pub price: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub r_code: i64,
    pub b_code_message: ::serde_json::Value,
    pub developer_message: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub message: ::serde_json::Value,
    pub status: Status,
}

//https://api.nasdaq.com/api/ipo/calendar?date=2020-04
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub priced: Priced,
    pub upcoming: Upcoming,
    pub filed: Filed,
    pub withdrawn: Withdrawn,
    pub month: i64,
    pub year: i64,
    pub total_results: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priced {
    pub headers: Headers,
    pub rows: Vec<Row>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    pub proposed_ticker_symbol: String,
    pub company_name: String,
    pub proposed_exchange: String,
    pub proposed_share_price: String,
    pub shares_offered: String,
    pub priced_date: String,
    pub dollar_value_of_shares_offered: String,
    pub deal_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    #[serde(rename = "dealID")]
    pub deal_id: String,
    pub proposed_ticker_symbol: String,
    pub company_name: String,
    pub proposed_exchange: String,
    pub proposed_share_price: String,
    pub shares_offered: String,
    pub priced_date: String,
    pub dollar_value_of_shares_offered: String,
    pub deal_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upcoming {
    pub upcoming_table: UpcomingTable,
    pub last_updated_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpcomingTable {
    pub headers: Headers2,
    pub rows: Vec<Row2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers2 {
    pub proposed_ticker_symbol: String,
    pub company_name: String,
    pub proposed_exchange: String,
    pub proposed_share_price: String,
    pub shares_offered: String,
    pub expected_price_date: String,
    pub dollar_value_of_shares_offered: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row2 {
    #[serde(rename = "dealID")]
    pub deal_id: String,
    pub proposed_ticker_symbol: String,
    pub company_name: String,
    pub proposed_exchange: String,
    pub proposed_share_price: String,
    pub shares_offered: String,
    pub expected_price_date: String,
    pub dollar_value_of_shares_offered: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filed {
    pub headers: Headers3,
    pub rows: Vec<Row3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers3 {
    pub proposed_ticker_symbol: String,
    pub company_name: String,
    pub filed_date: String,
    pub dollar_value_of_shares_offered: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row3 {
    #[serde(rename = "dealID")]
    pub deal_id: String,
    pub proposed_ticker_symbol: Option<String>,
    pub company_name: String,
    pub filed_date: String,
    pub dollar_value_of_shares_offered: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawn {
    pub headers: Headers4,
    pub rows: Vec<Row4>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers4 {
    pub proposed_ticker_symbol: String,
    pub company_name: String,
    pub proposed_exchange: String,
    pub shares_offered: String,
    pub filed_date: String,
    pub dollar_value_of_shares_offered: String,
    pub withdraw_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row4 {
    #[serde(rename = "dealID")]
    pub deal_id: String,
    pub proposed_ticker_symbol: ::serde_json::Value,
    pub company_name: String,
    pub proposed_exchange: ::serde_json::Value,
    pub shares_offered: String,
    pub filed_date: String,
    pub dollar_value_of_shares_offered: String,
    pub withdraw_date: String,
}

//https://www.nasdaq.com/api/v1/recent-articles/undefined/500
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsRoot {
    pub title: String,
    pub url: String,
    pub ago: String,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyStatsCurs {
    #[serde(rename = "Open")]
    pub open: AssetItem,
    #[serde(rename = "Bid")]
    pub bid: AssetItem,
    #[serde(rename = "Ask")]
    pub ask: AssetItem,
    #[serde(rename = "High")]
    pub high: AssetItem,
    #[serde(rename = "Low")]
    pub low: AssetItem,
}

