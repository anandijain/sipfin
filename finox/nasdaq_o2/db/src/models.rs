use super::schema::quotes;


#[derive(Queryable)]
pub struct Quote {
    pub id: i64,
    pub ticker: String,
    pub price: f64,
    // pub published: bool,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct NewQuote<'a> {
    pub id: &'a i64
    pub ticker: &'a String,
    pub price: &'a f64,
}