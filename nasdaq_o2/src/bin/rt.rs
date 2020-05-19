extern crate chrono;
extern crate csv;
extern crate dotenv;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use nasdaq_o2;
use nasdaq_o2::nasdaq::realtime::NDAQ_REALTIME_HEADER;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
//use chrono::{DateTime, Duration, Utc};
use std::{
    //env,
    path::Path,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, serde::Deserialize)]
struct Record {
    symbol: String,
    weight: f64,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let now = Instant::now();
    //let dt = Utc::now();
    //let args: Vec<String> = env::args().collect();
    //assert_eq!(
    //    3,
    //    args.len(),
    //    "CLI:   [1]: asset class [`stocks`, `commodities`, `currencies`] \\
    //    [2]: sfx [TODO] "
    //);
    let filepath = "../ref_data/weighted_tickers.csv";
    let mut rdr = csv::Reader::from_path(filepath).expect("tf csv");
    let mut rng = thread_rng();
    let mut recs: Vec<Record> = vec![];
    for res in rdr.deserialize() {
        let rec: Record = res.expect("dist weights are bad");
        //println!("rec {:?}", rec);
        recs.push(rec);
    }
    //let len = recs.len();
    let dist = WeightedIndex::new(recs.iter().map(|x| x.weight)).unwrap();
    //println!("dist {:?}", dist);
    let mut urls = vec![];
    for _ in 0..2500 {
        urls.push(
            nasdaq_o2::Security::Stock(recs[dist.sample(&mut rng)].symbol.clone())
                .to_nasdaq_rt_url()
                .unwrap(),
        );
        //urls.push(nasdaq_o2::Security::Stock(recs[dist.sample(&mut rng)].symbol.clone()));
    }
    //println!("sampled{:?}", urls);

    //let securities = nasdaq_o2::gen_secs(&args);
    //let urls: Vec<String> = match args[2].as_str() {
    //    "realtime-trades" => securities
    //        .iter()
    //        .map(|x| x.to_nasdaq_rt_url().unwrap())
    //        .collect(),
    //    s => securities.iter().map(|x| x.to_nasdaq_url(s)).collect(),
    //};
    //println!("{:#?}, {:#?}", securities, urls);

    // commodities chart requires fromdate and todate params as y-m-d
    //let header: Vec<String> = match args[2].as_str() {
    //    "realtime-trades" => NDAQ_REALTIME_HEADER.iter().map(|x| x.to_string()).collect(),
    //    "chart" => NDAQ_CHART_HEADER.iter().map(|x| x.to_string()).collect(),
    //    "option-chain" => NDAQ_OPTION_HEADER.iter().map(|x| x.to_string()).collect(),
    //    "info" => NDAQ_QUOTE_HEADER.iter().map(|x| x.to_string()).collect(),
    //    _ => panic!("fix this garbo"),
    //};
    let header = NDAQ_REALTIME_HEADER.iter().map(|x| x.to_string()).collect();
    let recs: Vec<Vec<String>> = nasdaq_o2::lil_fetchvv_rt(urls).await;
    //match args[2].as_str() {
    //        "realtime-trades" => nasdaq_o2::lil_fetchvv_rt(urls).await,
    //        "chart" => nasdaq_o2::lil_fetchvv_chart(urls).await,
    //        "option-chain" => nasdaq_o2::lil_fetchvv_oc(urls).await,
    //        "info" => nasdaq_o2::lil_fetchv(urls).await,
    //        _ => panic!("todo, make fetch generic over <T>"),
    //    };
    let len: usize = recs.len();
    //let filename = format!("./data/{}/{}_{}.csv", args[2], args[1], now);
    let unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros()
        .to_string();

    let elapsed = now.elapsed().as_secs().to_string();
    let filename = format!("./data/realtime-trades/test_{}.csv", unixtime);
    let fp = Path::new(&filename);
    println!(
        "{:?} exists: {:?} and is_absolute: {:?}",
        fp,
        fp.exists(),
        fp.is_absolute()
    );
    nasdaq_o2::write_csv(&fp, recs, header).expect("csv error");
    println!(
        "{}: {} seconds: {} records",
        filename,
        elapsed,
        len.to_string()
    );

    Ok(())
}
//
//pub fn caller() {
//    let (open, close) = epoch_bounds();
//        if now < open {
//            time::sleep(1);
//        } else if open < now && now < close {
//            tick();
//        } else {
//            panic!("mkt is closed");
//        };
//    }
//}
