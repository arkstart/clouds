-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    hosts_id integer NOT NULL,
    title VARCHAR NOT NULL,
    subtitle VARCHAR,
    description VARCHAR,
    category VARCHAR NOT NULL,
    product_url VARCHAR,
    free_tier BOOLEAN, -- Always Free
    free_trial BOOLEAN,
    base_price FLOAT,
    price_unit VARCHAR,
    price_timeunit VARCHAR,
    price_desc VARCHAR,
    multi_pricing BOOLEAN,
    FOREIGN KEY (hosts_id) REFERENCES hosts (id)
)
