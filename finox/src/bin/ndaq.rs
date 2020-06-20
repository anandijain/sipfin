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
            "d" | "dividends" => {
                let mut hm = HashMap::new();
                for symb in tickers.iter() {
                    hm.insert(
                        symb.to_string(),
                        bad_fmt2(symb, "dividends?assetclass=stocks&"),
                    );
                }

                finox::fetch_write::<dividends::DividendsRoot>(
                    hm,
                    "../data/nasdaq/dividends/",
                    &dividends::NDAQ_DIVIDEND_HEADER,
                )
                .await
            }
            "h" | "institutional-holdings" => {
                let mut hm = HashMap::new();
                for symb in tickers.iter() {
                    hm.insert(symb.to_string(), bad_fmt(symb, "institutional-holdings?"));
                }

                finox::fetch_write::<institutional::HolderRoot>(
                    hm,
                    "../data/nasdaq/institutional-holdings/",
                    &institutional::NDAQ_HOLDER_HEADER,
                )
                .await
            }
            "i" | "insider-trades" => {
                let mut hm = HashMap::new();
                for symb in tickers.iter() {
                    hm.insert(symb.to_string(), bad_fmt(symb, "insider-trades?"));
                }

                finox::fetch_write::<insiders::InsidersRoot>(
                    hm,
                    "../data/nasdaq/insider-trades/",
                    &insiders::NDAQ_INSIDER_HEADER,
                )
                .await
            }
            "oc" | "option-chain" => {
                let mut hm = HashMap::new();
                for symb in tickers.iter() {
                    hm.insert(symb.to_string(), bad_fmt(symb, "option-chain?"));
                }

                finox::fetch_write::<option_chain::OptionChainRoot>(
                    hm,
                    "../data/nasdaq/option-chain/",
                    &option_chain::NDAQ_OPTION_HEADER,
                )
                .await
            }
            "e" | "eps" => {
                let mut hm = HashMap::new();
                for symb in tickers.iter() {
                    hm.insert(symb.to_string(), bad_fmt2(symb, "eps?"));
                }

                finox::fetch_write::<earnings::EarningsRoot>(
                    hm,
                    "../data/nasdaq/earnings/",
                    &earnings::NDAQ_EARNINGS_HEADER,
                )
                .await
            }

            _ => panic!("nah2"),
        };
        println!("{:#?}", recs);
    }
    Ok(())
}

fn bad_fmt(t: &str, q: &str) -> String {
    format!("https://api.nasdaq.com/api/company/{}/{}limit=99999", t, q)
}

//LMAO
fn bad_fmt2(t: &str, q: &str) -> String {
    format!("https://api.nasdaq.com/api/quote/{}/{}limit=99999", t, q)
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
