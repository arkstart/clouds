-- Your SQL goes here
ALTER TABLE plans
    ADD COLUMN currency VARCHAR,
    ADD COLUMN discounted_price integer,
    ADD COLUMN free_domain BOOLEAN,
    ADD COLUMN domain_extension VARCHAR,
    ADD COLUMN database_benefit BOOLEAN,
    ADD COLUMN page_data VARCHAR,
    ADD COLUMN supported_languages VARCHAR
