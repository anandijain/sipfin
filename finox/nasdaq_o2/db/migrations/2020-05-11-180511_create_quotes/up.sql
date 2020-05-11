-- Your SQL goes here
CREATE TABLE quotes (
  id BIGSERIAL PRIMARY KEY,
  symbol VARCHAR NOT NULL,
  price DECIMAL NOT NULL
)
