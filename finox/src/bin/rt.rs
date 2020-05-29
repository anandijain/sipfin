extern crate chrono;
extern crate csv;
extern crate reqwest;
extern crate roses;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use chrono::{prelude::*, DateTime, Timelike, Utc};

use std::{
    // TODO
    collections::HashMap,
    env,
    path::Path,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

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
        let now = Instant::now();
        let dt = Utc::now();
        i = i + 1;
        //let unixtime = SystemTime::now()
        //    .duration_since(UNIX_EPOCH)
        //    .expect("Time went backwards")
        //    .as_micros()
        //    .to_string();

        //let filename = format!("./data/realtime-trades/{}.csv", unixtime);
        //let fp = Path::new(&filename);
        //println!(
        //    "{:?} exists: {:?} and is_absolute: {:?}",
        //    fp,
        //    fp.exists(),
        //    fp.is_absolute()
        //);

        let s = dt.num_seconds_from_midnight();
        // fix this spagetti
        if !debug && s < 13 * 3060 + 30 * 60 {
            println!("premarket {:?}\n", dt.timestamp());
            thread::sleep(Duration::from_secs(10));
        } else if !debug && s > 20 * 3600 {
            println!("market is closed{:?}\n", dt.timestamp());
            thread::sleep(Duration::from_secs(100));
        } else {
            println!("market is open{:?}", dt.timestamp());

            //println!("pairs{:?}", pairs);

            //let recs: Vec<Vec<String>> =

            //let fetches = nasdaq_o2::fetch_rt(hm.clone()).await;
            hm = finox::fetch_rt(hm).await;
            //for (k, v) in &fetches {
            //    hm[k] = v.to_owned();
            //}

            println!("hashmap {:#?}", hm);
            
            //let mut nnew = 0;


            
            //for f in fetches.iter() {

            //    let recs = f
            //        .to_owned()
            //        .0
            //        .into_iter()
            //        .flatten()
            //        .collect::<Vec<Vec<String>>>();

            //    let len = recs.len();

            //    if len > 0 {
            //        nnew += len;
            //        let symb = recs[0][0].clone();
            //        pairs.push((
            //            nasdaq_o2::Security::Stock(symb.to_string())
            //                .to_nasdaq_rt_url()
            //                .unwrap(),
            //            f.1,
            //        ));

            //        //println!("{:#?}: {} {:#?}", yo[0][0], yo.len(), f.1);
            //    }
            //}
            //let len: usize = fetches.len();

            //let elapsed = now.elapsed().as_secs().to_string();

            //println!(
            //    "{}: {} {} seconds: {} records from {} endpoints",
            //    i,
            //    filename,
            //    elapsed,
            //    nnew.to_string(),
            //    len.to_string(),
            //);
        }
    }
    //Ok(())
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
