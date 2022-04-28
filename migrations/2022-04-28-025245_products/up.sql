-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    hosts_id integer NOT NULL,
    title VARCHAR NOT NULL,
    subtitle VARCHAR,
    description VARCHAR,
    -- Analytic (ANLT), Storage (STRG), Databases (DTBS), Compute (CMPT), Containers (CNTN)
    category VARCHAR NOT NULL,
    product_url VARCHAR,
    free_tier BOOLEAN, -- Always Free
    free_trial BOOLEAN,
    base_price numeric,
    price_unit VARCHAR,
    price_timeunit VARCHAR,
    price_desc VARCHAR,
    multi_pricing BOOLEAN,
    FOREIGN KEY (hosts_id) REFERENCES hosts (id)
)
