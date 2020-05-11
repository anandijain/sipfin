#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub symbol: String,
    pub price: f64,
}
