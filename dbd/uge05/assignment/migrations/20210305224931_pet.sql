-- Add migration script here
CREATE TABLE IF NOT EXISTS pet (
    id SERIAL PRIMARY KEY NOT NULL,
    name CHARACTER VARYING(255) NOT NULL,
    age INTEGER NOT NULL,
    vet_id INTEGER NOT NULL REFERENCES pet(id)
);
