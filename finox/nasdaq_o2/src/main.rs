extern crate chrono;
extern crate csv;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate rand;

use rand::prelude::*;
use rand::distributions::WeightedIndex;

mod lib;
use std::{
    env,
    time::Instant,
};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        3,
        args.len(),
        "CLI:   [1]: asset class [`stocks`, `commodities`, `currencies`] \\
        [2]: sfx [TODO] "
    );
    
    let securities = lib::gen_secs(args);
    let urls: Vec<String> = match args[2].as_str() {
        "rt" => securities
        .iter()
        .map(|x| x.to_nasdaq_rt_url().unwrap())
        .collect(),
        s => securities.iter().map(|x| x.to_nasdaq_url(s)).collect(),
    };
    println!("{:#?}, {:#?}", securities, urls);
    // only send choices to fetcher
    // let freqs = lib::read_tickers()
    // assert_eq!(urls.len(), freqs.len())
    
    // TODO: read csv instead of txt, and unwrap to tuples
    
    
    let now = Instant::now();
    let recs = lib::lil_fetchvv(urls).await;
    let len: usize = recs.len();
    let t: String = lib::epoch_str();
    let filename: String = format!("/home/sippycups/D/nasdaq_o2/rt/{}.csv", t);
    lib::write_csv(
        &filename,
        recs,
        lib::nasdaq::NDAQ_REALTIME_HEADER
            .iter()
            .map(|x| x.clone().to_string())
            .collect(),
    )
    .expect("csv error");

    println!(
        "{}: {} seconds: {} records",
        filename,
        now.elapsed().as_secs(),
        len.to_string()
    );

    Ok(())
}
