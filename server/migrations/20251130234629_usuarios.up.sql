-- Add up migration script here
-- sqlx migrate run
CREATE TABLE IF NOT EXISTS usuarios (
    id UUID PRIMARY KEY NOT NULL,
    nome TEXT NOT NULL UNIQUE,
    cor TEXT NOT NULL DEFAULT '#000000'
)