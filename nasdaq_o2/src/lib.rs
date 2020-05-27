extern crate percent_encoding;
extern crate serde;
use chrono::{DateTime, FixedOffset, Utc};
use futures::stream::StreamExt;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub mod nasdaq;
//use nasdaq::gen::HasRecs;
use crate::nasdaq::realtime::RealtimeRoot;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// special for rt
/*
 *
 * maybe want to iter over a Arc<Mutex<hashmap<Security, DT<FO>>>>.
 * i think i am going to revert from weighted indices to full loop, but implement much better
 * logging on activity.
 * prob need to change fixedoffset to utc*/

pub async fn fetch_rt(pairs: Vec<(String, DateTime<FixedOffset>)>) -> Vec<(Option<Vec<Vec<String>>>, DateTime<FixedOffset>)>
{
    let fetches = futures::stream::iter(pairs.into_iter().map(|pair| async move {
        if let Ok(res) = reqwest::get(&pair.0).await {
            if let Ok(root) = res.json::<RealtimeRoot>().await {
                return root.to_recs(pair.1);
            } else {
                println!("serialize err {:#?}", pair.clone());
                return (None, pair.1);
            }
        }
        println!("response err: {:#?}", pair.clone());
        return (None, pair.1);
    }))
    .buffer_unordered(16)
    .collect::<Vec<(Option<Vec<Vec<String>>>, DateTime<FixedOffset>)>>()
    .await;
    println!("fetches: {:#?}", fetches);
    return fetches;
}

// when endpoints dont grab a vec
pub async fn lil_fetchv(urls: Vec<String>) -> Vec<Vec<String>> {
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.json::<crate::nasdaq::info::InfoRoot>().await {
                return Some(root.to_rec());
            }
            println!("serialized json wrong {}", url.clone());
            return None;
        }
        println!("no good1");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<String>>>>()
    .await;
    let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect();
    return recs;
}

// should probably be generic and return a Vec<Security>
pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn ndaq_url_to_ticker(url: String) -> String {
    let v: Vec<&str> = url.split("/").collect(); // divs
    return format!("{}", v[5]);
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Security {
    Commodity(String),
    Stock(String), // ? might need special treatment, far more endpoints for these
    Currency(String),
    Etf(String),
}

impl Security {
    pub fn to_nasdaq_url(&self, sfx: &str) -> String {
        // "insider-trades", historical "option-chain", "chart", "info", "dividends", realtime-trades
        let pre = "quote";
        match self {
            Security::Commodity(s) => garbo(pre, s, sfx, "commodities", ""),
            Security::Stock(s) => garbo(
                pre,
                s,
                sfx,
                "stocks",
                "&todate=2025-11-30&fromdate=2020-05-19&limit=99999",
            ),
            Security::Etf(s) => garbo(
                pre,
                s,
                sfx,
                "etf",
                "&todate=2025-11-30&fromdate=2020-05-19&limit=99999",
            ),
            Security::Currency(s) => garbo(pre, s, sfx, "currencies", ""),
        }
    }

    // only have stocks on rt
    pub fn to_nasdaq_rt_url(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Security::Stock(s) => Ok(garbo("quote", s, "realtime-trades", "stocks", "&limit=100")),
            _ => panic!("Nasdaq only has realtime stock quotes".to_string()),
        }
    }
}

pub fn garbo(pre: &str, s: &str, sfx: &str, sfx2: &str, sfx3: &str) -> String {
    format!(
        "https://api.nasdaq.com/api/{}/{}/{}?assetclass={}{}",
        pre, s, sfx, sfx2, sfx3
    )
}

// fix and percent encoding
pub fn gen_secs(args: &Vec<String>) -> Vec<Security> {
    let securities: Vec<Security> = match args[1].as_str() {
        "stocks" => Ok(read_tickers("../ref_data/tickers_stocks.txt")
            .iter()
            .map(|x| Security::Stock(x.to_string()))
            .collect::<Vec<Security>>()),
        "commodities" => Ok(read_tickers("../ref_data/tickers_commodities.txt")
            .iter()
            .map(|x| Security::Commodity(utf8_percent_encode(x, NON_ALPHANUMERIC).to_string()))
            .collect::<Vec<Security>>()),
        "currencies" => Ok(read_tickers("../ref_data/tickers_currencies.txt")
            .iter()
            .map(|x| Security::Currency(x.to_string()))
            .collect::<Vec<Security>>()),
        "etf" => Ok(read_tickers("../ref_data/tickers_stocks.txt")
            .iter()
            .map(|x| Security::Etf(x.to_string()))
            .collect::<Vec<Security>>()),

        _ => Err("invalid asset class provided"),
    }
    .unwrap();
    return securities;
}

#[derive(Debug, serde::Deserialize)]
struct Record {
    symbol: String,
    weight: f64,
}

pub fn nls_to_dt(s: &str) -> Result<DateTime<FixedOffset>, chrono::ParseError> {
    let t = format!("{} {} +05:00", Utc::now().format("%Y-%m-%d"), s);
    return DateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S %z");
}

