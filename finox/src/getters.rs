extern crate csv;
extern crate serde;

use crate::news;
use crate::yf;
use std::collections::HashMap;

use std::{thread, time};

pub const DELAY: std::time::Duration = time::Duration::from_millis(10);

pub const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36";

#[tokio::main]
pub async fn simple_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT.to_string()).build()?;
    let res = client.get(&url).send().await?;
    thread::sleep(DELAY);
    let body = res.text().await?;
    println!("{}: {:#?}", url, body);
    Ok(body)
}


#[tokio::main]
pub async fn simple_json(url: String) -> Result<(yf::YFinList), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT.to_string()).build()?;
    let resp = client.get(&url).send()
        .await?
        .json::<yf::YFinList>() // CHANGE TYPE
        .await?;
    println!("yo{}: {:#?}", &url.to_string(), yf::YFinList::to_records(&resp));
    println!("{}: {:#?}", &url.to_string(), yf::YFinList::to_records(&resp));
    Ok(resp)
}


pub fn yf_from_url(url: String) -> Option<Vec<Vec<String>>>{
    if let Ok(body) = simple_get(url) {
        let ohlcv: yf::Root = serde_json::from_str(&body.to_string()).unwrap();
        return Some(yf::Root::to_records(&ohlcv));
    }
    return None;
}
