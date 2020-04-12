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

fn get_csv(
    get_fn: fn(String) -> Result<Vec<csv::StringRecord>, reqwest::Error>,
    header: Vec<String>,
    symbs: Vec<String>,
    write_fn: String,
    ms_delay: u64,
) -> Result<(), Box<dyn Error>> {

    let mut wtr = csv::Writer::from_path(write_fn)?;
    wtr.write_record(&header);

    let delay = time::Duration::from_millis(ms_delay);

    for s in symbs.iter() {
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


    let write_fn = "./data/companies_2.csv".to_string();
    let symbs = read_tickers("./data/sp500tickers.txt");
    let header = vec![
        "id".to_string(),
        "short_name".to_string(),
        "market_cap".to_string(),
        "last_update".to_string(),
        "average_volume30_day".to_string(),
        "price".to_string(),
        "open_price".to_string(),
        "high_price".to_string(),
        "low_price".to_string(),
        "low_price52_week".to_string(),
        "high_price52_week".to_string(),
        "number_of_employees".to_string(),
        "price_earnings_ratio".to_string(),
        "shares_outstanding".to_string(),
    ];
    get_csv(company_to_rec, header, symbs, write_fn, 500);

    Ok(())
}
