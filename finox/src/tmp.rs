// https://www.defense.gov/data.json

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
// fn sync_main(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
//     // let xs: Vec<utils::Security> = utils::yf_x_urls();
//     // utils::yf_Xs(xs.to_owned());
//     let path = "./ref_data/yf_metas.csv".to_string();

//     let mut wtr = csv::Writer::from_path(path)?;
//     wtr.write_record(&yf::YF_META_HEADER);
//     wtr.flush();

//     for url in urls.iter() {
//         if let Ok(body) = getters::simple_get(url.clone()) {
//             if let Ok(root) = serde_json::from_str(&body.to_string()) {
//                 let rec = csv::StringRecord::from(yf::Root::meta_record(&root));
//                 println!("{:?}", rec);
//                 wtr.write_record(&rec)?;
//             }
//         } else {
//             continue;
//         }
//     }
//     wtr.flush();
//     // println!("{:#?}", getters::simple_get(x.to_string()));
//     // if let Some(recs) = getters::yf_from_url(utils::yf_url(x.to_owned())) {
//     // for r in recs.iter() {
//     //     println!("{:?}", r);
//     // }
//     Ok(())
// }
