-- Add migration script here
CREATE TABLE IF NOT EXISTS dog (
    bark_pitch FLOAT
) INHERITS (pet);


DROP VIEW IF EXISTS dog_vista;
DO $$
BEGIN
    CREATE VIEW dog_vista AS
        SELECT *
        FROM dog;
    EXCEPTION WHEN DUPLICATE_OBJECT THEN
    RAISE NOTICE 'dog_vista already exists - skipping...';
END
$$;


CREATE OR replace PROCEDURE update_dog(
   in_name VARCHAR(255),
   in_age INT, 
   in_vet_id INT,
   in_bark_pitch FLOAT,
   in_id INT
)
LANGUAGE plpgsql    
AS $$
BEGIN
    -- subtracting the amount from the sender's account 
        UPDATE dog SET (name, age, vet_id, bark_pitch) = (in_name, in_age, in_vet_id, in_FUR_COLOR)
        WHERE id = in_id;
    COMMIT;
END;$$


