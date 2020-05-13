-- Your SQL goes here

-- CREATE TABLE quotes (
--   id BIGSERIAL PRIMARY KEY,
--   ticker VARCHAR NOT NULL,
--   price VARCHAR NOT NULL
-- )

CREATE TABLE quotes (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR NOT NULL,
    company_name VARCHAR NOT NULL,
    stock_type VARCHAR NOT NULL,
    exchange VARCHAR NOT NULL,
    is_nasdaq_listed VARCHAR NOT NULL,
    is_nasdaq100 VARCHAR NOT NULL,
    is_held VARCHAR NOT NULL,
    last_trade_timestamp VARCHAR NOT NULL,
    last_sale_price VARCHAR NOT NULL,
    net_change VARCHAR NOT NULL,
    percentage_change VARCHAR NOT NULL,
    is_real_time VARCHAR NOT NULL,
    delta_indicator VARCHAR NOT NULL
)

