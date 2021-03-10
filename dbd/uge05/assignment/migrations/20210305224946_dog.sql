-- Add migration script here
CREATE TABLE IF NOT EXISTS dog (
    bark_pitch FLOAT
) INHERITS (pet);


DO $$
BEGIN
    CREATE VIEW dog_vista AS
        SELECT *
        FROM dog;
    EXCEPTION WHEN DUPLICATE_OBJECT THEN
    RAISE NOTICE 'dog_vista already exists - skipping...';
END
$$;

