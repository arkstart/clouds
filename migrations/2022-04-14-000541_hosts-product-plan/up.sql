-- Your SQL goes here
ALTER TABLE hosts
  ADD COLUMN product_based BOOLEAN,
  ADD COLUMN plan_based BOOLEAN
