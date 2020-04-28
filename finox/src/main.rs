extern crate chrono;
extern crate csv;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use futures::channel::mpsc;
use futures::future::join_all;
use futures::io::{AllowStdIo, AsyncWriteExt};
use futures::stream::StreamExt;
use futures::{executor::block_on, future::Future, stream::Stream};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{BufWriter, Cursor};
use std::time::Instant;
use tokio::task::JoinHandle;

mod bloomberg;
mod getters;
mod gs;
mod jpxnews;
mod keys;
mod news;
mod sa;
mod steam;
mod utils;
mod weather;
mod xueqiu;
mod yf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // utils::nytarchive();

    // utils::nytfeed();
    // utils::gsnews();
    // utils::jpxnews();
    // utils::reuters();
    // utils::wsj_videos();
    // utils::sa();
    // regexmain();
    let tickers = utils::read_tickers("./ref_data/hist_symbs.txt");
    let start_times = utils::read_tickers("./ref_data/first_trade_date.txt");
    assert!(tickers.len() == start_times.len());
    let mut urls: Vec<String> = Vec::new();

    for i in 0..tickers.len() {
        // urls.push(format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?lang=en-US&region=US&interval=1m&period1={}&period2=1587859200",tickers[i].to_string(), start_times[i].to_string()));
        urls.push(format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?lang=en-US&region=US&interval=1m&range=7d",tickers[i].to_string()));
    }
    
    println!("{:#?}", urls);

    // let index = urls
    //     .iter()
    //     .position(|r| {
    //         r.to_string()
    //             == "https://query1.finance.yahoo.com/v8/finance/chart/TOUR?region=US&range=1d"
    //                 .to_string()
    //     })
    //     .unwrap();

    // let todo_symbs = &urls[index..urls.len()];
    // sync_main(todo_symbs.to_vec());
    async_main(urls);
    Ok(())
}

// https://www.bloomberg.com/markets2/api/intraday/ZSL:US?days=1
fn sync_main(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // let xs: Vec<utils::Security> = utils::yf_x_urls();
    // utils::yf_Xs(xs.to_owned());
    let path = "./ref_data/yf_metas.csv".to_string();

    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(&yf::YF_META_HEADER);
    wtr.flush();
    for url in urls.iter() {
        if let Ok(body) = getters::simple_get(url.clone()) {
            if let Ok(root) = serde_json::from_str(&body.to_string()) {
                let rec = csv::StringRecord::from(yf::Root::meta_record(&root));
                println!("{:?}", rec);
                wtr.write_record(&rec);
            }
        } else {
            continue;
        }
    }
    wtr.flush();
    // println!("{:#?}", getters::simple_get(x.to_string()));
    // if let Some(recs) = getters::yf_from_url(utils::yf_url(x.to_owned())) {
    // for r in recs.iter() {
    //     println!("{:?}", r);
    // }
    Ok(())
}

#[tokio::main]
async fn async_main(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // let urls: Vec<String> = utils::yf_x_urls().into_iter().map(|x| utils::yf_url(x)).collect();
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        match reqwest::get(&url.clone()).await {
            Ok(resp) => match resp.json::<yf::Root>().await {
                Ok(root) => {
                    let recs: Vec<csv::StringRecord> = yf::Root::to_records(&root)
                        .into_iter()
                        .map(|x| csv::StringRecord::from(x))
                        .collect();

                    let symb = format!("{}", utils::yf_symb_from_url(url).unwrap());
                    println!("{} # records {}", symb, recs.len());
                    utils::writerecs_strvec(utils::simppath(symb.to_string()), utils::chart_headers(symb).to_vec(), recs);
                }
                Err(_) => println!("ERROR reading {}", url),
            },
            Err(_) => println!("ERROR downloading"),
        }
    }))
    .buffer_unordered(16)
    .collect::<Vec<()>>();
    fetches.await;
    Ok(())
}

