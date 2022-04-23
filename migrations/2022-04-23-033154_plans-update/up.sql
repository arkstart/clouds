-- Your SQL goes here
ALTER TABLE plans
  ALTER COLUMN name TYPE VARCHAR,
  ALTER COLUMN description TYPE VARCHAR,
  ALTER COLUMN price TYPE integer,
  ALTER COLUMN price_unit TYPE  VARCHAR,
  ALTER COLUMN price_timeunit TYPE VARCHAR,
  ALTER COLUMN price_desc TYPE VARCHAR,
  -- Concurrent Build
  ALTER COLUMN concurrent_build TYPE integer,
  ALTER COLUMN concurrent_build_unit TYPE VARCHAR,
  ALTER COLUMN concurrent_build_timeunit TYPE VARCHAR,
  ALTER COLUMN concurrent_build_desc TYPE VARCHAR,
  -- Bandwidth
  ALTER COLUMN bandwidth TYPE integer,
  ALTER COLUMN bandwidth_unit TYPE VARCHAR,
  ALTER COLUMN bandwidth_timeunit TYPE VARCHAR,
  ALTER COLUMN bandwidth_desc TYPE VARCHAR,
  -- Build
  ALTER COLUMN build TYPE integer,
  ALTER COLUMN build_unit TYPE VARCHAR,
  ALTER COLUMN build_timeunit TYPE VARCHAR,
  ALTER COLUMN build_desc TYPE VARCHAR,
  -- Analytic
  ALTER COLUMN analytic TYPE BOOLEAN,
  ALTER COLUMN analytic_price TYPE  integer,
  ALTER COLUMN analytic_unit TYPE VARCHAR,
  ALTER COLUMN analytic_timeunit TYPE VARCHAR,
  ALTER COLUMN analytic_desc TYPE VARCHAR,
  ADD COLUMN plan_url VARCHAR
