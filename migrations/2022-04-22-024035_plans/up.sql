-- Your SQL goes here
CREATE TABLE IF NOT EXISTS plans (
    id SERIAL PRIMARY KEY,
    hosts_id integer NOT NULL,
    name VARCHAR,
    description VARCHAR,
    price integer,
    price_unit VARCHAR DEFAULT 'N/A',
    price_timeunit VARCHAR DEFAULT 'N/A',
    price_desc VARCHAR DEFAULT 'N/A',
    -- Concurrent Build
    concurrent_build integer,
    concurrent_build_unit VARCHAR DEFAULT 'N/A',
    concurrent_build_timeunit VARCHAR DEFAULT 'N/A',
    concurrent_build_desc VARCHAR DEFAULT 'N/A',
    -- Bandwidth
    bandwidth integer,
    bandwidth_unit VARCHAR DEFAULT 'N/A',
    bandwidth_timeunit VARCHAR DEFAULT 'N/A',
    bandwidth_desc VARCHAR DEFAULT 'N/A',
    -- Build
    build integer,
    build_unit VARCHAR DEFAULT 'N/A',
    build_timeunit VARCHAR DEFAULT 'N/A',
    build_desc VARCHAR DEFAULT 'N/A',
    -- Analytic
    analytic BOOLEAN,
    analytic_price integer,
    analytic_unit VARCHAR DEFAULT 'N/A',
    analytic_timeunit VARCHAR DEFAULT 'N/A',
    analytic_desc VARCHAR DEFAULT 'N/A',
    FOREIGN KEY (hosts_id) REFERENCES hosts (id)
)
