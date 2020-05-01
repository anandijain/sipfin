extern crate chrono;
extern crate csv;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
use noria::DataType;
use noria_server::Builder;

use futures::stream::StreamExt;
use regex::Regex;
use std;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

mod bloomberg;
mod getters;
mod gs;
mod jpxnews;
mod keys;
mod ndaq;
mod news;
mod sa;
mod steam;
mod utils;
mod weather;
mod xue;
mod xueqiu;
mod yf;
// let index = urls
//     .iter()
//     .position(|r| {
//         r.to_string()
//             == "https://query1.finance.yahoo.com/v8/finance/chart/TOUR?region=US&range=1d"
//                 .to_string()
//     })
//     .unwrap();
// urls.push(format!(
//     "https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks",
//     tickers[i].to_string()
// ));

fn newsmain() -> Result<(), Box<dyn std::error::Error>> {
    // utils::nytarchive();
    utils::nytfeed();
    utils::gsnews();
    utils::jpxnews();
    utils::reuters();
    utils::wsj_videos();
    utils::sa();
    bloomberg::news();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // async_main(urls);

    // id NOT NULL, \
    // let sql = " 
    //             CREATE TABLE Quote ( \
    //                 qid int, \
    //                 symbol varchar(255), \
    //                 company_name varchar(255), \
    //                 stock_type varchar(255), \
    //                 exchange varchar(255), \
    //                 is_nasdaq_listed varchar(255), \
    //                 is_nasdaq100 varchar(255), \
    //                 is_held varchar(255), \
    //                 last_trade_timestamp varchar(255), \
    //                 last_sale_price varchar(255), \
    //                 net_change varchar(255), \
    //                 percentage_change varchar(255), \
    //                 is_real_time varchar(255), \
    //                 delta_indicator varchar(255), \
    //                 PRIMARY KEY(qid));

    //             CREATE TABLE YFQuote ( \
    //                 qid int, \
    //                 symbol varchar(255), \
    //                 t varchar(255), \
    //                 o varchar(255), \
    //                 h varchar(255), \
    //                 l varchar(255), \
    //                 c varchar(255), \
    //                 v varchar(255), \
    //                 PRIMARY KEY(qid));

    //             CREATE TABLE XueQiuQuote ( \
    //                 qid int, \
    //                 symbol varchar(255), \
    //                 timestamp varchar(255), \
    //                 current varchar(255), \
    //                 trade_volume varchar(255), \
    //                 volume varchar(255), \
    //                 open varchar(255), \
    //                 high varchar(255), \
    //                 low varchar(255), \
    //                 last_close varchar(255), \
    //                 avg_price varchar(255), \
    //                 amount varchar(255), \
    //                 percent varchar(255), \
    //                 chg varchar(255), \
    //                 market_capital varchar(255), \
    //                 turnover_rate varchar(255), \
    //                 amplitude varchar(255), \
    //                 current_year_percent varchar(255), \
    //                 level varchar(255), \
    //                 trade_session varchar(255), \
    //                 PRIMARY KEY(qid));
                    
    //             QUERY getQuotes: \
    //                 SELECT Quote.qid, symbol, last_trade_timestamp,  from Quote;";

    // let persistence_params = noria_server::PersistenceParameters::new(
    //     noria_server::DurabilityMode::Permanent,
    //     Duration::from_millis(1),
    //     Some(String::from("example")),
    //     1,
    // );

    // let mut builder = Builder::default();

    // builder.log_with(noria_server::logger_pls());
    // builder.set_persistence(persistence_params);
    // let (mut blender, done) = builder.start_local().await.unwrap();

    // blender.install_recipe(sql).await.unwrap();
    // println!("{}", blender.graphviz().await.unwrap());
    // let mut quote = blender.table("Quote").await.unwrap();
    // let mut yfquote = blender.table("YFQuote").await.unwrap();
    // let mut quote = blender.table("XueQiuQuote").await.unwrap();

    let tickers = utils::read_tickers("./ref_data/tickers.txt");
    let urls: Vec<String> = tickers
        .iter()
        .map(|x| {
            format!(
                "https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks",
                // "https://stock.xueqiu.com/v5/stock/realtime/quotec.json?symbol={}",
                // "https://query1.finance.yahoo.com/v8/finance/chart/{}?region=US&range=7d",
                x.to_string()
            )
        })
        .collect();


    
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            // println!("ok {}", url.clone());
            if let Ok(root) = res.json::<ndaq::AssetRoot>().await {
                // println!("{}, {:?}", url.clone(), root.clone());
                // let symb = utils::yf_symb_from_url(url.clone()).unwrap().to_string();
                // let recs: Vec<csv::StringRecord> = yf::Root::to_records(&root).iter().map(|x| csv::StringRecord::from(x.clone())).collect();
                // let mut wtr = csv::Writer::from_path(utils::simppath(symb.clone(), "yf7d".to_string()));
                // match wtr {
                //     Ok(mut wtr) => {
                //     wtr.write_record(&utils::YF_HEADER);
                //     for r in recs.iter() {
                //         wtr.write_record(r);
                //     }
                //     wtr.flush();
                // },
                //     Err(_) => return None

                // }
                // utils::writerecs(, &utils::YF_HEADER, recs.clone());
                // let num_recs = recs.len();
                    // .iter()
                    // .map(|x| x.iter().map(|y| DataType::from(y.to_string())).collect())
                    // .collect();

                // let q: Vec<DataType> = vec![
                //     root.data.symbol.into(),
                //     root.data.company_name.into(),
                //     root.data.stock_type.into(),
                //     root.data.exchange.into(),
                //     root.data.is_nasdaq_listed.to_string().into(),
                //     root.data.is_nasdaq100.to_string().into(),
                //     root.data.is_held.to_string().into(),
                //     root.data.primary_data.last_trade_timestamp.into(),
                //     root.data.primary_data.last_sale_price.into(),
                //     root.data.primary_data.net_change.into(),
                //     root.data.primary_data.percentage_change.into(),
                //     root.data.primary_data.is_real_time.to_string().into(),
                //     root.data.primary_data.delta_indicator.into(),
                // ];

                let csvq: Vec<String> = vec![
                    root.data.symbol.to_string(),
                    root.data.stock_type.to_string(),
                    root.data.exchange.to_string(),
                    root.data.primary_data.last_trade_timestamp.to_string(),
                    root.data.primary_data.last_sale_price.to_string(),
                    root.data.primary_data.net_change.to_string(),
                    root.data.primary_data.percentage_change.to_string(),
                    root.data.primary_data.is_real_time.to_string().to_string(),
                    root.data.primary_data.delta_indicator.to_string(),
                ];
                let symb = csvq[0].clone();
                // println!("{}: {}, {:#?}", symb.clone(), num_recs, recs.last());
                println!("{}", symb.clone());
                return Some(csvq);
            }
            println!("no");
            return None;
        }
        println!("no2");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<String>>>>();
    let vecs = fetches.await;
    println!("{:#?}", vecs);
    // tokio::time::delay_for(Duration::from_secs(1)).await;
    let file_name = "nasdaq_new2.csv".to_string();
    // let file = std::fs::OpenOptions::new().append(true).open(file_name)?;
    // let mut wtr = csv::Writer::from_writer(file);
    let mut wtr = csv::Writer::from_path(file_name)?;
    wtr.write_record(vec![
        "symbol",
        "stock_type",
        "exchange",
        "last_trade_timestamp",
        "last_sale_price",
        "net_change",
        "percentage_change",
        "is_real_time",
        "delta_indicator",
    ]);
    
    let mut i: i64 = 0;
    for v in vecs.iter() {
        if let Some(rec) = v {
            // i += 1;
            // let mut to_write: Vec<DataType> = vec![i.into()];
            // to_write.append(&mut rec.clone());
            // println!("{:?}", to_write.clone());
            // quote.insert(to_write).await.unwrap();
            wtr.write_record(rec);
        }
    }
    wtr.flush();

    
    // let mut viewQuotes = blender.view("getQuotes").await.unwrap();
    // let lookup_id = 2;
    // tokio::time::delay_for(Duration::from_millis(1000)).await;
    // let a_quote = viewQuotes.lookup(&[lookup_id.into()], true).await.unwrap();
    // // let view_two = quote.view
    // println!("{:#?}{:#?}", viewQuotes, a_quote);

    Ok(())
}

// #[tokio::main]
// async fn async_main(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
//     let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
//         match reqwest::get(&url.clone()).await {
//             Ok(resp) => match resp.json::<ndaq::AssetRoot>().await {
//                 Ok(root) => {
//                     let asset = ndaq::AssetRoot::to_record(&root);
//                     let rec: csv::StringRecord = csv::StringRecord::from(asset.clone());
//                     let symb = format!("{}", utils::symb_from_ndaq_url(url.clone()).unwrap());
//                     println!("{} {:#?}", symb, asset.clone());

//                 }
//                 Err(_) => println!("ERROR reading {}", url),
//             },
//             Err(_) => println!("ERROR downloading"),
//         }
//     }))
//     .buffer_unordered(16)
//     .collect::<Vec<()>>();
//     fetches.await;
//     Ok(())
// }

// fn paginate<'t>(v: Vec<'t>, n: usize) -> 'a | 'b ??
//
