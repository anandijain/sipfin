//#![deny(warnings)]
extern crate reqwest;
extern crate roses;
extern crate serde;
extern crate tokio;
use futures::stream::StreamExt;
use scraper::{Html, Selector};
use std::path::Path;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    cik: String,
    name: String,
    form: String,
    href: String,
    date: String,
}
/*
 * 1. go to cik root
 * 2. get # filings
 * 3. gen urls w pages
 * 4. collect all pages into single v<v<str>>
 * 5. write cik to unique csv in async block
 */
pub const SEC_DOCS_HEADER: [&'static str; 3] = ["cik", "url", "nfilings"];

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let ciks = roses::read_tickers("../ref_data/ciks_parsed.txt");
    let _fetches = futures::stream::iter(ciks.into_iter().map(|cik| async move {
        if let Ok(res) = reqwest::get(&cik_to_url(&cik)).await {
            if let Ok(textrsp) = res.text().await {
                if let Some(doc_urls) = cik_docs(&textrsp) {
                    if let Some(np_int) = nresults(&textrsp) {
                        let file_name = format!(
                            "../data/sec/{}_{}.csv",
                            cik,
                            chrono::Utc::now().to_rfc3339()
                        );
                        let file_path = Path::new(&file_name);
                        let recs = doc_urls
                            .iter()
                            .map(|x| vec![cik.clone(), x.to_string(), np_int.to_string()])
                            .collect();
                        roses::write_csv(file_path, recs, &SEC_DOCS_HEADER).expect("csv err");
                    }
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
    Ok(())
}

pub fn cik_to_url(s: &str) -> String {
    format!("https://sec.report/CIK/{}", s)
}

pub fn nresults(textrsp: &str) -> Option<u64> {
    let sel = Selector::parse("small").unwrap();
    let doc = Html::parse_document(&textrsp);
    // docs on current page
    //let docs_sel = Selector::parse("");
    // num pages
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
pub fn cik_docs(textrsp: &str) -> Option<Vec<String>> {
    let sel = Selector::parse("div").unwrap();
    let a_tags = Selector::parse("a").unwrap();
    let trows = Selector::parse("tr").unwrap();
    let doc = Html::parse_document(&textrsp);
    let mut hrefs = vec![];
    for (i, div) in doc.select(&sel).enumerate() {
        if let Some("documents") = div.value().attr("id") {
            println!("{}: {:#?}", i, div.value());
            for (_j, divbody) in div.select(&sel).enumerate() {
                if let Some("panel-body") = divbody.value().attr("class") {
                    for (j, tr) in div.select(&trows).enumerate() {
                        for a_tag in tr.select(&a_tags) {
                            let href = a_tag.value().attr("href").unwrap().to_string();
                            println!("{} {}: {:#?}", i, j, href);
                            hrefs.push(href);
                        }
                    }
                }
            }
        }
    }

    return Some(hrefs);
}
