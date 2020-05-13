extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::quotes::dsl::*;

    let connection = establish_connection();
    let results = quotes.filter(published.eq(true))
        .limit(5)
        .load::<Quote>(&connection)
        .expect("Error loading quotes");

    println!("Displaying {} quotes", results.len());
    for quote in results {
        println!("{}", quote.id);
        println!("----------\n");
        println!("{}", quote.ticker);
        println!("----------\n");
        println!("{}", quote.price);
    }
}