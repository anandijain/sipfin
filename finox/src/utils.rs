extern crate csv;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader},
    path::Path,
};

use crate::getters;
use crate::types;
use crate::news;

pub fn writerecs(
    file_name: String,
    header: &[&str],
    records: Vec<csv::StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_name.to_string())?;
    wtr.write_record(header);
    for r in records.iter() {
        wtr.write_record(r);
    }
    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn currencies() -> Result<(), reqwest::Error> {
    for s in currency_symbols.iter() {
        if let Ok(curs) = getters::get_currency(s.to_string()) {
            let write_fn = format!("./data/USD{}.csv", s.to_string());
            if let Ok(recs) = types::Intraday::to_records(&curs[0]) {
                writerecs(write_fn, &["date_time", &curs[0].ticker.to_string()], recs);
            }
        }
    }
    Ok(())
}

// pub fn sp500(start: String, write_header: bool) -> Result<(), csv::Error> {
//     let symbs = read_tickers("./data/sp500tickers.txt");
//     let index = symbs
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &symbs[index..symbs.len()];

//     let headlines_fn = "./data/sp500_headlines.csv".to_string();
//     let metadata_fn = "./data/sp500.csv".to_string();

//     if write_header {
//         let mut meta_wtr = csv::Writer::from_path(&metadata_fn)?;
//         let mut lines_wtr = csv::Writer::from_path(&headlines_fn)?;
//         meta_wtr.write_record(&stock_header);
//         lines_wtr.write_record(&headlines_header);
//     }
//     // else {
//     //     let lines_append = match OpenOptions::new().append(true).open(headlines_fn) {
//     //         Ok(mut f) => f,
//     //         Err(e) => panic!("Could not open the file for appendin: {:?}", e),
//     //     };
//     //     let meta_append = match OpenOptions::new().append(true).open(metadata_fn) {
//     //         Ok(mut f) => f,
//     //         Err(e) => panic!("Could not open the file for appendin: {:?}", e),
//     //     };
//     //     let mut meta_wtr = csv::Writer::from_writer(&meta_append);
//     //     let mut lines_wtr = csv::Writer::from_writer(&lines_append);
//     // }

//     for s in todo_symbs.iter() {
//         if let Ok(c) = getters::get_datastrip(s.to_string()) {
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

pub fn news() -> Result<(), csv::Error> {
    let write_fn = "./data/news.csv";
    let mut wtr = csv::Writer::from_path(&write_fn)?;
    wtr.write_record(&news_header);
    for s in news_symbols.iter(){
        if let Ok(news_vec) = getters::get_news(s.to_string()) {
            if let Ok(recs) = news::NewsVec::to_records(&news_vec) {
                for r in recs.iter() {
                    wtr.write_record(r);
                }
            }
        }
    }
    Ok(())
}

//todo
// fn dash() -> Result<(), reqwest::Error> {
//     let dashboard_root = "https://www.bloomberg.com/markets/api/data-dashboard/tileset/";
// https://www.bloomberg.com/markets2/api/report/income/EQT/MSFT%3AUS/annual?locale=en&currency=USD
// https://www.bloomberg.com/markets/api/security/currency/cross-rates/USD,EUR
// https://www.bloomberg.com/markets2/api/people/2029055
// https://www.bloomberg.com/markets2/api/peopleForCompany/101743
// https://www.bloomberg.com/markets/api/sectors/S5INFT%3AIND?locale=en
// https://www.bloomberg.com/markets2/api/history/MSFT%3AUS/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily
// https://www.bloomberg.com/markets2/api/history/CL1%3ACOM/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily
// https://www.bloomberg.com/markets/api/comparison/news?securityType=GOVERNMENT_BOND&limit=1000&locale=en
// anotha SP1:IND,DM1:IND,SX5E:IND,UKX:IND,DAX:IND,NKY:IND,SHCOMP:IND,SPX:IND,RTY:IND,DXY:CUR,USDJPY:CUR,EURUSD:CUR,XAU:CUR,USGG10YR:IND,USGG2YR:IND,LEGATRUU:IND,CL1:COM,CO1:COM
//     let symbs = vec!["commodities", "futures", "asia", "americas", "europe"];

// }
pub const stock_header: [&'static str; 15] = [
    "id",
    "short_name",
    "market_cap",
    "co_phone",
    "last_update",
    "average_volume30_day",
    "price",
    "open_price",
    "high_price",
    "low_price",
    "low_price52_week",
    "high_price52_week",
    "number_of_employees",
    "price_earnings_ratio",
    "shares_outstanding",
];

pub const currency_symbols: [&'static str; 35] = [
    "EUR", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN", "BRL", "CLP", "COP", "PEN", "CRC",
    "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF", "RUB", "TRY", "ILS", "KES", "ZAR",
    "MAD", "NZD", "PHP", "SGD", "IDR", "CNY", "INR", "MYR", "THB",
];

pub const news_symbols: [&'static str; 5] = [
"GOVERNMENT_BOND", "COMMODITY", "COMMON_STOCK", "CURRENCY", "BLOOMBERG_BARCLAYS_INDEX"
];

pub const news_header: [&'static str; 3] = [
"url", "headline", "date_time"
];

pub const headlines_header: [&'static str; 4] = ["id", "url", "headline", "lastmod"];



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