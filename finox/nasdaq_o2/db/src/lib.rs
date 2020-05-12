#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Quote, NewQuote};

pub fn create_quote<'a>(conn: &PgConnection, id: &'a i64, ticker: &'a str, price: &'a f64) -> Quote {
    use schema::quotes;

    let new_quote = NewQuote {
        id: id
        ticker: ticker
        price: price
    };

    diesel::insert_into(posts::table)
        .values(&new_quote)
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


