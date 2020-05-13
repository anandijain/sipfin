-- Your SQL goes here
CREATE TABLE quotes (
  id BIGSERIAL PRIMARY KEY,
  ticker VARCHAR NOT NULL,
  price DECIMAL NOT NULL
)
