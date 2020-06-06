pub mod cme;
pub mod headers;
pub mod keys;
pub mod nasdaq;
pub mod news;
pub mod roses;
pub mod sec;
pub mod yf;

use crate::nasdaq::realtime::RealtimeRoot;
use chrono::{DateTime, FixedOffset, Utc};
use futures::stream::StreamExt;
//use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::{collections::HashMap, error::Error, fmt, fs, path::Path, thread, time::Duration};

pub trait HasRecs {
    fn to_recs(&self) -> Vec<Vec<String>>;
}

pub trait HasRec {
    fn to_rec(&self) -> Vec<String>;
}

pub const NYT_DELAY: Duration = Duration::from_millis(6000);

pub async fn fetch<'a, T: ?Sized>(urls: Vec<String>) -> Result<Vec<Vec<String>>, String>
where
    for<'de> T: HasRecs + serde::Deserialize<'de> + 'a,
{
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            //REMOVE SLEEP
            thread::sleep(Duration::from_millis(100));
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
pub async fn fetch_one<'a, T: ?Sized>(urls: Vec<String>) -> Vec<Option<T>>
where
    for<'de> T: HasRec + serde::Deserialize<'de> + 'a,
{
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.json::<T>().await {
                return Some(root);
            }
            println!("serialized json wrong {}", url.clone());
            return None;
        }
        println!("no good1");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<T>>>()
    .await;
    return fetches;
}

pub async fn fetch_strings(urls: Vec<String>) -> Vec<Option<sec::SecFormHeader>> {
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.text().await {
                if let Some(header) = sec::sec_header(&root) {
                    if let Some(recs) = sec::sec_13f(&root) {
                        let realfn = url
                            .clone()
                            .split("/")
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>();

                        let file_name = format!(
                            "../data/sec/13f/{}.csv",
                            realfn.last()?.split(".").collect::<Vec<_>>().first()?
                        );

                        let mut wtr = csv::Writer::from_path(file_name.clone()).unwrap();

                        for rec in recs.iter() {
                            wtr.serialize(rec).unwrap();
                        }

                        println!("{}: {:#?}", file_name, recs.len());
                        wtr.flush().unwrap();
                        return Some(header);
                    }
                }
            }
            println!("serialized text wrong {}", url.clone());
            return None;
        }
        println!("no good1");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<sec::SecFormHeader>>>()
    .await;
    return fetches;
}

