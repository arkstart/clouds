
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS domain_benefits (
    id SERIAL PRIMARY KEY,
    products_id integer NOT NULL,
    https_support BOOLEAN NOT NULL,
    free_domain BOOLEAN NOT NULL,
    custom_domain BOOLEAN NOT NULL,
    domain_extension VARCHAR NOT NULL,
    FOREIGN KEY (products_id) REFERENCES products (id)
)
