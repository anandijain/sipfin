extern crate chrono;
extern crate reqwest;
extern crate roses;
extern crate tokio;

use std::path::Path;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut urls = vec![]; 
    let root = "https://www.cmegroup.com/CmeWS/mvc/Quotes/Future/";
    for i in 1..1000 {
        urls.push(format!("{}{}/G", root, i));
    }

    if let Ok(recs) = finox::fetch::<finox::cme::CMERoot>(urls).await {
        println!("{:#?}", recs);
        let file_name = format!("../data/cme/cme_{}.csv", chrono::Utc::now().to_rfc3339());
        let file_path = Path::new(&file_name);
        roses::write_csv(file_path, recs, &finox::headers::CME_QUOTE_HEADER).expect("csv prob");
    }
    Ok(())
}


