-- Add migration script here
CREATE TABLE IF NOT EXISTS caretaker(
    id SERIAL PRIMARY KEY NOT NULL,
    name CHARACTER VARYING(255),
    phone CHARACTER VARYING(255),
    street CHARACTER VARYING(255),
    city_id INTEGER REFERENCES city(id)
)