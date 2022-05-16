-- Your SQL goes here
ALTER TABLE hosts
    ADD COLUMN always_free BOOLEAN,
    ADD COLUMN free_tier BOOLEAN,
    ADD COLUMN frontend_support BOOLEAN,
    ADD COLUMN backend_support BOOLEAN,
    ADD COLUMN database_support BOOLEAN,
    ADD COLUMN template VARCHAR