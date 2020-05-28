extern crate chrono;
extern crate reqwest;
extern crate roses;
extern crate tokio;

use std::path::Path;

//let symbs: Vec<&str> = finox::headers::CURRENCY_SYMBOLS_YF.to_vec(); //.into_iter().cloned().collect();
//let t = epoch_str();
//let urls = gen_yfx_urls(symbs);
//https://query1.finance.yahoo.com/v8/finance/chart/GOOG?lang=en-US&region=US&interval=1d&period1=0&period2=1590451200

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tickers = roses::read_tickers("../ref_data/tickers_stocks.txt");
    let urls = tickers.iter().map(|x| format!("https://query2.finance.yahoo.com/v8/finance/chart/{}?region=US&interval=1d&period1=345479400&period2=1590498425", x)).collect::<Vec<_>>();
    if let Ok(recs) = finox::fetch::<finox::yf::YFRoot>(urls).await {
        //println!("{:#?}", recs);
        let file_name = format!("../data/yf/yf_{}.csv", chrono::Utc::now().to_rfc3339());
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
