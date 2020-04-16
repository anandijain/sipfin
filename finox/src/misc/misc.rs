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

// pub fn hs_and_st() -> Result<(), reqwest::Error> {
//     let url = "https://comtrade.un.org/Data/cache/classificationST.json";
//     let write_fn = "st.csv";
//     //  "https://comtrade.un.org/Data/cache/classificationST.json"];
//     // for url in urls.iter() {
//     if let Ok(body) = getters::simple_get(url.to_string()) {
//         let res: uncomtrade::ResMeta = serde_json::from_str(&body.to_string()).unwrap();
//         let recs = uncomtrade::ResMeta::to_records(&res);
//         writerecs(write_fn.to_string(), &["id", "text", "parent"], recs);
//     }
//     Ok(())
// }


// pub fn currencies_intraday(start: String) -> Result<(), reqwest::Error> {
//     let index = CURRENCY_SYMBOLS
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &CURRENCY_SYMBOLS[index..CURRENCY_SYMBOLS.len()];
//     for s1 in todo_symbs.iter() {
//         for s2 in CURRENCY_SYMBOLS.iter() {
//             if s1 == s2 {
//                 continue;
//             }
//             let symb = format!("{}{}%3ACUR", s1.to_string(), s2.to_string());
//             if let Some(curs) = getters::get_intraday(symb.to_string()) {
//                 let prices_fn = format!("./data/{}_intraday_prices.csv", symb.to_string());
//                 if let Ok(recs) = types::Intraday::price_records(&curs[0]) {
//                     writerecs(prices_fn, &["date_time", &curs[0].ticker.to_string()], recs);
//                 }
//             } else {
//                 println!("currency route missing: {}", symb.to_string());
//                 continue;
//             }
//         }
//     }
//     Ok(())
// }

// pub fn currencies_history(start: String) -> Result<(), reqwest::Error> {
//     let index = CURRENCY_SYMBOLS
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &CURRENCY_SYMBOLS[index..CURRENCY_SYMBOLS.len()];
//     for s1 in todo_symbs.iter() {
//         for s2 in CURRENCY_SYMBOLS.iter() {
//             if s1 == s2 {
//                 continue;
//             }
//             let symb = format!("{}{}%3ACUR", s1.to_string(), s2.to_string());
//             if let Some(curs) = getters::get_history(symb.to_string()) {
//                 let prices_fn = format!("./data/{}_history_prices.csv", symb.to_string());
//                 if let Ok(recs) = types::Intraday::price_records(&curs[0]) {
//                     writerecs(prices_fn, &["date_time", &curs[0].ticker.to_string()], recs);
//                 }
//             } else {
//                 println!("currency route missing: {}", symb.to_string());
//                 continue;
//             }
//         }
//     }
//     Ok(())
// }

// pub fn commodities_prices(start: String) -> Result<(), reqwest::Error> {
//     let index = COMMODITIES_SYMBOLS
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &COMMODITIES_SYMBOLS[index..COMMODITIES_SYMBOLS.len()];
//     for s in todo_symbs.iter() {
//         if let Some(hist) = getters::get_history(format!("{}%3ACOM", s.to_string())) {
//             if let Ok(prices) = types::Intraday::price_records(&hist[0]) {
//                 let prices_fn = format!("./data/{}_prices.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 writerecs(prices_fn, &["date_time", &price_col], prices);
//             }
//             if let Ok(volume) = types::Intraday::volume_records(&hist[0]) {
//                 let volume_fn = format!("./data/{}_volume.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 writerecs(volume_fn, &["date_time", &vol_col], volume);
//             }
//         }
//     }
//     Ok(())
// }

// pub fn commodities_intraday() -> Result<(), reqwest::Error> {
//     // let index = COMMODITIES_SYMBOLS
//     //     .iter()
//     //     .position(|r| r.to_string() == start.to_string())
//     //     .unwrap();

//     // let todo_symbs = &COMMODITIES_SYMBOLS[index..COMMODITIES_SYMBOLS.len()];
//     for s in COMMODITIES_SYMBOLS.iter() {
//         if let Some(hist) = getters::get_history(format!("{}%3ACOM", s.to_string())) {
//             if let Ok(prices) = types::Intraday::price_records(&hist[0]) {
//                 let prices_fn = format!("./data/{}_intraday_prices.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 writerecs(prices_fn, &["date_time", &price_col], prices);
//             }
//             if let Ok(volume) = types::Intraday::volume_records(&hist[0]) {
//                 let volume_fn = format!("./data/{}_intraday_volume.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 writerecs(volume_fn, &["date_time", &vol_col], volume);
//             }
//         }
//     }
//     Ok(())
// }

// pub fn news() -> Result<(), csv::Error> {
//     let write_fn = "./data/news.csv";
//     let mut wtr = csv::Writer::from_path(&write_fn)?;
//     wtr.write_record(&NEWS_HEADER);
//     for s in NEWS_SYMBOLS.iter() {
//         if let Some(news_vec) = getters::get_news(s.to_string()) {
//             if let Ok(recs) = news::NewsVec::to_records(&news_vec) {
//                 for r in recs.iter() {
//                     wtr.write_record(r);
//                 }
//             }
//         }
//     }
//     Ok(())
// }

