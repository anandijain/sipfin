extern crate chrono;
extern crate csv;
extern crate dotenv;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

//use rand::prelude::*;
//use rand::distributions::WeightedIndex;
use nasdaq_o2;
use nasdaq_o2::nasdaq::{
    chart::NDAQ_CHART_HEADER, info::NDAQ_QUOTE_HEADER, realtime::NDAQ_REALTIME_HEADER, option_chain::NDAQ_OPTION_HEADER,
};
use std::{env, path::Path, time::Instant};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        3,
        args.len(),
        "CLI:   [1]: asset class [`stocks`, `commodities`, `currencies`] \\
        [2]: sfx [TODO] "
    );

    let securities = nasdaq_o2::gen_secs(&args);
    let urls: Vec<String> = match args[2].as_str() {
        "rt" => securities
            .iter()
            .map(|x| x.to_nasdaq_rt_url().unwrap())
            .collect(),
        s => securities.iter().map(|x| x.to_nasdaq_url(s)).collect(),
    };
    println!("{:#?}, {:#?}", securities, urls);
    // only send choices to fetcher
    // let freqs = nasdaq_o2::read_tickers()
    // assert_eq!(urls.len(), freqs.len())

    // TODO: read csv instead of txt, and unwrap to tuples
    // commodities chart requires fromdate and todate params as y-m-d
    let now = Instant::now();
    let recs: Vec<Vec<String>> = match args[2].as_str() {
        "rt" => nasdaq_o2::lil_fetchvv_rt(urls).await,
        "chart" => nasdaq_o2::lil_fetchvv_chart(urls).await,
        "option-chain" => nasdaq_o2::lil_fetchvv_oc(urls).await,
        "info" => nasdaq_o2::lil_fetchv(urls).await,
        _ => panic!("todo, make fetch generic over <T>"),
    };

    let header: Vec<String> = match args[2].as_str() {
        "rt" => NDAQ_REALTIME_HEADER.iter().map(|x| x.to_string()).collect(),
        "chart" => NDAQ_CHART_HEADER.iter().map(|x| x.to_string()).collect(),
        "option-chain" => NDAQ_OPTION_HEADER.iter().map(|x| x.to_string()).collect(),
        "info" => NDAQ_QUOTE_HEADER.iter().map(|x| x.to_string()).collect(),
        _ => panic!("fix this garbo"),
    };
    let len: usize = recs.len();
    let t: String = nasdaq_o2::epoch_str();
    let filename = format!(
        "//home/sippycups/D/nasdaq_o2/{}/{}_{}.csv",
        args[2], args[1], t
    );
    let fp = Path::new(&filename);
    println!("{:?}{:?}", fp.exists(), fp.is_absolute());
    // fix so that all this is auto configd by cli args
    nasdaq_o2::write_csv(&fp, recs, header).expect("csv error");

    println!(
        "{}: {} seconds: {} records",
        filename,
        now.elapsed().as_secs(),
        len.to_string()
    );

    Ok(())
}
