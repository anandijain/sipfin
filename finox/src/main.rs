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

mod getters;
mod keys;
mod news;
mod sa;
mod steam;
mod types;
mod utils;
mod yf;
//old
// utils::currencies_intraday("INR".to_string())
// utils::sp500("CMCSA".to_string(), false)
// utils::news()
// utils::stock_prices("CB".to_string())
// utils::commodities_prices("LMCADS03".to_string())
// utils::stock_intraday("AMGN".to_string())
// utils::hs_and_st()

fn main() -> Result<(), reqwest::Error> {
    // let urls: Vec<String> = utils::yf_us_urls();
    // let t1 = Instant::now();
    // sync_main(urls);
    // println!("{}", t1.elapsed().as_secs());
    let urls2: Vec<utils::Security> = utils::f_securities();
    let t2 = Instant::now();
    async_main(urls2);
    println!("{}", t2.elapsed().as_secs());
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
    // let client = reqwest::Client::builder().build()?;
    let fetches = futures::stream::iter(secs.into_iter().map(|symb| async move {
        match reqwest::get(&utils::yf_url(symb.clone())).await {
            Ok(resp) => match resp.json::<yf::Root>().await {
                Ok(root) => {
                    let recs: Vec<csv::StringRecord> = yf::Root::to_records(&root)
                        .into_iter()
                        .map(|x| csv::StringRecord::from(x))
                        .collect();
                    // println!("{}", recs.len());

                    // for r in recs.iter() {
                    //     println!("{:?}", r);
                    // }
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
//         match reqwest::get(&url).await {
//             Ok(resp) => {
//                 match resp.json::<yf::Root>().await {
//                     Ok(json) => {
//                         println!("RESPONSE: bytes from {}", url);
//                     }
//                     Err(_) => println!("ERROR reading {}", url),
//                 }
//             }
//             Err(_) => println!("ERROR downloading {}", url),
//         }
//         Ok(())
//     }));
//     }
//     join_all(tasks).await;
//     Ok(())
// }

// #[tokio::main]
// async fn tmpmain() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }
