use crate::schema::quotes;

#[derive(diesel::Queryable, Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Quote {
    pub id: i32,
    pub symbol: String,
    pub company_name: String,
    pub stock_type: String,
    pub exchange: String,
    pub is_nasdaq_listed: String,
    pub is_nasdaq100: String,
    pub is_held: String,
    pub last_trade_timestamp: String,
    pub last_sale_price: String,
    pub net_change: String,
    pub percentage_change: String,
    pub is_real_time: String,
    pub delta_indicator: String,
}


#[derive(diesel::Insertable, Clone)]
#[table_name="quotes"]
pub struct NewQuote<'a> {
    pub symbol: &'a str,
    pub company_name: &'a str,
    pub stock_type: &'a str,
    pub exchange: &'a str,
    pub is_nasdaq_listed: &'a str,
    pub is_nasdaq100: &'a str,
    pub is_held: &'a str,
    pub last_trade_timestamp: &'a str,
    pub last_sale_price: &'a str,
    pub net_change: &'a str,
    pub percentage_change: &'a str,
    pub is_real_time: &'a str,
    pub delta_indicator: &'a str,
}
