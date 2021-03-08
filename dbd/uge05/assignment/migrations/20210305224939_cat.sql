-- Add migration script here
CREATE TABLE IF NOT EXISTS cat(
    fur_color CHARACTER VARYING(255)
) INHERITS (pet);

