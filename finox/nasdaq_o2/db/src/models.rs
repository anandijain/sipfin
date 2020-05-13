use super::schema::quotes;
#[derive(Queryable)]
pub struct Quote {
    pub id: i64,
    pub ticker: String,
    pub price: f32,
    // pub published: bool,
}


#[derive(Insertable)]
#[table_name="quotes"]
pub struct NewQuote<'a> {
    pub id: &'a i64,
    pub ticker: &'a str,
    pub price: &'a f32,
}