extern crate chrono;
extern crate csv;
use chrono::{Datelike, Utc};

use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::{Instant, Duration},
    thread,
};

use crate::getters;
use crate::keys;
use crate::news;
use crate::sa;
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

pub fn write_yf(url: String, s: String) -> Result<(), csv::Error> {
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
        write_yf(url.to_string(), s.to_string());
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
            write_yf(url.to_string(), symb.to_string());
        }
    }
    Ok(())
}

pub fn yf_F() -> Result<(), reqwest::Error> {
    for s in COMMODITIES_SYMBOLS_YF.iter() {
        let url = yf_url(Security::F(s.to_string()));
        write_yf(url.to_string(), s.to_string());
    }
    Ok(())
}

pub fn sa() -> Result<(), reqwest::Error> {
    let url = "https://seekingalpha.com/get_trending_articles";
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: sa::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = sa::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./sa.csv".to_string(), &SA_HEADER, recs);
    }
    Ok(())
}

pub fn nytfeed() -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.nytimes.com/svc/news/v3/content/all/all.json?api-key={}",
        keys::NYT_KEY.to_string()
    );
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: news::NYTFeed = serde_json::from_str(&body.to_string()).unwrap();
        let recs = news::NYTFeed::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./nytfeed.csv".to_string(), &NYT_FEED_HEADER, recs);
    }
    Ok(())
}

pub fn nytarchive() -> Result<(), csv::Error> {
    let filename = "./nyt_archive.csv".to_string();
    let mut wtr = csv::Writer::from_path(filename)?;
    let nyt_delay: std::time::Duration = Duration::from_millis(6000);

    wtr.write_record(&NYT_ARCHIVE_HEADER);
    for i in 1853..2019 {
        for j in 1..13 {
            let url = format!(
                "https://api.nytimes.com/svc/archive/v1/{}/{}.json?api-key={}",
                i,
                j,
                keys::NYT_KEY.to_string()
            );
            if let Ok(body) = getters::simple_get(url.to_string()) {
                let root: news::NYTArchive = serde_json::from_str(&body.to_string()).unwrap();
                let recs: Vec<csv::StringRecord> = news::NYTArchive::to_records(&root)
                    .into_iter()
                    .map(|x| csv::StringRecord::from(x))
                    .collect();
                for r in recs.iter(){
                    wtr.write_record(r);
                }
                thread::sleep(nyt_delay);
            }
        }
    }
    wtr.flush();
    Ok(())
}

pub const CURRENCY_SYMBOLS_YF: [&'static str; 6] = ["USD", "EUR", "JPY", "GBP", "AUD", "CAD"];

pub const COMMODITIES_SYMBOLS_YF: [&'static str; 23] = [
    "ES", "YM", "NQ", "RTY", "ZB", "ZN", "ZF", "ZT", "GC", "SI", "HG", "PA", "CL", "HO", "NG",
    "RB", "BZ", "C", "KW", "SM", "BO", "S", "CT",
];

pub const NEWS_HEADER: [&'static str; 3] = ["url", "headline", "date_time"];

pub const HEADLINES_HEADER: [&'static str; 4] = ["id", "url", "headline", "lastmod"];

pub const YF_HEADER: [&'static str; 6] = ["t", "o", "h", "l", "c", "v"];

pub const SA_HEADER: [&'static str; 8] = [
    "id",
    "author_id",
    "publish_on",
    "title",
    "slug",
    "ncomments",
    "author_name",
    "path",
];

pub const NYT_FEED_HEADER: [&'static str; 14] = [
    "slug",
    "section",
    "subsec",
    "title",
    "abs",
    "by",
    "item_type",
    "source",
    "first_pub",
    "created",
    "pub",
    "updated",
    "mat_type",
    "url",
];

pub const NYT_ARCHIVE_HEADER: [&'static str; 12] = [
    "id", "wc", "by", "pub", "doctype", "page", "headline", "kicker", "snippet", "abstract", "url", "source",
];
