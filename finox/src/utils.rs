extern crate chrono;
extern crate csv;
use chrono::{Datelike, Timelike, Utc};

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

use crate::getters;
use crate::news;
use crate::types;
use crate::yf;

// // IND COM CUR US GOV
// pub enum Security {
//     IND(String),
//     COM(String),
//     CUR(String),
//     US(String),
//     GOV(String),
// }

pub enum Security {
    F(String),
    X(String),
    US(String),
}

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

pub fn simppath(s: String, sfx: String) -> String {
    //sfx enum x, f, us
    let now = Utc::now();
    return format!(
        "./data/{}:{}_{}_{}_{}.csv",
        s.to_string(),
        sfx.to_string(),
        now.year(),
        now.month(),
        now.day()
    );
}


pub fn chart_headers(s: String) -> Vec<String> {
    let mut headers: Vec<String> = vec!["t".to_string()];

    for elt in YF_HEADER[1..YF_HEADER.len()].iter() {
        headers.push(format!("{}_{}", elt.to_string(), s.to_string()));
    }
    return headers;
}

pub fn write_yf(url: String) -> Result(() -> csv::Error) {
    if let Some(recs) = yf_symb(url.to_string()) {
        if let Ok(mut wtr) = csv::Writer::from_path(simppath(s.to_string(), "F".to_string())) {
            let headers = chart_headers(s.to_string());
            wtr.write_record(headers);
            for r in recs.iter() {
                wtr.write_record(r);
            }
            wtr.flush();
        }
    }
    Ok(())
}

pub fn yf_url(s: Security) -> String {
    let root = "https://query1.finance.yahoo.com/v8/finance/chart/";
    let sfx = "&range=7d&interval=1m";
    match s {
        Security::F(s) => vec![root, &s, "=F?symbol=", &s, sfx].join(""),
        Security::X(s) => vec![root, &s, "=X?symbol=", &s, sfx].join(""),
        Security::US(s) => vec![root, &s, "?region=US", sfx].join(""),
    }
}

pub fn yf_symb(url: String) -> Option<Vec<csv::StringRecord>> {
    if let Some(tohlcv) = getters::yf_from_url(url.to_string()) {
        let mut recs: Vec<csv::StringRecord> = tohlcv
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        return Some(recs);
    }
    return None;
}

pub fn yf_US() -> Result<(), reqwest::Error> {
    let symbs = read_tickers("./data/sp500tickers_yf.txt");
    for s in symbs.iter() {
        let url = yf_url(Security::US(s.to_string()));
        if let Some(recs) = yf_symb(url.to_string()) {
            writerecs(simppath(s.to_string(), "US".to_string()), &YF_HEADER, recs);
        }
    }
    Ok(())
}

pub fn yf_X() -> Result<(), reqwest::Error> {
    for s1 in CURRENCY_SYMBOLS_YF.iter() {
        for s2 in CURRENCY_SYMBOLS_YF.iter() {
            if s1 == s2 {
                continue;
            }
            let symb = format!("{}{}", s1.to_string(), s2.to_string());
            let url = yf_url(Security::X(symb.clone()));
            if let Some(recs) = yf_symb(url.to_string()) {
                writerecs(
                    simppath(symb.to_string(), "X".to_string()),
                    &YF_HEADER,
                    recs,
                );
            }
        }
    }
    Ok(())
}

pub fn yf_F() -> Result<(), reqwest::Error> {
    for s in COMMODITIES_SYMBOLS_YF.iter() {
        let url = yf_url(Security::F(s.to_string()));
        return write_yf(url);
    }
    Ok(())
}

pub const STOCK_HEADER: [&'static str; 15] = [
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

pub const CURRENCY_SYMBOLS: [&'static str; 40] = [
    "USD", "EUR", "XAU", "XAG", "XPT", "XPD", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN",
    "BRL", "CLP", "COP", "PEN", "CRC", "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF",
    "RUB", "TRY", "ILS", "KES", "ZAR", "MAD", "NZD", "PHP", "SGD", "IDR", "CNY", "INR", "MYR",
    "THB",
];
pub const CURRENCY_SYMBOLS_YF: [&'static str; 6] = ["USD", "EUR", "JPY", "GBP", "AUD", "CAD"];

pub const NEWS_SYMBOLS: [&'static str; 5] = [
    "GOVERNMENT_BOND",
    "COMMODITY",
    "COMMON_STOCK",
    "CURRENCY",
    "BLOOMBERG_BARCLAYS_INDEX",
];

pub const COMMODITIES_SYMBOLS: [&'static str; 37] = [
    "CO1", "CL1", "XB1", "NG1", "HO1", "GC1", "SI1", "HG1", "C%201", "W%201", "CC1", "CT1", "LC1",
    "QS1", "JX1", "MO1", "JG1", "LMCADS03", "LMAHDS03", "LMZSDS03", "LMSNDS03", "O%201", "RR1",
    "S%201", "SM1", "BO1", "RS1", "KC1", "SB1", "JO1", "CT1", "OL1", "LB1", "JN1", "DL1", "FC1",
    "LH1",
];
pub const COMMODITIES_SYMBOLS_YF: [&'static str; 23] = [
    "ES", "YM", "NQ", "RTY", "ZB", "ZN", "ZF", "ZT", "GC", "SI", "HG", "PA", "CL", "HO", "NG",
    "RB", "BZ", "C", "KW", "SM", "BO", "S", "CT",
];

pub const NEWS_HEADER: [&'static str; 3] = ["url", "headline", "date_time"];

pub const HEADLINES_HEADER: [&'static str; 4] = ["id", "url", "headline", "lastmod"];

pub const YF_HEADER: [&'static str; 6] = ["t", "o", "h", "l", "c", "v"];
