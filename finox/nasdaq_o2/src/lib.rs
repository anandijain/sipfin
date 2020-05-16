pub mod nasdaq;

pub mod schema;

pub use self::models::{NewQuote, Quote};
pub mod models;

#[macro_use]
extern crate diesel;
pub use diesel::prelude::*;
