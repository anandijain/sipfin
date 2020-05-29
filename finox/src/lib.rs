pub mod cme;
pub mod headers;
pub mod keys;
pub mod nasdaq;
pub mod news;
pub mod yf;

extern crate roses;

use crate::nasdaq::realtime::RealtimeRoot;
use chrono::{DateTime, FixedOffset, Utc};
use futures::stream::StreamExt;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::{collections::HashMap, error::Error, fmt, path::Path, time::Duration};

pub trait HasRecs {
    fn to_recs(&self) -> Vec<Vec<String>>;
}

pub const NYT_DELAY: Duration = Duration::from_millis(6000);

pub async fn fetch<'a, T: ?Sized>(urls: Vec<String>) -> Result<Vec<Vec<String>>, String>
where
    for<'de> T: HasRecs + serde::Deserialize<'de> + 'a,
{
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            if let Ok(root) = res.json::<T>().await {
                return Some(root.to_recs());
            } else {
                println!("serialize err {}", url.clone());
                return None;
            }
        }
        println!("response err: {}", url.clone());
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<Vec<String>>>>>()
    .await;
    let recs = fetches
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<Vec<String>>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<String>>>();
    //Ok(Box::new(fetches.into_iter().flatten().collect::<Vec<T>>()))
    Ok(recs)
}

// when endpoints dont grab a vec
//pub async fn fetch_rec(urls: Vec<String>) -> Vec<Vec<String>> {
//    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
//        if let Ok(res) = reqwest::get(&url.clone()).await {
//            if let Ok(root) = res.json::<crate::nasdaq::info::InfoRoot>().await {
//                return Some(root.to_rec());
//            }
//            println!("serialized json wrong {}", url.clone());
//            return None;
//        }
//        println!("no good1");
//        return None;
//    }))
//    .buffer_unordered(16)
//    .collect::<Vec<Option<Vec<String>>>>()
//    .await;
//    let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect();
//    return recs;
//}

pub fn gsnews() -> Result<(), reqwest::Error> {
    let url = "https://www.goldmansachs.com/insights/insights-articles.json";
    if let Ok(body) = roses::simple_get(url.to_string()) {
        let root: news::gs::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = news::gs::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        roses::writerecs("./gsnews.csv".to_string(), &headers::GS_HEADER, recs)
            .expect("csv problem");
    }
    Ok(())
}
pub fn nyt_archive_urls() -> Vec<String> {
    let mut urls = vec![];
    for i in 1853..2019 {
        for j in 1..=12 {
            let url = format!(
                "https://api.nytimes.com/svc/archive/v1/{}/{}.json?api-key={}",
                i,
                j,
                crate::keys::NYT_KEY.to_string()
            );
            urls.push(url);
        }
    }
    urls
}

pub async fn fetch_rt(
    hm: HashMap<Security, DateTime<FixedOffset>>,
) -> HashMap<Security, DateTime<FixedOffset>> {
    let fetches = futures::stream::iter(hm.into_iter().map(|pair| async move {
        if let Ok(res) = reqwest::get(&pair.0.to_nasdaq_rt_url().unwrap()).await {
            if let Ok(root) = res.json::<RealtimeRoot>().await {
                if let (Some(recs), newt) = root.to_recs(pair.1) {
                    let file_name = format!(
                        "../data/nasdaq/realtime-trades/{}_{}.csv",
                        pair.0,
                        Utc::now().to_rfc3339()
                    );
                    let fp = Path::new(&file_name);
                    //TODO append to file for specific ticker
                    roses::write_csv(&fp, recs, &crate::nasdaq::realtime::NDAQ_REALTIME_HEADER)
                        .expect("csv error");
                    println!("{:#?}", &pair.0.to_nasdaq_rt_url().unwrap());
                    return (pair.0, newt);
                } else {
                    return pair;
                }
            } else {
                println!("serialize err {:#?}", pair.clone());
                println!("{:#?}", &pair.0.to_nasdaq_rt_url().unwrap());
                //return (pair.0, pair.1);
                return pair;
            }
        }
        println!("response err: {:#?}", pair.clone());
        println!("{:#?}", &pair.0.to_nasdaq_rt_url().unwrap());
        return pair;
    }))
    .buffer_unordered(16)
    .collect::<HashMap<Security, DateTime<FixedOffset>>>()
    .await;
    //println!("fetches: {:#?}", fetches);
    return fetches;
}

pub fn ndaq_url_to_ticker(url: String) -> String {
    let v: Vec<&str> = url.split("/").collect(); // divs
    return format!("{}", v[5]);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde_derive::Serialize, serde_derive::Deserialize)]
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

impl fmt::Display for Security {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mystr = match self {
            Security::Commodity(s) => s,
            Security::Stock(s) => s,
            Security::Currency(s) => s,
            Security::Etf(s) => s,
        };
        write!(f, "{}", mystr)
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
        "stocks" => Ok(roses::read_tickers("../ref_data/tickers_stocks.txt")
            .iter()
            .map(|x| Security::Stock(x.to_string()))
            .collect::<Vec<Security>>()),
        "commodities" => Ok(roses::read_tickers("../ref_data/tickers_commodities.txt")
            .iter()
            .map(|x| Security::Commodity(utf8_percent_encode(x, NON_ALPHANUMERIC).to_string()))
            .collect::<Vec<Security>>()),
        "currencies" => Ok(roses::read_tickers("../ref_data/tickers_currencies.txt")
            .iter()
            .map(|x| Security::Currency(x.to_string()))
            .collect::<Vec<Security>>()),
        "etf" => Ok(roses::read_tickers("../ref_data/tickers_stocks.txt")
            .iter()
            .map(|x| Security::Etf(x.to_string()))
            .collect::<Vec<Security>>()),

        _ => Err("invalid asset class provided"),
    }
    .unwrap();
    return securities;
}

pub fn nls_to_dt(s: &str) -> Result<DateTime<FixedOffset>, chrono::ParseError> {
    let t = format!("{} {} +05:00", Utc::now().format("%Y-%m-%d"), s);
    return DateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S %z");
}
