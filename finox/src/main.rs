extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod getters;
mod types;
mod news;
mod utils;

fn main() -> Result<(), csv::Error> {
    // utils::currencies();
    // utils::sp500("CMCSA".to_string(), false)
    utils::news()
}
