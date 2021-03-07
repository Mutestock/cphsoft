-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY NOT NULL,
    username CHARACTER VARYING(255),
    password CHARACTER VARYING(255)
);