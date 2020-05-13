pub mod nasdaq;

// pub use self::schema::quotes;
pub mod schema;

pub use self::models::{NewQuote, Quote};
pub mod models;
// pub mod db;
// pub trait Recordize {
//     fn to_(&self) -> String;
// }