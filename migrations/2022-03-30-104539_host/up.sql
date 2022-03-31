-- Your SQL goes here
CREATE TABLE IF NOT EXIST host (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    url VARCHAR NOT NULL
)
