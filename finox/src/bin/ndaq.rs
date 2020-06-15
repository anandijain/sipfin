extern crate tokio;
use finox::nasdaq::*;
use std::{collections::HashMap, env};

#[tokio::main]
pub async fn main() -> Result<(), String> {
    let endpoints = env::args().collect::<Vec<_>>();
    if endpoints.len() < 2 {
        panic!("nah");
    }
    let filepath = "../ref_data/tickers_stocks.txt";
    let tickers = finox::roses::read_tickers(filepath);

    // todo async
    for ep in endpoints[1..endpoints.len()].iter() {
        let recs = match ep.as_str() {
            "dividends" => {
                let mut hm = HashMap::new();
                let urls = tickers
                    .iter()
                    .map(|x| bad_fmt(x, "dividends"))
                    .collect::<Vec<_>>();
                for symb in tickers.iter() {
                    hm.insert(symb.to_string(), bad_fmt(symb, "dividends"));
                }

                finox::fetch_write::<dividends::DividendsRoot>(
                    hm,
                    "../data/nasdaq/dividends/",
                    &dividends::NDAQ_DIVIDEND_HEADER,
                )
                .await
            }
            "institutional-holdings" => {
                let mut hm = HashMap::new();
                let urls = tickers
                    .iter()
                    .map(|x| bad_fmt(x, "institutional-holdings"))
                    .collect::<Vec<_>>();
                for symb in tickers.iter() {
                    hm.insert(symb.to_string(), bad_fmt(symb, "institutional-holdings"));
                }

                finox::fetch_write::<institutional::HolderRoot>(
                    hm,
                    "../data/nasdaq/institutional-holdings/",
                    &institutional::NDAQ_HOLDER_HEADER,
                )
                .await
            }
            _ => panic!("nah2"),
            //"insider-trades"=> "",
            //""=> "",
        };
        println!("{:#?}", recs);
    }
    Ok(())
}

fn bad_fmt(t: &str, q: &str) -> String {
    format!("https://api.nasdaq.com/api/company/{}/{}?limit=99999", t, q)
}

//Dividend-History
//Historical-Quotes
//Historical-NOCP
//Financials
//Earnings
//P/E-&-PEG-Ratios
//Option-Chain
//Short-Interest
//Institutional-Holdings
//Insider-Activity"""
