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

// https://api.nasdaq.com/api/quote/AAPL/info?assetclass=stocks
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetRoot {
    pub data: AssetData,
    pub message: ::serde_json::Value,
    pub status: Status,
}

impl AssetRoot {
    pub fn to_record(&self) -> Vec<String> {
        let rec = AssetData::to_record(&self.data);
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetData {
    pub symbol: String,
    pub company_name: String,
    pub stock_type: String,
    pub exchange: String,
    pub is_nasdaq_listed: bool,
    pub is_nasdaq100: bool,
    pub is_held: bool,
    pub primary_data: PrimaryData,
    pub secondary_data: ::serde_json::Value,
    pub key_stats: KeyStats,
    pub market_status: String,
    pub asset_class: String,
}

impl AssetData {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec![
            self.symbol.to_string(),
            self.company_name.to_string(),
            self.stock_type.to_string(),
            self.exchange.to_string(),
            self.is_nasdaq_listed.to_string(),
            self.is_nasdaq100.to_string(),
            self.is_held.to_string(),
        ];
        rec.append(&mut PrimaryData::to_record(&self.primary_data));
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryData {
    pub last_sale_price: String,
    pub net_change: String,
    pub percentage_change: String,
    pub delta_indicator: String,
    pub last_trade_timestamp: String,
    pub is_real_time: bool,
}

impl PrimaryData {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec![
            self.last_trade_timestamp.to_string(),
            self.last_sale_price.to_string(),
            self.net_change.to_string(),
            self.percentage_change.to_string(),
            self.is_real_time.to_string(),
            self.delta_indicator.to_string(),
        ];
        return rec;
    }
}

pub const NDAQ_QUOTE_HEADER: [&'static str; 13] = [
    "symbol",
    "company_name",
    "stock_type",
    "exchange",
    "is_nasdaq_listed",
    "is_nasdaq100",
    "is_held",
    "last_trade_timestamp",
    "last_sale_price",
    "net_change",
    "percentage_change",
    "is_real_time",
    "delta_indicator",
];

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryData {
    pub last_sale_price: String,
    pub net_change: String,
    pub percentage_change: String,
    pub delta_indicator: String,
    pub last_trade_timestamp: String,
    pub is_real_time: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyStats {
    #[serde(rename = "Volume")]
    pub volume: AssetItem,
    #[serde(rename = "PreviousClose")]
    pub previous_close: AssetItem,
    #[serde(rename = "OpenPrice")]
    pub open_price: AssetItem,
    #[serde(rename = "MarketCap")]
    pub market_cap: AssetItem,
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


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetItem {
    pub label: String,
    pub value: String,
}




#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionRoot {
    pub data: OptionData,
    pub message: ::serde_json::Value,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionData {
    pub total_record: i64,
    pub last_trade: String,
    pub option_chain_list: OptionChainList,
    pub month_filter: Vec<MonthFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainList {
    pub headers: OptionHeaders,
    pub rows: Vec<OptionRow>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionHeaders {
    pub call: OptionHeader,
    pub put: OptionHeader,
    pub other_or_common: OtherOrCommon,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionHeader {
    pub last: String,
    pub change: String,
    pub bid: String,
    pub ask: String,
    pub volume: String,
    pub openinterest: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherOrCommon {
    pub strike: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionRow {
    pub call: OptionRow2,
    pub put: OptionRow2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionRow2 {
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


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthFilter {
    pub month: String,
    pub dates: Vec<AssetItem>,
}
