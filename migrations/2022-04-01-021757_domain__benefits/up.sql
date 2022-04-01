-- Your SQL goes here
CREATE TABLE IF NOT EXISTS domain_benefits (
    id SERIAL PRIMARY KEY,
    products_id integer NOT NULL,
    https_support BOOLEAN,
    free_domain BOOLEAN,
    custom_domain BOOLEAN,
    domain_extension VARCHAR,
    FOREIGN KEY (products_id) REFERENCES products (id)
)
