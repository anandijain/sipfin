//#![deny(warnings)]
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate tokio;
use finox::{roses, sec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = roses::read_into::<sec::SecIndex>("../ref_data/sec13f.csv").unwrap();
    let urls: Vec<String> = index
        .iter()
        .map(|x| format!("https://sec.gov/Archives/{}", x.filename))
        .collect();
    let headers = finox::fetch_strings(urls.to_vec()).await;
    println!("{:#?}", headers);
    Ok(())
}

//pub fn cik_to_url(s: &str) -> String {
//    format!("https://sec.report/CIK/{}", s)
//}
//
//pub fn nresults(textrsp: &str) -> Option<u64> {
//    let sel = Selector::parse("small").unwrap();
//    let doc = Html::parse_document(&textrsp);
//    // docs on current page
//    //let docs_sel = Selector::parse("");
//    // num pages
//    for n in doc.select(&sel) {
//        let text = n.text().collect::<Vec<_>>();
//        for txtelt in text.iter() {
//            if txtelt.contains(&"Results") {
//                let split_txt: Vec<&str> = txtelt.split(' ').collect();
//                let num_filings = split_txt[2];
//                let num_pages: f64 = num_filings.parse::<f64>().unwrap() / 50.;
//                let np_int = num_pages.ceil() as u64;
//                return Some(np_int);
//            }
//        }
//    }
//    return None;
//}
//pub fn cik_docs(textrsp: &str) -> Option<Vec<String>> {
//    let sel = Selector::parse("div").unwrap();
//    let a_tags = Selector::parse("a").unwrap();
//    let trows = Selector::parse("tr").unwrap();
//    let doc = Html::parse_document(&textrsp);
//    let mut hrefs = vec![];
//    for (i, div) in doc.select(&sel).enumerate() {
//        if let Some("documents") = div.value().attr("id") {
//            println!("{}: {:#?}", i, div.value());
//            for (_j, divbody) in div.select(&sel).enumerate() {
//                if let Some("panel-body") = divbody.value().attr("class") {
//                    for (j, tr) in div.select(&trows).enumerate() {
//                        for a_tag in tr.select(&a_tags) {
//                            let href = a_tag.value().attr("href").unwrap().to_string();
//                            println!("{} {}: {:#?}", i, j, href);
//                            hrefs.push(href);
//                        }
//                    }
//                }
//            }
//        }
//    }
//
//    return Some(hrefs);
//}
