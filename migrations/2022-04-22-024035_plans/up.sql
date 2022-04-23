-- Your SQL goes here
CREATE TABLE IF NOT EXISTS plans (
    id SERIAL PRIMARY KEY,
    hosts_id integer NOT NULL,
    name VARCHAR,
    description VARCHAR,
    plan_url VARCHAR
    price integer,
    price_unit VARCHAR,
    price_timeunit VARCHAR,
    price_desc VARCHAR,
    -- Concurrent Build
    concurrent_build integer,
    concurrent_build_unit VARCHAR,
    concurrent_build_timeunit VARCHAR,
    concurrent_build_desc VARCHAR,
    -- Bandwidth
    bandwidth integer,
    bandwidth_unit VARCHAR,
    bandwidth_timeunit VARCHAR,
    bandwidth_desc VARCHAR,
    -- Build
    build integer,
    build_unit VARCHAR,
    build_timeunit VARCHAR,
    build_desc VARCHAR,
    -- Analytic
    analytic BOOLEAN,
    analytic_price integer,
    analytic_unit VARCHAR,
    analytic_timeunit VARCHAR,
    analytic_desc VARCHAR,
    FOREIGN KEY (hosts_id) REFERENCES hosts (id)
)
