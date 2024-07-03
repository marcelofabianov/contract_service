-- Add down migration script here
DROP INDEX IF EXISTS idx_customers_trace_id;

DROP INDEX IF EXISTS idx_customers_public_id;

DROP INDEX IF EXISTS idx_customers_deleted_at;

DROP INDEX IF EXISTS idx_customers_document;

DROP TABLE IF EXISTS customers;
