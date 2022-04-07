-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    hosts_id integer NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    build_limit VARCHAR,
    bandwith_limit VARCHAR,
    site_limit VARCHAR,
    https_support BOOLEAN,
    free_domain BOOLEAN,
    custom_domain BOOLEAN,
    domain_extension VARCHAR,
    FOREIGN KEY (hosts_id) REFERENCES hosts (id)
)
