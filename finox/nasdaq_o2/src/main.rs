#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate csv;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use diesel::prelude::*;
use dotenv::dotenv;
use futures::stream::StreamExt;

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

mod nasdaq;
use nasdaq::{
    chart::ChartRoot, dividends::DividendsRoot, info::InfoRoot, info::NDAQ_QUOTE_HEADER,
    insiders::InsidersRoot, option_chain::OptionChainRoot, realtime::RealtimeRoot, realtime::NDAQ_REALTIME_HEADER,
};

mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tickers: Vec<String> = read_tickers("/home/sippycups/sipfin/finox/ref_data/tickers.txt"); // TODO: get from sql table
    let all_urls = gen_urls(tickers, vec!["realtime-trades".to_string()]);
    let urls: Vec<String> = all_urls[0].clone();
    let now = Instant::now();

    // make distinct if endpoint serves a vec<rec> or a rec

    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.json::<RealtimeRoot>().await {
                return Some(root.to_recs());
            }
            println!("serialized json wrong {}", url.clone());
            return None;
        }
        println!("no good1");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<Vec<String>>>>>()
    .await;
    // let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect();
    // let recs: Vec<Vec<Vec<String>>> = fetches.into_iter().flatten().collect();
    let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect::<Vec<Vec<Vec<String>>>>().into_iter().flatten().collect();
    // let recs: Vec<Vec<String>> = fetches.iter().flat_map(|x| x.into_iter().flatten().collect::<Vec<Vec<Vec<String>>>>()).collect();
    let len: usize = recs.len();

    let t: String = epoch_str();
    let filename: String = format!("./data/rt/{}.csv", t);
    write_csv(
        filename,
        recs,
        NDAQ_REALTIME_HEADER
            .iter()
            .map(|x| x.clone().to_string())
            .collect(),
    )?;

    println!(
        "{} seconds: {} records",
        now.elapsed().as_secs(),
        len.to_string()
    );

    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn write_csv(
    filename: String,
    data: Vec<Vec<String>>,
    header: Vec<String>,
) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path(filename.to_string())
        .expect(format!("whtf csv {}", filename).as_ref());
    wtr.write_record(header.clone())?;
    wtr.flush()?;
    for row in data.iter() {
        assert_eq!(header.len(), row.len()); // perf hit?
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn gen_urls(tickers: Vec<String>, sfxs: Vec<String>) -> Vec<Vec<String>> {
    // let tick_sfxs = vec!["insider-trades"];
    // "option-chain", "chart", "info", "dividends", realtime-trades
    let mut urls: Vec<Vec<String>> = vec![];
    for sfx in sfxs.iter() {
        let sfx_urls: Vec<String> = tickers
            .iter()
            .map(|x| {
                format!(
                    "https://api.nasdaq.com/api/quote/{}/{}?assetclass=stocks&limit=100",
                    // "https://api.nasdaq.com/api/company/{}/{}?limit=99999&type=ALL",
                    x.to_string(),
                    sfx.to_string()
                )
            })
            .collect();
        urls.push(sfx_urls);
    }
    return urls;
}

pub fn epoch_str() -> String {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string();
    return t;
}

pub fn ndaq_url_to_ticker(url: String) -> String {
    let v: Vec<&str> = url.split("/").collect(); // divs
    return format!("{}_insider", v[5]);
}
// pub fn lilfetcher(urls: Vec<String>, )

// pub fn establish_connection() -> PgConnection {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

// pub fn create_quote<'a>(conn: &diesel::pg::PgConnection, q: &'a models::NewQuote) -> models::Quote {
//     diesel::insert_into(schema::quotes::table)
//         .values(q)
//         .get_result(conn)
//         .expect("Error saving new post")
// }
