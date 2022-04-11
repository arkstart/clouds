-- Your SQL goes here
ALTER TABLE hosts
  ADD COLUMN always_free BOOLEAN,
  ADD COLUMN free_tier BOOLEAN,
  ADD COLUMN frontend_support BOOLEAN DEFAULT true,
  ADD COLUMN backend_support BOOLEAN,
  ADD COLUMN database_support BOOLEAN,
  ALTER COLUMN description TYPE VARCHAR,
  ALTER COLUMN url TYPE VARCHAR
  