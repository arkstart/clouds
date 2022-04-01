-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    free BOOLEAN,
    pricing VARCHAR,
    
    build_limit VARCHAR,
    bandwith_limit VARCHAR
    site_limit VARCHAR
    FOREIGN KEY (hosts_id) REFERENCES hosts (id)
)
