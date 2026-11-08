-- Add migration script here
CREATE TABLE IF NOT EXISTS apps (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    app TEXT NOT NULL,
    secret TEXT NOT NULL
);
