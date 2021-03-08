-- Add migration script here
CREATE TABLE IF NOT EXISTS dog (
    bark_pitch FLOAT
) INHERITS (pet);