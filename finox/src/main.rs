extern crate chrono;
extern crate csv;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use futures::future::join_all;
use futures::stream::StreamExt;
use futures::{executor::block_on, future::Future, stream::Stream};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use tokio::task::JoinHandle;

mod keys;
mod getters;
mod news;
mod sa;
mod steam;
mod utils;
mod yf;
mod weather;
mod bloomberg;
//old
// utils::currencies_intraday("INR".to_string())
// utils::sp500("CMCSA".to_string(), false)
// utils::news()
// utils::stock_prices("CB".to_string())
// utils::commodities_prices("LMCADS03".to_string())
// utils::stock_intraday("AMGN".to_string())
// utils::hs_and_st()

// async fn main() -> Result<(), reqwest::Error> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let urls: Vec<String> = utils::yf_us_urls();
    // let t1 = Instant::now();
    // sync_main(urls);
    // println!("{}", t1.elapsed().as_secs());
    // const mystr: 'static String = "hello";
    
    // let urls2: Vec<utils::Security> = utils::x_securities();
    // let t2 = Instant::now();
    // async_main(urls2);
    // println!("{}", t2.elapsed().as_secs());

    // weather::cities_to_csv();

    // utils::yf_US(Some("RDNT".to_string()));
    // block_on(async_main());
    // utils::yf_US(None);
    // utils::yf_X();
    // utils::yf_F();

    // utils::nytarchive();

    // utils::nytfeed();
    // utils::reuters();
    // utils::wsj_videos();
    // utils::sa();
    // utils::steam_purchases();

    // let urls: Vec<String> = vec!(
    //     // "https://finance.yahoo.com/_finance_doubledown/api/resource/YFinLists;listIds=%5B%22currencies%22%5D".to_string(),
    //     // "https://finance.yahoo.com/_finance_doubledown/api/resource/YFinLists;listIds=%5B%22commodities%22%5D".to_string(),
    //     // "https://finance.yahoo.com/_finance_doubledown/api/resource/YFinLists;listIds=%5B%22bonds%22%5D".to_string(),
    //     "https://www.jpx.co.jp/english/news/news_ym_01.json".to_string(),
        
    // );
    // async_yflist(urls);
    let urls = bloomberg::currency_urls();
    println!("{:#?}", urls);
    Ok(())
}


#[tokio::main]
async fn async_yflist(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    
    // let client = reqwest::Client::builder().build()
    let file_name = "jpxnewstest.csv".clone().to_string();
    let mut wtr = csv::Writer::from_path(file_name.clone())?;
    wtr.write_record(&mut news::JPXNewsHeader.iter());
    
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        match reqwest::get(&url.clone()).await {
            Ok(resp) => match resp.json::<Vec<news::JPXNews>>().await {
                Ok(root) => {
                    let recs: Vec<csv::StringRecord> = root
                        .into_iter()
                        .map(|x| csv::StringRecord::from(news::JPXNews::to_record(&x)))
                        .collect();
                    println!("RESPONSE: # records {}", recs.len());
                    // println!("RESPONSE: {:#?}", recs);
                    utils::appendrecs("jpxnewstest.csv".to_string(), recs);
                }
                Err(_) => println!("ERROR reading"),
            },
            Err(_) => println!("ERROR downloading"),
        }
    }))
    .buffer_unordered(16)
    .collect::<Vec<()>>();
    fetches.await;
    Ok(())
}

fn sync_main(secs: Vec<utils::Security>) -> Result<(), Box<dyn std::error::Error>> {
    // let xs: Vec<utils::Security> = utils::yf_x_urls();
    // utils::yf_Xs(xs.to_owned());
    for x in secs.iter() {
        if let Some(recs) = getters::yf_from_url(utils::yf_url(x.to_owned())) {
            println!("{}", recs.len());
            // for r in recs.iter() {
            //     println!("{:?}", r);
            // }
        }
    }
    Ok(())
}

#[tokio::main]
async fn async_main(secs: Vec<utils::Security>) -> Result<(), Box<dyn std::error::Error>> {
    // let urls: Vec<String> = utils::yf_x_urls().into_iter().map(|x| utils::yf_url(x)).collect();
    let fetches = futures::stream::iter(secs.into_iter().map(|symb| async move {
        match reqwest::get(&utils::yf_url(symb.clone())).await {
            Ok(resp) => match resp.json::<yf::Root>().await {
                Ok(root) => {
                    let recs: Vec<csv::StringRecord> = yf::Root::to_records(&root)
                        .into_iter()
                        .map(|x| csv::StringRecord::from(x))
                        .collect();
                    println!("RESPONSE: # records {}", recs.len());
                    utils::writerecs(utils::simppath(symb), &utils::YF_HEADER, recs);
                }
                Err(_) => println!("ERROR reading"),
            },
            Err(_) => println!("ERROR downloading"),
        }
    }))
    .buffer_unordered(16)
    .collect::<Vec<()>>();
    fetches.await;
    Ok(())
}