//
pub async fn fetch_write<'a, T: ?Sized>(
    hm: HashMap<String, String>,
    relpath: &str,
    header: &[&str],
) -> Result<Vec<String>, reqwest::Error>
where
    for<'de> T: HasRecs + serde::Deserialize<'de> + 'a,
{
    let fetches = futures::stream::iter(hm.into_iter().map(|pair| async move {
        if let Ok(res) = reqwest::get(&pair.1.clone()).await {
            //thread::sleep(Duration::from_millis(100));
            if let Ok(root) = res.json::<T>().await {
                let recs = root.to_recs();
                let file_name = format!("{}{}.csv", relpath.clone(), pair.0);
                let fp = Path::new(&file_name);
                if fp.exists() == true {
                    let f = fs::OpenOptions::new()
                        .append(true)
                        .open(fp)
                        .expect("opening file prob");
                    roses::to_csv(f, recs.clone(), None).expect("csv error");
                } else {
                    let f = fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(fp)
                        .expect("opening file prob");
                    roses::to_csv(f, recs.clone(), Some(header)).expect("csv error");
                }
                println!("{}: {}", pair.0, recs.len());
                return Some(pair.0);
            }
            println!("serialized json wrong {:#?}", pair.clone());
            return None;
        }
        println!("res err");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<String>>>()
    .await;
    //println("{:#?}", fetches
    Ok(fetches.into_iter().flatten().collect::<Vec<String>>())
}

pub async fn fetch_rt(
    hm: HashMap<Security, DateTime<FixedOffset>>,
) -> HashMap<Security, DateTime<FixedOffset>> {
    let fetches = futures::stream::iter(hm.into_iter().map(|pair| async move {
        if let Ok(res) = reqwest::get(&pair.0.to_nasdaq_rt_url().unwrap()).await {
            if let Ok(root) = res.json::<RealtimeRoot>().await {
                if let (Some(recs), newt) = root.to_new_recs(pair.1) {
                    let file_name = format!(
                        "../data/nasdaq/realtime-trades/{}.csv",
                        pair.0,
                        //Utc::now().to_rfc3339()
                    );
                    let fp = Path::new(&file_name);
                    if fp.exists() == true {
                        let f = fs::OpenOptions::new()
                            .append(true)
                            .open(fp)
                            .expect("opening file prob");
                        roses::to_csv(f, recs, None).expect("csv error");
                    } else {
                        let f = fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(fp)
                            .expect("opening file prob");
                        roses::to_csv(
                            f,
                            recs,
                            Some(&crate::nasdaq::realtime::NDAQ_REALTIME_HEADER),
                        )
                        .expect("csv error");
                    }
                    return (pair.0, newt);
                } else {
                    return pair;
                }
            } else {
                println!("serialize err {:#?}", pair.clone());
                return pair;
            }
        }
        println!("response err: {:#?}", pair.clone());
        return pair;
    }))
    .buffer_unordered(16)
    .collect::<HashMap<Security, DateTime<FixedOffset>>>()
    .await;
    return fetches;
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

    pub fn to_yf(&self) -> String {
        match self {
            Security::Stock(s) | Security::Etf(s) => format!("https://query2.finance.yahoo.com/v8/finance/chart/{}?interval=1d&period1=0&period2=1590498425", s),
            Security::Currency(s) => format!("https://query2.finance.yahoo.com/v8/finance/chart/{}=X?interval=1d&period1=0&period2=1590498425", s),
            Security::Commodity(s) => format!("https://query2.finance.yahoo.com/v8/finance/chart/{}=F?interval=1d&period1=0&period2=1590498425", s),
            //_ => panic!("others not supported")
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
pub fn gen_secs(asset_class: &str) -> (Vec<Security>, &[&str]) {
    match asset_class {
        "stocks" => (
            roses::read_tickers("../ref_data/tickers_stocks.txt")
                .iter()
                .map(|x| Security::Stock(x.to_string()))
                .collect::<Vec<Security>>(),
            &headers::YF_STOCKS,
        ),
        "commodities" => (
            roses::read_tickers("../ref_data/tickers_commodities.txt")
                .iter()
                .map(|x| Security::Commodity(x.to_string()))
                //utf8_percent_encode(x, NON_ALPHANUMERIC).to_string()))
                .collect::<Vec<Security>>(),
            &headers::YF_COMMODITIES,
        ),
        "currencies" => (
            // prob broken, need to interlace the symbols
            roses::read_tickers("../ref_data/tickers_currencies.txt")
                .iter()
                .map(|x| Security::Currency(x.to_string()))
                .collect::<Vec<Security>>(),
            &headers::YF_CURRENCIES,
        ),
        "etf" => (
            roses::read_tickers("../ref_data/tickers_stocks.txt")
                .iter()
                .map(|x| Security::Etf(x.to_string()))
                .collect::<Vec<Security>>(),
            &headers::YF_STOCKS,
        ),

        _ => panic!("invalid asset class provided"),
    }
}

pub fn nls_to_dt(s: &str) -> Result<DateTime<FixedOffset>, chrono::ParseError> {
    let t = format!("{} {} +05:00", Utc::now().format("%Y-%m-%d"), s);
    return DateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S %z");
}

pub fn ndaq_url_to_ticker(url: String) -> String {
    let v: Vec<&str> = url.split("/").collect(); // divs
    return format!("{}", v[5]);
}
