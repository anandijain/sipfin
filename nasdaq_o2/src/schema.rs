table! {
    quotes (id) {
        id -> Int4,
        symbol -> Varchar,
        company_name -> Varchar,
        stock_type -> Varchar,
        exchange -> Varchar,
        is_nasdaq_listed -> Varchar,
        is_nasdaq100 -> Varchar,
        is_held -> Varchar,
        last_trade_timestamp -> Varchar,
        last_sale_price -> Varchar,
        net_change -> Varchar,
        percentage_change -> Varchar,
        is_real_time -> Varchar,
        delta_indicator -> Varchar,
    }
}