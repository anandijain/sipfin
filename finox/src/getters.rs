extern crate csv;
extern crate serde;

use std::error::Error;
use std::fs::OpenOptions;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::{thread, time};

use crate::types;

#[tokio::main]
pub async fn get_company(t: String) -> Result<Vec<types::Root>, reqwest::Error> {
    let url = [
        "https://www.bloomberg.com/markets2/api/datastrip/",
        &t,
        "%3AUS",
    ]
    .concat();
    println!("{}", url);
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let company: Vec<types::Root> = serde_json::from_str(&body.to_string()).unwrap();
    Ok(company)
}

pub fn company_to_rec(t: String) -> Result<Vec<csv::StringRecord>, reqwest::Error> {
    let mut ret: Vec<csv::StringRecord> = Vec::new();
    if let Ok(res) = get_company(t.to_string()) {
        for c in res.iter() {
            let rec = types::Root::to_record(c);
            ret.push(rec);
        }
    } else {
        println!("oh fuck");
    }
    Ok(ret)
}

pub fn headlines_to_rec(t: String) -> Result<Vec<csv::StringRecord>, reqwest::Error> {
    let mut ret: Vec<csv::StringRecord> = Vec::new();
    if let Ok(res) = get_company(t.to_string()) {
        for c in res.iter() {
            if let Ok(recs) = types::PressRelease::to_headlines(pr) {
                ret.concat(recs)
            }
        }
    } else {
        println!("oh fuck");
    }
    Ok(ret)
}

#[tokio::main]
pub async fn get_currency(t: String) -> Result<Vec<types::Intraday>, reqwest::Error> {
    let url = [
        "https://www.bloomberg.com/markets2/api/intraday/USD",
        &t,
        "%3ACUR?days=10&interval=0&volumeInterval=0",
    ]
    .concat();
    println!("{}", url);
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
    Ok(cur)
}

pub fn cur_to_rec(t: String) -> Result<Vec<csv::StringRecord>, reqwest::Error> {
    let mut ret: Vec<csv::StringRecord> = Vec::new();
    if let Ok(res) = get_currency(t.to_string()) {
        for c in res.iter() {
            for p in c.price.iter() {
                let rec = csv::StringRecord::from(types::Price::to_record(p));
                ret.push(rec);
            }
        }
    } else {
        println!("oh fuck");
    }
    Ok(ret)
}

// prob need to depr
pub fn get_csv(
    get_fn: fn(String) -> Result<Vec<csv::StringRecord>, reqwest::Error>,
    header: Vec<String>,
    symbols: Vec<String>,
    write_fn: String,
    ms_delay: u64,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_fn)?;
    wtr.write_record(&header);

    let delay = time::Duration::from_millis(ms_delay);

    for s in symbols.iter() {
        println!("{}", s.to_string());
        if let Ok(recs) = get_fn(s.to_string()) {
            for r in recs.iter() {
                wtr.write_record(r)?;
            }
        }
        thread::sleep(delay);
    }
    wtr.flush();
    Ok(())
}
