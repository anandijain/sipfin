extern crate chrono;
extern crate csv;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

// extern crate nasdaq;
mod nasdaq;
use nasdaq::{
    chart::ChartRoot,
    option_chain::OptionChainRoot,
};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use futures::stream::StreamExt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let all_urls = gen_urls();

    let option_urls: Vec<String> = all_urls[0].clone();
    let chart_urls: Vec<String> = all_urls[1].clone();
    let info_urls: Vec<String> = all_urls[2].clone();


    let fetches = futures::stream::iter(option_urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            if let Ok(root) = res.json::<OptionChainRoot>().await {
                println!("{:#?}", root.clone());
                
                // let symb = root.data.symbol.clone(); // used in chart
                let symb = root.get_ticker(); // used in op chart
                let chart: Vec<Vec<String>> = root.to_recs();
                println!("{:#?}", chart.clone());

                let t = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs()
                    .to_string();

                let filename: String = format!("./data/{}_{}.csv", symb, t);
                match write_csv(
                    filename,
                    chart,
                    vec!["t".to_string(), symb.clone()]
                ) {
                    Ok(_) => println!("allgood"),
                    _ => println!("CSV FUCKED good"),
                }
                return Some(());
            }
            println!("no good {}", url.clone());
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
    let mut wtr = csv::Writer::from_path(filename.to_string()).expect("whtf");
    wtr.write_record(header);
    wtr.flush();
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