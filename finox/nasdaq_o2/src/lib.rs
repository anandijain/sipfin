
// use crate::models::{NewQuote, Quote};
// use nasdaq::gen::{HasRec, HasRecs};
// use crate::main;
//extern crate nasdaq_o2;
extern crate percent_encoding;
use futures::stream::StreamExt;
mod nasdaq;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use nasdaq::gen::HasRecs;
// mod models;
// mod schema;

use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub async fn lil_fetchvv(
    urls: Vec<String>,
) -> Vec<Vec<String>> {
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.json::<nasdaq::RealtimeRoot>().await {
                return Some(root.to_recs());
            }
            println!("serialized json wrong {}", url.clone());
            return None;
        }
        println!("no good1");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<Vec<String>>>>>()
    .await;
    let recs: Vec<Vec<String>> = fetches
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<Vec<String>>>>()
        .into_iter()
        .flatten()
        .collect();
    return recs;
}

pub async fn lil_fetchv(urls: Vec<String>) -> Vec<Vec<String>> {
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.json::<nasdaq::InfoRoot>().await {
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

pub fn write_csv(
    filename: &str,
    data: Vec<Vec<String>>,
    header: Vec<String>,
) -> Result<(), csv::Error> {
    let path = Path::new(filename);
    let mut wtr = csv::Writer::from_path(path).expect(format!("whtf csv {}", filename).as_ref());
    wtr.write_record(header.clone())?;
    wtr.flush()?;
    let len = header.len();
    for row in data.iter() {
        assert_eq!(len, row.len()); // perf hit?
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn epoch_str() -> String {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string();
    return t;
}

pub fn ndaq_url_to_ticker(url: String) -> String {
    let v: Vec<&str> = url.split("/").collect(); // divs
    return format!("{}_insider", v[5]);
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Security {
    Commodity(String),
    Stock(String), // ? might need special treatment, far more endpoints for these
    Currency(String),
}

impl Security {
    pub fn to_nasdaq_url(&self, sfx: &str) -> String {
        // "insider-trades", historical "option-chain", "chart", "info", "dividends", realtime-trades
        let pre = "quote";
        match self {
            Security::Commodity(s) => garbo(pre, s, sfx, "commodities", ""),
            Security::Stock(s) => garbo(pre, s, sfx, "stocks", ""),
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
pub fn gen_secs(args: Vec<String>) -> Vec<Security> {
    let securities: Vec<Security> = match args[1].as_str() {
        "stocks" => Ok(
            read_tickers("/home/sippycups/sipfin/finox/ref_data/tickers.txt")
                .iter()
                .map(|x| Security::Stock(x.to_string()))
                .collect::<Vec<Security>>(),
        ),
        "commodities" => Ok(
            read_tickers("/home/sippycups/sipfin/finox/ref_data/tickers_commodities.txt")
                .iter()
                .map(|x| Security::Commodity(utf8_percent_encode(x, NON_ALPHANUMERIC).to_string()))
                .collect::<Vec<Security>>(),
        ),
        "currencies" => Ok(
            read_tickers("/home/sippycups/sipfin/finox/ref_data/tickers_currencies.txt")
                .iter()
                .map(|x| Security::Currency(x.to_string()))
                .collect::<Vec<Security>>(),
        ),
        _ => Err("invalid asset class provided"),
    }
    .unwrap();
    return securities;
}