// pub fn sp500(start: String, write_header: bool) -> Result<(), csv::Error> {
//     let symbs = read_tickers("./data/sp500tickers.txt");
//     let index = symbs
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &symbs[index..symbs.len()];

//     let headlines_fn = "./data/sp500_headlines.csv".to_string();
//     let metadata_fn = "./data/sp500.csv".to_string();
//     let mut meta_wtr = csv::Writer::from_path(&metadata_fn)?;
//     let mut lines_wtr = csv::Writer::from_path(&headlines_fn)?;
//     meta_wtr.write_record(&STOCK_HEADER);
//     lines_wtr.write_record(&HEADLINES_HEADER);
//     for s in todo_symbs.iter() {
//         let symb = format!("{}%3AUS", s.to_string());
//         if let Some(c) = getters::get_datastrip(symb.to_string()) {
//             if let Ok(headlines) = types::Root::to_headlines(&c[0]) {
//                 for r in headlines.iter() {
//                     lines_wtr.write_record(r);
//                 }
//             }
//             let metadata_record = types::Root::to_record(&c[0]);
//             meta_wtr.write_record(&metadata_record);
//         }
//     }
//     meta_wtr.flush();
//     lines_wtr.flush();
//     Ok(())
// }

// pub fn stock_prices(start: String) -> Result<(), reqwest::Error> {
//     let symbs = read_tickers("./data/sp500tickers.txt");
//     let index = symbs
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &symbs[index..symbs.len()];
//     for s in todo_symbs.iter() {
//         if let Some(hist) = getters::get_history(format!("{}%3AUS", s.to_string())) {
//             if let Ok(recs) = types::Intraday::price_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_history_price.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 writerecs(write_fn, &["date_time", &price_col], recs);
//             }
//             if let Ok(recs) = types::Intraday::volume_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_history_vol.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 writerecs(write_fn, &["date_time", &vol_col], recs);
//             }
//         }
//     }
//     Ok(())
// }
// pub fn stock_intraday(start: String) -> Result<(), reqwest::Error> {
//     let symbs = read_tickers("./data/sp500tickers.txt");
//     let index = symbs
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &symbs[index..symbs.len()];
//     for s in todo_symbs.iter() {
//         let symb = format!("{}%3AUS", s.to_string());
//         if let Some(hist) = getters::get_intraday(symb.to_string()) {
//             if let Ok(recs) = types::Intraday::price_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_intraday_price.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 writerecs(write_fn, &["date_time", &price_col], recs);
//             }
//             if let Ok(recs) = types::Intraday::volume_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_intraday_vol.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 writerecs(write_fn, &["date_time", &vol_col], recs);
//             }
//         }
//     }
//     Ok(())
// }

// match 
// US(s) => vec![s, "%3AUS"].join(""),
// F(s) => vec![s, "%3ACOM"].join("")

// pub fn bloomberg_url(s: Security) -> String {
//     let root = "https://www.bloomberg.com/";
//     let prefix = "markets2/api/datastrip/";

//     let intra_prefix = "/markets2/api/intraday/";
//     let intra_sfx = "?days=10&interval=0&volumeInterval=0";
    
//     let hist_prefix = "markets2/api/history/";
//     "&limit=1000"
//     let news_prefix"/markets/api/comparison/news?securityType="
//     let news_sfx "/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily"

//     match s {
//         Security::F(s) => vec![root, &s, "=F?symbol=", &s, sfx].join(""),
//         Security::X(s) => vec![root, &s, "=X?symbol=", &s, sfx].join(""),
//         Security::US(s) => vec![root, &s, "?region=US", sfx].join(""),
//     }
// }

// pub fn get_datastrip(t: String) -> Option<Vec<types::Root>> {
//     let url = [
        
//         &t,
//         "%3AUS",
    
//     if let Ok(body) = simple_get(url) {
//         let company: Vec<types::Root> = serde_json::from_str(&body.to_string()).unwrap();
//         if company != vec![] {
//             return Some(company);
//         }
//     }
//     None
// }

// pub fn get_intraday(t: String) -> Option<Vec<types::Intraday>> {
//     if let Ok(body) = simple_get(url) {
//         let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
//         if cur != vec![] {
//             return Some(cur);
//         }
//     }
//     None
// }

// pub fn get_history(t: String) -> Option<Vec<types::Intraday>> {
//     if let Ok(body) = simple_get(url) {
//         let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
//         if cur != vec![] {
//             return Some(cur);
//         }
//     }
//     None
// }

// pub fn get_news(t: String) -> Option<news::NewsVec> {

//     if let Ok(body) = simple_get(url) {
//         let cur: news::NewsVec = serde_json::from_str(&body.to_string()).unwrap();
//         if cur.news != vec![] {
//             return Some(cur);
//         }
//     }
//     None
// }