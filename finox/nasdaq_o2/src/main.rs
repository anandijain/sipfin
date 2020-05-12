extern crate chrono;
extern crate csv;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use futures::stream::StreamExt;

extern crate types;

// pub mod types;
fn foo() {

    types::
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let all_urls = gen_urls();
    println!("Hello, {:#?}!", all_urls);
    
    let fetches = futures::stream::iter(all_urls[1].into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            if let Ok(root) = res.json::<types::nasdaq::chart::ChartRoot>().await {
                
                let symb = root.data.symbol.clone();
                // println!("{}", symb);

                let chart: Vec<Vec<String>> = root
                    .data
                    .chart
                    .iter()
                    .map(|c| vec![c.x.to_string(), c.y.to_string()])
                    .collect();

                let t = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs()
                    .to_string();

                match write_csv(
                    format!("/home/sippycups/nasdaq_o2/data/{}_{}.csv", symb, t),
                    chart,
                    vec!["t".to_string(), symb.clone()]
                ) {
                    Ok(_) => println!("allgood"),
                    _ => println!("no good"),
                }
                return Some(());
            }
            println!("no good");
            return None;
        }
        println!("no good1");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<()>>>();
    let _vecs = fetches.await;

    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn write_csv(filename: String, data: Vec<Vec<String>>, header: Vec<String>) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path(&filename)?;
    wtr.write_record(header);
    // assert_eq!(header.len(), data[0].len() )
    for row in data.iter() {
        wtr.write_record(row);
    }
    wtr.flush();
    Ok(())
}


pub fn gen_urls() -> Vec<Vec<String>> {
    let tick_sfxs = vec!["option-chain", "chart", "info"];
    let tickers: Vec<String> = read_tickers("/home/sippycups/sipfin/finox/ref_data/tickers.txt"); // TODO: get from sql table
    let mut urls: Vec<Vec<String>> = vec![];
    for sfx in tick_sfxs.iter() {
        let sfx_urls: Vec<String> = tickers
        .iter()
        .map(|x| {
            format!(
                "https://api.nasdaq.com/api/quote/{}/{}?assetclass=stocks",
                x.to_string(),
                sfx.to_string()
            )
        })
        .collect();
        urls.push(sfx_urls);
    }
    return urls;
}