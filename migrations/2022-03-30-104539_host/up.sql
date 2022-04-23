-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hosts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    url VARCHAR,
    free_tier BOOLEAN,
    frontend_support BOOLEAN DEFAULT true,
    backend_support BOOLEAN,
    database_support BOOLEAN,
    product_based BOOLEAN,
    plan_based BOOLEAN
)


  
