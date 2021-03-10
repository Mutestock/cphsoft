-- Add migration script here
CREATE TABLE IF NOT EXISTS cat(
    fur_color CHARACTER VARYING(255)
) INHERITS (pet);

DO $$
BEGIN
    CREATE VIEW cat_vista AS
        SELECT *
        FROM cat;
    EXCEPTION WHEN DUPLICATE_OBJECT THEN
    RAISE NOTICE 'cat_vista already exists - skipping...';
END
$$;


