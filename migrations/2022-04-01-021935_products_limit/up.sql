-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products_limit (
    id SERIAL PRIMARY KEY,
    products_id integer NOT NULL,
    build_limit VARCHAR,
    bandwith_limit VARCHAR,
    site_limit VARCHAR,
    FOREIGN KEY (products_id) REFERENCES products (id)
)
