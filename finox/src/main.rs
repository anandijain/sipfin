extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::error::Error;
use std::fs::OpenOptions;
use std::{thread, time};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod getters;
mod types;
mod utils;


fn main() -> Result<(), reqwest::Error> {
    currencies()
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

fn sp500() -> Result<(), reqwest::Error> {
    let stock_header: Vec<String> = vec![
        "id".to_string(),
        "short_name".to_string(),
        "market_cap".to_string(),
        "co_phone".to_string(),
        "last_update".to_string(),
        "average_volume30_day".to_string(),
        "price".to_string(),
        "open_price".to_string(),
        "high_price".to_string(),
        "low_price".to_string(),
        "low_price52_week".to_string(),
        "high_price52_week".to_string(),
        "number_of_employees".to_string(),
        "price_earnings_ratio".to_string(),
        "shares_outstanding".to_string(),
    ];
    let write_fn = "./data/sp500.csv".to_string();
    let symbs = utils::read_tickers("./data/sp500tickers.txt");
    getters::get_csv(
        getters::company_to_rec,
        stock_header,
        symbs.to_vec(),
        write_fn,
        1000,
    );
    Ok(())
}

fn sp500_headlines() -> Result<(), reqwest::Error> {
    let headlines_header: Vec<String> = vec![
        "id".to_string(),
        "url".to_string(),
        "headline".to_string(),
        "lastmod".to_string(),
    ];
    let write_fn = "./data/sp500_headlines.csv".to_string();
    let symbs = utils::read_tickers("./data/sp500tickers.txt");
    // let index = symbs.iter().position(|r| r.to_string() == "COF").unwrap();
    // let todo_symbs = &symbs[index..symbs.len()];
    getters::get_csv(
        getters::headlines_to_rec,
        headlines_header,
        symbs.to_vec(),
        write_fn,
        1000,
    );
    Ok(())
}

fn currencies() -> Result<(), reqwest::Error> {
    //"VEF",
    let symbs_cur = vec![
        "EUR", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN", "BRL", "CLP", "COP", "PEN",  "CRC",
        "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF", "RUB", "TRY", "ILS", "KES", "ZAR",
        "MAD", "NZD", "PHP", "SGD", "IDR", "CNY", "INR", "MYR", "THB"
    ];

    for s in symbs_cur.iter() {
        let write_fn = format!("./data/test_USD{}.csv", s.to_string());
        getters::get_csv(
            getters::cur_to_rec,
            vec!["date_time".to_string(), format!("USD{}", s.to_string())],
            vec![s.to_string()],
            write_fn,
            1000,
        );
    }
    Ok(())
}
