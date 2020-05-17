#![deny(warnings)]
extern crate reqwest;
extern crate csv;
extern crate tokio;
use nasdaq_o2;
//use std::error::Error;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let links = urls();
    let sel = Selector::parse("small").unwrap();

    let mut wtr = csv::Writer::from_path("../../../tmp.csv").expect("csv errr");


    for elt in links.iter() {
        let res = reqwest::get(elt).await?.text().await?;
        let doc = Html::parse_document(&res);
        //for smol in doc.select(&sel);
        for n in doc.select(&sel) {
            println!("{:#?}", n.text());
        }
        //break;
    }
    Ok(())
}

pub fn urls() -> Vec<String> {
    nasdaq_o2::read_tickers("/home/sippycups/sipfin/ref_data/ciks_parsed.txt")
        .iter()
        .map(|x| cik_to_url(x))
        .collect()
}

pub fn cik_to_url(s: &str) -> String {
    format!("https://sec.report/CIK/{}", s)
}
