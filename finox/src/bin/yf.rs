extern crate chrono;
extern crate reqwest;
extern crate roses;
extern crate tokio;

use std::{collections::HashMap, env};

//let symbs: Vec<&str> = finox::headers::CURRENCY_SYMBOLS_YF.to_vec(); //.into_iter().cloned().collect();
//let t = epoch_str();
//let urls = gen_yfx_urls(symbs);
//https://query1.finance.yahoo.com/v8/finance/chart/GOOG?lang=en-US&region=US&interval=1d&period1=0&period2=1590451200

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* args:
     * 1: one in ['stocks', 'commodities', 'currencies']
     */

    let args = env::args().collect::<Vec<String>>();
    let (tickers, headers) = finox::gen_secs(&args[1]);
    let mut hm: HashMap<String, String> = HashMap::new();
    for symb in tickers.iter() {
        hm.insert(symb.to_string(), symb.to_yf());
    }
    if let Ok(recs) = finox::fetch_write::<finox::yf::YFRoot>(hm, "../data/yf/", headers).await {
        println!("{:#?}", recs);
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
