extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

mod getters;
mod types;
mod news;
mod utils;
mod yf;

fn main() -> Result<(), reqwest::Error> {
    // utils::currencies_intraday("INR".to_string())
    // utils::sp500("CMCSA".to_string(), false)
    // utils::news()
    // utils::stock_prices("CB".to_string())
    // utils::commodities_prices("LMCADS03".to_string())
    // utils::stock_intraday("AMGN".to_string())
    // utils::hs_and_st()
    utils::yf_cur_today();

    Ok(())
}
