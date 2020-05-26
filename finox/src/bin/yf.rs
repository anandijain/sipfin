extern crate chrono;
extern crate csv;
extern crate regex;
extern crate reqwest;
extern crate roses;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tickers = roses::read_tickers("../ref_data/tickers_stocks.txt");
    //let symbs: Vec<&str> = finox::headers::CURRENCY_SYMBOLS_YF.to_vec(); //.into_iter().cloned().collect();
    //let t = epoch_str(); 
    //let urls = gen_yfx_urls(symbs);
    let urls = tickers.iter().map(|x| format!("https://query2.finance.yahoo.com/v8/finance/chart/{}?interval=1d&period1=0&period2=999999999", x)).collect::<Vec<_>>();
    if let Ok(recs) = finox::fetch::<finox::yf::YFRoot>(urls).await {
        println!("{:#?}", recs);
        let file_name = format!(
            "../data/yf/yf_{}.csv",
            chrono::Utc::now().to_rfc3339()
        );
        let file_path = Path::new(&file_name);
        roses::write_csv(file_path, recs, &finox::headers::YF_HEADER).expect("csv prob");
    }
    Ok(())
}

// TODO look into the correct pattern for interlacing like this
pub fn gen_yfx_urls(symbs: Vec<&str>) -> Vec<String> {
    let len = symbs.len();
    let mut urls: Vec<String> = vec![];
    for (i, x) in symbs.iter().enumerate() {
        for y in symbs[i..len].iter() {
            if x == y {
                continue;
            };
            urls.push(
            format!(
                //"https://query2.finance.yahoo.com/v8/finance/chart/{}=F?interval=1d&period1=0&period2=1589932800",
                "https://query1.finance.yahoo.com/v8/finance/chart/{}{}=X?interval=1d&period1=0&period2=1589932800",
                x.to_string(),
                y.to_string()
            )
        );
        }
    }
    urls
}
