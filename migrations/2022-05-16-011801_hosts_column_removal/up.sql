-- Your SQL goes here
ALTER TABLE hosts
    DROP COLUMN always_free,
    DROP COLUMN free_tier,
    DROP COLUMN frontend_support,
    DROP COLUMN backend_support,
    DROP COLUMN database_support,
    DROP COLUMN product_based,
    DROP COLUMN plan_based