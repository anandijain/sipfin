//#![deny(warnings)]
extern crate csv;
extern crate reqwest;
extern crate tokio;

use futures::stream::StreamExt;
use nasdaq_o2;
use scraper::{Html, Selector};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let ciks = nasdaq_o2::read_tickers("../ref_data/ciks_parsed.txt");
    let filename = "./data/sec/counts.csv";
    let fp = Path::new(&filename);
    let fetches = futures::stream::iter(ciks.into_iter().map(|cik| async move {
        if let Ok(res) = reqwest::get(&cik_to_url(&cik)).await {
            if let Ok(textrsp) = res.text().await{
            let sel = Selector::parse("small").unwrap();
            let doc = Html::parse_document(&textrsp);
            for n in doc.select(&sel) {
                let text = n.text().collect::<Vec<_>>();
                for txtelt in text.iter() {
                    if txtelt.contains(&"Results") {
                        let split_txt: Vec<&str> = txtelt.split(' ').collect();
                        let num_filings = split_txt[2];
                        println!("{:#?}", cik);
                        return Some(vec![cik.to_string(), num_filings.to_string()]);
                    }
                }
            }
            return None;
            } else {
                return None;
            }
        } else {
            println!("response err{}", cik.clone());
            return None;
        }
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<String>>>>()
    .await;
    let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect();
    nasdaq_o2::write_csv(
        &fp,
        recs,
        vec!["cik".to_string(), "num_filings".to_string()],
    )
    .expect("csv prob");
    Ok(())
}

pub fn cik_to_url(s: &str) -> String {
    format!("https://sec.report/CIK/{}", s)
}
