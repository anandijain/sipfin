extern crate csv;
extern crate serde;

use crate::types;
use crate::news;
use std::{thread, time};

pub const DELAY: std::time::Duration = time::Duration::from_millis(1000);

#[tokio::main]
pub async fn simple_get(url: String) -> Result<String, reqwest::Error> {
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    thread::sleep(DELAY);
    let body = res.text().await?;
    Ok(body)
}


#[tokio::main]
pub async fn get_datastrip(t: String) -> Option<Vec<types::Root>> {
    let url = [
        "https://www.bloomberg.com/markets2/api/datastrip/",
        &t,
        "%3AUS",
    ]
    .concat();
    println!("{}", url);
    if let Ok(body) = simple_get(url) {
        let company: Vec<types::Root> = serde_json::from_str(&body.to_string()).unwrap();
        if company != vec![] {
            Some(company)
        }
    }
    None
}

pub fn get_intraday(t: String) -> Option<Vec<types::Intraday>> {
    let url = [
        "https://www.bloomberg.com/markets2/api/intraday/",
        &t,
        "?days=10&interval=0&volumeInterval=0",
    ]
    .concat();
    println!("{}", url);
    if let Ok(body) = simple_get(url) {
        let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
        if cur != vec![] {
            Some(cur)
        }
    }
    None
}

#[tokio::main]
pub async fn get_history(t: String) -> Option<Vec<types::Intraday>> {
    let url = [
        "https://www.bloomberg.com/markets2/api/history/",
        &t,
        "/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily",
    ]
    .concat();
    println!("{}", url);
    if let Ok(body) = simple_get(url) {
        let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
        if cur != vec![] {
            Some(cur)
        }
    }
    None
}

#[tokio::main]
pub async fn get_news(t: String) -> Option<news::NewsVec> {
    let url = [
        "https://www.bloomberg.com/markets/api/comparison/news?securityType=",
        &t,
        "&limit=1000",
    ]
    .concat();
    println!("{}", url);

    if let Ok(body) = simple_get(url) {
        let cur: news::NewsVec = serde_json::from_str(&body.to_string()).unwrap();
        if cur != vec![] {
            Some(cur)
        }
    }
    None
}

// IND COM CUR US GOV  