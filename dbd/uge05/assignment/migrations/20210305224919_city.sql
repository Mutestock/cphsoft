-- Add migration script here
CREATE TABLE IF NOT EXISTS city (
    id SERIAL PRIMARY KEY NOT NULL,
    zip_code CHARACTER VARYING(255),
    city_name CHARACTER VARYING(255)
)