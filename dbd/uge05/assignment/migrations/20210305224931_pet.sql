-- Add migration script here
CREATE TABLE IF NOT EXISTS pet (
    id SERIAL PRIMARY KEY NOT NULL,
    name CHARACTER VARYING(255),
    age INTEGER
)

