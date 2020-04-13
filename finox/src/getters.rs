extern crate csv;
extern crate serde;

use std::error::Error;

use crate::types;
use crate::news;
use std::{thread, time};

pub const DELAY: std::time::Duration = time::Duration::from_millis(1000);

#[tokio::main]
pub async fn get_datastrip(t: String) -> Result<Vec<types::Root>, reqwest::Error> {
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
    thread::sleep(DELAY);
    Ok(company)
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
    thread::sleep(DELAY);
    Ok(cur)
}

#[tokio::main]
pub async fn get_history(t: String) -> Result<Vec<types::Intraday>, reqwest::Error> {
    let url = [
        "https://www.bloomberg.com/markets2/api/history/",
        &t,
        "/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily",
    ]
    .concat();
    println!("{}", url);
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
    thread::sleep(DELAY);
    Ok(cur)
}

#[tokio::main]
pub async fn get_news(t: String) -> Result<news::NewsVec, reqwest::Error> {
    let url = [
        "https://www.bloomberg.com/markets/api/comparison/news?securityType=",
        &t,
        "&limit=1000",
    ]
    .concat();
    println!("{}", url);
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let cur: news::NewsVec = serde_json::from_str(&body.to_string()).unwrap();
    thread::sleep(DELAY);
    Ok(cur)
}

// IND COM US GOV  