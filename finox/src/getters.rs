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

pub fn yf_from_url(url: String) -> Option<Vec<Vec<String>>>{
    if let Ok(body) = simple_get(url) {
        let ohlcv: yf::Root = serde_json::from_str(&body.to_string()).unwrap();
        return Some(yf::Root::to_records(&ohlcv));
    }
    return None;
}