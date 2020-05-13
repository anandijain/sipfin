pub mod nasdaq;

// pub use self::schema::quotes;
pub mod schema;

pub use self::models::{NewQuote, Quote};
pub mod models;

#[macro_use]
extern crate diesel;
pub use diesel::prelude::*;
// pub mod db;
// pub trait Recordize {
//     fn to_(&self) -> String;
// }