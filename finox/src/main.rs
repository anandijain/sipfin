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
    // utils::currencies()
    // utils::sp500("CMCSA".to_string(), false)
    // utils::news()
    // utils::prices("INTC".to_string())
    utils::commodities_prices("JX1".to_string())
}
