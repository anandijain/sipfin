extern crate chrono;
extern crate csv;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate roses;

use finox;
use futures::stream::StreamExt;
use headers::*;
use std;
use std::time::{SystemTime, UNIX_EPOCH};

use finox::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tickers = roses::read_tickers("../ref_data/tickers.txt");
    let symbs: Vec<&str> = CURRENCY_SYMBOLS_YF.to_vec(); //.into_iter().cloned().collect();
    let len = symbs.len();
    let mut urls: Vec<String> = vec![];
    for (i, x) in symbs.iter().enumerate() {
        for y in symbs[i..len].iter() {
            if x == y {
                continue;
            };
            urls.push(
            format!(
                //"https://query2.finance.yahoo.com/v8/finance/chart/{}=F?interval=1d&period1=0&period2=1589932800",
                "https://query1.finance.yahoo.com/v8/finance/chart/{}{}=X?interval=1d&period1=0&period2=1589932800",
                x.to_string(),
                y.to_string()
            )
        );
        }
    }
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.json::<yf::Root>().await {
                let recs: Vec<Vec<String>> = yf::Root::to_records(&root);
                println!("{}: {}", url, recs.len());
                return Some(recs);
            }
            println!("no json {}", url);
            return None;
        }
        println!("no2");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<Vec<String>>>>>();
    let vecs = fetches.await;
    let recs = vecs
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<Vec<String>>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<String>>>();
    //println!("{:#?}", recs);
    let cur_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let file_name = format!("../data/yf_currencies_{}.csv", cur_time.to_string());
    let mut wtr = csv::Writer::from_path(file_name)?;
    wtr.write_record(vec!["symbol", "t", "o", "h", "l", "c", "v"])?;

    for rec in recs.iter() {
        wtr.write_record(rec)?;
    }
    wtr.flush()?;

    Ok(())
}
