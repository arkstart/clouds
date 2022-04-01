-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    build_limit VARCHAR,
    bandwith_limit VARCHAR
    site_limit VARCHAR
    FOREIGN KEY (products_id) REFERENCES products (id)
)
