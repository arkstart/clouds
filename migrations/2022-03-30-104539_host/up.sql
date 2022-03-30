-- Your SQL goes here
CREATE TABLE host (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    url VARCHAR NOT NULL
)
