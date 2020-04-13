extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
mod getters;
mod types;
mod news;
mod utils;

fn main() -> Result<(), reqwest::Error> {
    // utils::currencies_intraday("INR".to_string())
    // utils::sp500("CMCSA".to_string(), false)
    // utils::news()
    // utils::stock_prices("INTC".to_string())
    // utils::commodities_prices("LMCADS03".to_string())
    utils::stock_intraday("AMGN".to_string())
}
