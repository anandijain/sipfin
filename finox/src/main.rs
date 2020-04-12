extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::error::Error;
use std::fs::OpenOptions;
use std::{thread, time};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod types;

fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[tokio::main]
async fn get_company(t: String) -> Result<Vec<types::Root>, reqwest::Error> {
    let url = [
        "https://www.bloomberg.com/markets2/api/datastrip/",
        &t,
        "%3AUS",
    ]
    .concat();
    println!("{}", url);
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let company: Vec<types::Root> = serde_json::from_str(&body.to_string()).unwrap();
    Ok(company)
}

fn company_to_rec(t: String) -> Result<Vec<csv::StringRecord>, reqwest::Error> {
    let mut ret: Vec<csv::StringRecord> = Vec::new();
    if let Ok(res) = get_company(t.to_string()) {
        for c in res.iter() {
            let rec = csv::StringRecord::from(types::Root::to_record(c));
            ret.push(rec);
        }
    } else {
        println!("oh fuck");
    }
    Ok(ret)
}

fn headlines_to_rec(t: String) -> Result<Vec<csv::StringRecord>, reqwest::Error> {
    let mut ret: Vec<csv::StringRecord> = Vec::new();
    if let Ok(res) = get_company(t.to_string()) {
        for c in res.iter() {
            if let Some(prs) = &c.press_releases {
                for pr in prs.iter() {
                    let rec = csv::StringRecord::from(types::PressRelease::to_record(pr));
                    ret.push(rec);
                }
            }
        }
    } else {
        println!("oh fuck");
    }
    Ok(ret)
}

#[tokio::main]
async fn get_currency(t: String) -> Result<Vec<types::Intraday>, reqwest::Error> {
    let url = [
        "https://www.bloomberg.com/markets2/api/intraday/USD",
        &t,
        "%3ACUR?days=10&interval=1&volumeInterval=1",
    ]
    .concat();
    println!("{}", url);
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string();
    let client = reqwest::Client::builder().user_agent(ua).build()?;
    let res = client.get(&url).send().await?;
    let body = res.text().await?;
    let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
    Ok(cur)
}

fn cur_to_rec(t: String) -> Result<Vec<csv::StringRecord>, reqwest::Error> {
    let mut ret: Vec<csv::StringRecord> = Vec::new();
    if let Ok(res) = get_currency(t.to_string()) {
        for c in res.iter() {
            for p in c.price.iter() {
                let rec = csv::StringRecord::from(types::Price::to_record(p));
                ret.push(rec);
            }
        }
    } else {
        println!("oh fuck");
    }
    Ok(ret)
}

fn get_csv(
    get_fn: fn(String) -> Result<Vec<csv::StringRecord>, reqwest::Error>,
    header: Vec<String>,
    symbols: Vec<String>,
    write_fn: String,
    ms_delay: u64,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_fn)?;
    wtr.write_record(&header);

    let delay = time::Duration::from_millis(ms_delay);

    for s in symbols.iter() {
        println!("{}", s.to_string());
        if let Ok(recs) = get_fn(s.to_string()) {
            for r in recs.iter() {
                wtr.write_record(r)?;
            }
        }
        thread::sleep(delay);
    }
    wtr.flush();
    Ok(())
}

fn main() -> Result<(), reqwest::Error> {
    //     let file_to_append = match OpenOptions::new().append(true).open(write_fn){
    //     Ok(mut f) => f,
    //     Err(e) => panic!("Could not open the file for appendin: {:?}", e)
    // };

    // let index = symbs.iter().position(|r| r.to_string() == "EVRG").unwrap();
    // let todo_symbs = &symbs[0..index];
    // println!("{:#?}", todo_symbs);
    // let dashboard_root = "https://www.bloomberg.com/markets/api/data-dashboard/tileset/";
    // let collection_tags = vec![
    //     "commodities",
    //     "futures",
    //     "asia",
    //     "americas",
    //     "europe",

    // ];

    // let stock_header: Vec<String> = vec![
    //     "id".to_string(),
    //     "short_name".to_string(),
    //     "market_cap".to_string(),
    //     "co_phone".to_string(),
    //     "last_update".to_string(),
    //     "average_volume30_day".to_string(),
    //     "price".to_string(),
    //     "open_price".to_string(),
    //     "high_price".to_string(),
    //     "low_price".to_string(),
    //     "low_price52_week".to_string(),
    //     "high_price52_week".to_string(),
    //     "number_of_employees".to_string(),
    //     "price_earnings_ratio".to_string(),
    //     "shares_outstanding".to_string(),
    // ];
    let headlines_header: Vec<String> = vec![
        "id".to_string(),
        "url".to_string(),
        "headline".to_string(),
        "lastmod".to_string(),
    ];

    let symbs_cur = vec![
        "EUR", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN", "BRL", "CLP", "PEN", "VEF", "CRC",
        "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF", "RUB", "TRY", "ILS", "KES", "ZAR",
        "MAD",
    ];
    let write_fn = "./data/sp500_headlines.csv".to_string();
    let symbs = read_tickers("./data/sp500tickers.txt");
    // for s in symbs_cur.iter(){
    //     let write_fn = format!("./data/USD{}.csv", s.to_string());
    //     get_csv(cur_to_rec, vec!["date_time".to_string(), format!("USD{}", s.to_string())], vec![s.to_string()], write_fn, 750);
    // }
    get_csv(headlines_to_rec, headlines_header, symbs, write_fn, 1000);
    Ok(())
}
