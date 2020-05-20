extern crate chrono;
extern crate csv;
extern crate dotenv;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use chrono::{Utc, Timelike};
use nasdaq_o2;
use nasdaq_o2::nasdaq::realtime::NDAQ_REALTIME_HEADER;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::{
    env,
    path::Path,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, serde::Deserialize)]
struct Record {
    symbol: String,
    weight: f64,
}
//let args: Vec<String> = env::args().collect();
//assert_eq!(
//    3,
//    args.len(),
//    "CLI:   [1]: asset class [`stocks`, `commodities`, `currencies`] \\
//    [2]: sfx [TODO] "
//);
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
//match args[2].as_str() {
//        "realtime-trades" => nasdaq_o2::lil_fetchvv_rt(urls).await,
//        "chart" => nasdaq_o2::lil_fetchvv_chart(urls).await,
//        "option-chain" => nasdaq_o2::lil_fetchvv_oc(urls).await,
//        "info" => nasdaq_o2::lil_fetchv(urls).await,
//        _ => panic!("todo, make fetch generic over <T>"),
//    };

#[tokio::main]
async fn main() -> Result<(), String> {
    let debug = if env::args().collect::<Vec<String>>().len() > 1 {true} else {false};
    let filepath = "../ref_data/weighted_tickers.csv";
    let mut rdr = csv::Reader::from_path(filepath).expect("tf csv");
    let mut rng = thread_rng();
    let mut recs: Vec<Record> = vec![];
    for res in rdr.deserialize() {
        let rec: Record = res.expect("dist weights are bad");
        //println!("rec {:?}", rec);
        recs.push(rec);
    }
    let dist = WeightedIndex::new(recs.iter().map(|x| x.weight)).unwrap();

    let mut i = 0;

    loop {
        let now = Instant::now();
        let dt = Utc::now();
        i = i + 1;
        println!("{}: {:?}\n", i, dt);
        let s = dt.num_seconds_from_midnight();
        // fix this spagetti 
        if  !debug && s < 13 * 3060 + 30 * 60 { 
            println!("premarket {:?}\n", dt.timestamp());
            thread::sleep(Duration::from_secs(10));
        } else if !debug && s > 20 * 3600 {
            println!("market is closed{:?}\n", dt.timestamp());
            break;
        } else {
            println!("market is open{:?}\n", dt.timestamp());
            let mut urls = vec![];
            for _ in 0..2500 {
                urls.push(
                    nasdaq_o2::Security::Stock(recs[dist.sample(&mut rng)].symbol.clone())
                        .to_nasdaq_rt_url()
                        .unwrap(),
                );
            }
            let header = NDAQ_REALTIME_HEADER.iter().map(|x| x.to_string()).collect();
            let recs: Vec<Vec<String>> = nasdaq_o2::lil_fetchvv_rt(urls).await;
            let len: usize = recs.len();
            
            let unixtime = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_micros()
                .to_string();

            let elapsed = now.elapsed().as_secs().to_string();
            let filename = format!("./data/realtime-trades/rfc3339_test_{}.csv", unixtime);
            let fp = Path::new(&filename);
            println!(
                "{:?} exists: {:?} and is_absolute: {:?}",
                fp,
                fp.exists(),
                fp.is_absolute()
            );

            nasdaq_o2::write_csv(&fp, recs, header).expect("csv error");
            println!(
                "{}: {} {} seconds: {} records",
                i,
                filename,
                elapsed,
                len.to_string(),
            );
        }
    }
    Ok(())
}
