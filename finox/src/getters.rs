extern crate csv;
extern crate serde;

use crate::news;
use crate::types;
use crate::yf;
use std::{thread, time};

pub const DELAY: std::time::Duration = time::Duration::from_millis(10);

#[tokio::main]
pub async fn simple_get(url: String) -> Result<String, reqwest::Error> {
    println!("{}", url);
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    thread::sleep(DELAY);
    let body = res.text().await?;
    Ok(body)
}

pub enum Security {
    F(String),
    X(String),
    US(String),
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

pub fn yf_US(t: String) -> Option<Vec<Vec<String>>> {
    let url = yf_url(Security::US(t));
    if let Ok(body) = simple_get(url) {
        let ohlcv: yf::Root = serde_json::from_str(&body.to_string()).unwrap();
        return Some(yf::Root::to_records(&ohlcv));
    }
    // let yfroot: yf::Root = reqwest::Client::new().user_agent(ua).get(url.to_string()).json(&yfroot).await?.json().await?;
    return None;
}

pub fn yf_X(t: String) -> Option<Vec<Vec<String>>> {
    let url = yf_url(Security::X(t));
    if let Ok(body) = simple_get(url) {
        let ohlcv: yf::Root = serde_json::from_str(&body.to_string()).unwrap();
        return Some(yf::Root::to_records(&ohlcv));
    }
    return None;
}

pub fn yf_F(t: String) -> Option<Vec<Vec<String>>> {
    let url = yf_url(Security::F(t));
    if let Ok(body) = simple_get(url) {
        let ohlcv: yf::Root = serde_json::from_str(&body.to_string()).unwrap();
        return Some(yf::Root::to_records(&ohlcv));
    }
    return None;
}
