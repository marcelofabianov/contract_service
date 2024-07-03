-- Add up migration script here
CREATE TABLE customers (
    transaction_id UUID NOT NULL UNIQUE,
    id SERIAL PRIMARY KEY,
    public_id UUID NOT NULL UNIQUE,
    document VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    disabled_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_customers_transaction_id ON customers (transaction_id);

CREATE INDEX idx_customers_public_id ON customers (public_id);

CREATE INDEX idx_customers_deleted_at ON customers (deleted_at);

CREATE INDEX idx_customers_document ON customers (document);