// #[tokio::main]
// async fn async_many_one_to_one(
//     path: String,
//     urls: Vec<String>,
// ) -> Result<Vec<csv::StringRecord>, Box<dyn std::error::Error>> {
//     let (tx, rx) = mpsc::unbounded();
//     let mut wtr = csv::Writer::from_path(path)?;
//     wtr.write_record(&yf::YF_META_HEADER);
//     let mut recs: Vec<csv::StringRecord> = Vec::new();
//     let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
//         match reqwest::get(&url.clone()).await {
//             Ok(resp) => match resp.json::<yf::Root>().await {
//                 Ok(root) => {
//                     let rec: Vec<String> = yf::Root::meta_record(&root).await;
//                     // let symb = format!("{}", utils::yf_symb_from_url(url).unwrap());
//                     // println!("RESPONSE: {} # records {:?}", symb, rec);
//                     // Ok(rec)
//                     Some(rec)
//                     // wtr.write_record(&rec);
//                 }
//                 // _ => Err(format!("ERROR reading {}", url)),
//                 _ => None,
//             },
//             // _ => Err(format!("ERROR reading {}", url)),
//             _ => None,
//         }
//     }))
//     // .buffer_unordered(10)
//     .collect::<Vec<Option<Vec<String>>>>()
//     .await;
//     // .collect::<Vec<csv::StringRecord>>().await;
//     fetches.await;
//     Ok(recs)
// }

fn regexmain() -> Result<(), Box<dyn std::error::Error>> {
    // let file = File::open("rentec_13f.xml")?;
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    let res = vec![
        Regex::new(r"<nameOfIssuer>(?P<val>.+)</nameOfIssuer>.*()").unwrap(),
        Regex::new(r"<titleOfClass>(?P<val>.+)</titleOfClass>.*()").unwrap(),
        Regex::new(r"<cusip>(?P<val>.+)</cusip>.*()").unwrap(),
        Regex::new(r"<value>(?P<val>.+)</value>.*()").unwrap(),
        Regex::new(r"<sshPrnamt>(?P<val>.+)</sshPrnamt>.*()").unwrap(),
        Regex::new(r"<sshPrnamtType>(?P<val>.+)</sshPrnamtType>.*()").unwrap(),
        Regex::new(r"<investmentDiscretion>(?P<val>.+)</investmentDiscretion>.*()").unwrap(),
        Regex::new(r"<otherManager>(?P<val>.+)</otherManager>.*()").unwrap(),
        Regex::new(r"<Sole>(?P<val>.+)</Sole>.*()").unwrap(),
        Regex::new(r"<Shared>(?P<val>.+)</Shared>.*()").unwrap(),
        Regex::new(r"<None>(?P<val>.+)</None>.*()").unwrap(),
    ];
    // buf_reader.read_to_string(&mut contents)?;
    let filenames = utils::read_tickers("./rentec13urls.txt");
    for (i, url) in filenames.iter().enumerate() {
        let mut allcaps: Vec<Vec<String>> = Vec::new();
        let contents = getters::simple_get(url.to_string()).unwrap();
        for re in res.iter() {
            let mut rec: Vec<String> = Vec::new();
            for cap in re.captures_iter(&contents.to_string()) {
                if let Some(val) = cap.name("val") {
                    rec.push(val.as_str().to_string());
                } else {
                    println!("OH FUCK");
                    rec.push("".to_string());
                }
            }
            allcaps.push(rec);
        }
        let path = format!(
            "./ref_data/rentec/regex_rentec_holdings_{}.csv",
            i.to_string()
        );
        let mut wtr = csv::Writer::from_path(path)?;
        let len = allcaps[0].len();
        for vec in allcaps.iter() {
            assert_eq!(len, vec.len());
            let rec = csv::StringRecord::from(vec.clone());
            wtr.write_record(&rec);
        }
    }
    Ok(())
}

pub const SEC13F_HEADER: [&'static str; 11] = [
    "nameOfIssuer",
    "titleOfClass",
    "cusip",
    "value",
    "sshPrnamt",
    "sshPrnamtType",
    "investmentDiscretion",
    "otherManager",
    "Sole",
    "Shared",
    "None",
];
