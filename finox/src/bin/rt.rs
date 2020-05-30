extern crate chrono;
extern crate reqwest;
extern crate roses;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use chrono::{prelude::*, Timelike, Utc};

use std::{collections::HashMap, env, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();
    let debug = if args.len() > 1 { true } else { false };
    let filepath = "../ref_data/tickers_stocks.txt";
    let tickers = roses::read_tickers(filepath);
    // <(String, DateTime<FixedOffset>)> = vec![];
    let mut hm = HashMap::new();
    for symb in tickers.iter() {
        hm.insert(
            finox::Security::Stock(symb.to_string()),
            FixedOffset::east(5 * 3600).ymd(1970, 1, 1).and_hms(0, 1, 1),
        );
    }

    let mut i: usize = 0;
    loop {
        let dt = Utc::now();
        i = i + 1;

        let s = dt.num_seconds_from_midnight();
        // fix this spagetti
        if !debug && s < 13 * 3060 + 30 * 60 {
            println!("premarket {:?}\n", dt.timestamp());
            thread::sleep(Duration::from_secs(10));
        } else if !debug && s > 20 * 3600 {
            println!("market is closed{:?}\n", dt.timestamp());
            thread::sleep(Duration::from_secs(100));
        } else {
            println!("{}: market is open{:?}", i, dt.to_rfc3339());

            hm = finox::fetch_rt(hm).await;

            println!("hashmap {:#?}", hm);
        }
    }
    //Ok(())
}

//match args[2].as_str() {
//        "realtime-trades" => nasdaq_o2::lil_fetchvv_rt(urls).await,
//        "chart" => nasdaq_o2::lil_fetchvv_chart(urls).await,
//        "option-chain" => nasdaq_o2::lil_fetchvv_oc(urls).await,
//        "info" => nasdaq_o2::lil_fetchv(urls).await,
//        _ => panic!("todo, make fetch generic over <T>"),
//    };
