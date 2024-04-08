-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    user varchar NOT NULL,
    quote TEXT NOT NULL,
    inserted_at TIMESTAMPZ NOT NULL,
    updated_at TIMESTAMPZ NOT NULL,
    UNIQUE (user, quote)
);