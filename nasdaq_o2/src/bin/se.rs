//#![deny(warnings)]
extern crate csv;
extern crate reqwest;
extern crate tokio;
extern crate serde;
use futures::stream::StreamExt;
use nasdaq_o2;
use scraper::{Html, Selector};
use std::path::Path;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    cik: String,
    name: String,
    form: String, 
    href: String,
    date: String
}
/* 
 * 1. go to cik root
 * 2. get # filings 
 * 3. gen urls w pages
 * 4. collect all pages into single v<v<str>> 
 * 5. write cik to unique csv in async block
 */


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let ciks = nasdaq_o2::read_tickers("../ref_data/ciks_parsed.txt");
    let filename = "./data/sec/counts.csv";
    let fp = Path::new(&filename);
    let fetches = futures::stream::iter(ciks.into_iter().map(|cik| async move {
        if let Ok(res) = reqwest::get(&cik_to_url(&cik)).await {
            if let Ok(textrsp) = res.text().await{
                if let Some(np_int) = nresults(&textrsp) {

                    let mut cik_urls = vec![];
                    for j in 1..=np_int {
                        cik_urls.push(cik_to_url(&cik).push_str(&format!("/{}", j)));
                    }
    
                    
                    println!("{:#?} {:?}", cik, np_int);  
                    //return Some(vec![cik.to_string(), num_filings.to_string()]);

                }
            } else {
                //return None;
            }
        } else {
            println!("response err{}", cik.clone());
            //return None;
        }
    }))
    .buffer_unordered(16)
    .collect::<Vec<_>>()
    .await;
    //let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect();
    //nasdaq_o2::write_csv(
    //    &fp,
    //    recs,
    //    vec!["cik".to_string(), "num_filings".to_string()],
    //)
    //.expect("csv prob");

    Ok(())
}

pub fn cik_to_url(s: &str) -> String {
    format!("https://sec.report/CIK/{}", s)
}

pub fn nresults(textrsp: &str) -> Option<u64> {
    let sel = Selector::parse("small").unwrap();
            let doc = Html::parse_document(&textrsp);
            for n in doc.select(&sel) {
                let text = n.text().collect::<Vec<_>>();
                for txtelt in text.iter() {
                    if txtelt.contains(&"Results") {
                        let split_txt: Vec<&str> = txtelt.split(' ').collect();
                        let num_filings = split_txt[2];
                        let num_pages: f64 = num_filings.parse::<f64>().unwrap() / 50.;
                        let np_int = num_pages.ceil() as u64;
                        return Some(np_int);
                    } 

                }
            }    
                    return None;
} 
