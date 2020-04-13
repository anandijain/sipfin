extern crate csv;
use std::error::Error;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

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


pub const headlines_header: [&'static str; 4] = [
        "id",
        "url",
        "headline",
        "lastmod",
    ];

pub fn writerecs(file_name: String, header: &[&str], records: Vec<csv::StringRecord>) -> Result<(), Box<dyn Error>> {
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


//fix
// fn open_append() -> Result<(), reqwest::Error> {
//     let file_to_append = match OpenOptions::new().append(true).open(write_fn) {
//         Ok(mut f) => f,
//         Err(e) => panic!("Could not open the file for appendin: {:?}", e),
//     };
// }



//todo
// fn dash() -> Result<(), reqwest::Error> {
//     let dashboard_root = "https://www.bloomberg.com/markets/api/data-dashboard/tileset/";
//     let symbs = vec!["commodities", "futures", "asia", "americas", "europe"];

// }