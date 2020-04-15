/*
todo
fn dash() -> Result<(), reqwest::Error> {
let symbs = vec!["commodities", "futures", "asia", "americas", "europe"];
let dashboard_root = "https://www.bloomberg.com/markets/api/data-dashboard/tileset/";
https://www.bloomberg.com/markets2/api/report/income/EQT/MSFT%3AUS/annual?locale=en&currency=USD
https://www.bloomberg.com/markets/api/security/currency/cross-rates/USD,EUR
https://www.bloomberg.com/markets2/api/people/2029055
https://www.bloomberg.com/markets2/api/peopleForCompany/101743
https://www.bloomberg.com/markets/api/sectors/S5INFT%3AIND?locale=en
https://www.bloomberg.com/markets2/api/history/MSFT%3AUS/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily
https://www.bloomberg.com/markets2/api/history/CL1%3ACOM/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily
https://www.bloomberg.com/markets/api/comparison/news?securityType=GOVERNMENT_BOND&limit=1000&locale=en
anotha SP1:IND,DM1:IND,SX5E:IND,UKX:IND,DAX:IND,NKY:IND,SHCOMP:IND,SPX:IND,RTY:IND,DXY:CUR,USDJPY:CUR,EURUSD:CUR,XAU:CUR,USGG10YR:IND,USGG2YR:IND,LEGATRUU:IND,CL1:COM,CO1:COM
https://www.bloomberg.com/bbg-gfx/bgreen-widget-data/dashboard-data.json
https://oec.world/en/profile/country/arg/
https://api.nasdaq.com/api/quote/watchlist?symbol=cl%3anmx%7ccommodities&symbol=ho%3anmx%7ccommodities&symbol=rb%3anmx%7ccommodities&symbol=ng%3anmx%7ccommodities&symbol=bz%3anmx%7ccommodities&symbol=eh%7ccommodities

}


notes 
big currencies are winning
platinum is low
mxnpen
zarcrc
zarusd
arscop
cadchf
cadmxn
cadnok
cadzar
cadaud
usdcrc
usdidr
usdsek
usdtry
*/

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Collections {
//     pub field_data_collection: Vec<FieldDataCollection>,
// }

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FieldDataCollection {
//     pub id: String,
//     pub issued_currency: String,
//     pub long_name: String,
//     pub price: String,
//     pub price_change1_day: String,
//     pub percent_change1_day: String,
//     #[serde(rename = "tradingDayCloseUTC")]
//     pub trading_day_close_utc: String,
//     #[serde(rename = "lastUpdateUTC")]
//     pub last_update_utc: String,
//     #[serde(rename = "MEDIA_SECURITY_TYPE")]
//     pub media_security_type: String,
//     #[serde(rename = "MEDIA_SECURITY_SUBTYPE")]
//     pub media_security_subtype: String,
//     pub security_type: String,
//     pub short_name: String,
//     pub commodity_contract_date: String,
//     pub price_date: String,
//     pub last_update_time: String,
//     #[serde(rename = "lastUpdateISO")]
//     pub last_update_iso: String,
//     pub user_time_zone: String,
//     pub market_open: bool,
//     pub commodity_units: Option<String>,
// }