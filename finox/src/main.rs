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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // async_main(urls);

    // qid NOT NULL AUTO_INCREMENT, \
    // inline recipe definition
    let sql = " 
                CREATE TABLE Quote ( \
                    symb varchar(255), \
                    company varchar(255), \
                    last_trade_t varchar(255), \
                    last_sale_price varchar(255), \
                    PRIMARY KEY(symb));
                    
                QUERY getQuotes: \
                    SELECT * from Quote;";

    let persistence_params = noria_server::PersistenceParameters::new(
        noria_server::DurabilityMode::Permanent,
        Duration::from_millis(1),
        Some(String::from("example")),
        1,
    );

    let mut builder = Builder::default();

    builder.log_with(noria_server::logger_pls());
    builder.set_persistence(persistence_params);
    let (mut blender, done) = builder.start_local().await.unwrap();

    blender.install_recipe(sql).await.unwrap();
    println!("{}", blender.graphviz().await.unwrap());
    let mut quote = blender.table("Quote").await.unwrap();

    let tickers = utils::read_tickers("./ref_data/nasdaqtickers.txt");
    let urls: Vec<String> = tickers
        .iter()
        .map(|x| {
            format!(
                "https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks",
                x.to_string()
            )
        })
        .collect();
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            if let Ok(root) = res.json::<ndaq::AssetRoot>().await {
                // println!("GANG");
                // quote
                //     .insert(
                let q: Vec<String> = vec![
                    // i.into(),
                    root.data.symbol.into(),
                    root.data.company_name.into(),
                    root.data.primary_data.last_trade_timestamp.into(),
                    root.data.primary_data.last_sale_price.into(),
                ];

                println!("{:#?}", q.clone());
                return Some(q);
                // )
                // .await
                // .unwrap();
            }
            return None;
            // else {
            //     println!("NO GANG");

            // }
        }
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<String>>>>();
    println!("{:#?}", fetches);
    println!("Finished writing! Let's wait for things to propagate...");
    tokio::time::delay_for(Duration::from_secs(1)).await;
    // let xs: Vec<DataType::i64> = (1..10).collect();
    // println!("Reading...");
    // for i in 1..30 {
    //     println!("{:#?}", quotes.lookup(&[i.into()], true).await);
    // }

    let vecs = fetches.await;

    for v in vecs.iter() {
        if let Some(rec) = v {
            let noria_rec: Vec<DataType> = rec.iter().map(|x| x.to_string().into()).collect();
            quote.insert(noria_rec).await.unwrap();
        }
    }

    println!("{:#?}", blender.view("getQuotes").await.unwrap());
    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // utils::nytarchive();
//     // utils::nytfeed();
//     // utils::gsnews();
//     // utils::jpxnews();
//     // utils::reuters();
//     // utils::wsj_videos();
//     // utils::sa();
//     // bloomberg::news();
//     bloomberg::sp500("A".to_string(), true);
//     // regexmain();
//     // let urls = utils::read_tickers("./ref_data/hist_symbs.txt").iter().map(|x| format!(
//     //         "https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks",
//     //         x.to_string())).collect();
//     // // let start_times = utils::read_tickers("./ref_data/first_trade_date.txt");
//     // // assert!(tickers.len() == start_times.len());
//     // async_main(urls);
//     // let todo_symbs = &urls[index..urls.len()];
//     // sync_main(todo_symbs.to_vec());

//     Ok(())
// }

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
                wtr.write_record(&rec)?;
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
    let file_name = "./ref_data/ndaq_new.csv";
    // let file = std::fs::OpenOptions::new().append(true).open(file_name.to_string())?;
    let mut file = std::fs::File::create(file_name.to_string())?;
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&ndaq::NDAQ_QUOTE_HEADER);
    wtr.flush()?;
    // wtr.close();

    // let urls: Vec<String> = utils::yf_x_urls().into_iter().map(|x| utils::yf_url(x)).collect();
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        match reqwest::get(&url.clone()).await {
            Ok(resp) => match resp.json::<ndaq::AssetRoot>().await {
                Ok(root) => {
                    // let recs: Vec<csv::StringRecord> = ndaq::AssetRoot::to_records(&root)
                    //     .into_iter()
                    //     .map(|x| csv::StringRecord::from(x))
                    //     .collect();
                    let asset = ndaq::AssetRoot::to_record(&root);
                    let rec: csv::StringRecord = csv::StringRecord::from(asset.clone());
                    let symb = format!("{}", utils::symb_from_ndaq_url(url.clone()).unwrap());
                    println!("{} {:#?}", symb, asset.clone());

                    // let file = std::fs::OpenOptions::new().append(true).open(file_name)?;
                    // let mut wtr = csv::Writer::from_writer(file);
                    // wtr.write_record(&rec);
                    // wtr.flush().await?;

                    // utils::writerecs_strvec(utils::simppath(symb.to_string()), utils::chart_headers(symb).to_vec(), rec);
                    // utils::appendrec(utils::simppath(symb.to_string()), rec);
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
// async fn iomain() -> io::Result<()> {
//     let mut f = File::open("./ref_data/foo.csv").await?;
//     for i in 1..10 {
//         let mut writer = io::BufWriter::new(f);
//         writer.write(&[i]).await?;

//     }
//     // {
//     //     let mut writer = io::BufWriter::new(f);

//     //     // write a byte to the buffer
//     //     writer.write(&[42u8]).await?;
//     // }
//     // read up to 10 bytes

//     Ok(())
// }
