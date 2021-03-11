-- Add migration script here
CREATE TABLE IF NOT EXISTS cat(
    fur_color CHARACTER VARYING(255)
) INHERITS (pet);


DROP VIEW IF EXISTS cat_vista;
DO $$
BEGIN
    CREATE VIEW cat_vista AS
        SELECT *
        FROM cat;
    EXCEPTION WHEN DUPLICATE_OBJECT THEN
    RAISE NOTICE 'cat_vista already exists - skipping...';
END
$$;

CREATE OR replace PROCEDURE update_cat(
   in_name VARCHAR(255),
   in_age INT, 
   in_vet_id INT,
   in_FUR_COLOR VARCHAR(255),
   in_id INT
)
LANGUAGE plpgsql    
AS $$
BEGIN
    -- subtracting the amount from the sender's account 
        UPDATE cat SET (name, age, vet_id, fur_color) = (in_name, in_age, in_vet_id, in_FUR_COLOR)
        WHERE id = in_id;
    COMMIT;
END;$$


