/* 

*/
DO $$
BEGIN
    CREATE ROLE restricted_user WITH PASSWORD 'restricted_user_password' LOGIN;
    GRANT ALL PRIVILEGES ON pet TO restricted_user;
    GRANT ALL PRIVILEGES ON caretaker TO restricted_user;
    GRANT ALL PRIVILEGES ON vet TO restricted_user;
    GRANT ALL PRIVILEGES ON city TO restricted_user;
    REVOKE ALL PRIVILEGES ON dog FROM restricted_user;
    REVOKE ALL PRIVILEGES ON cat FROM restricted_user;
    GRANT ALL PRIVILEGES ON cat_vista TO restricted_user;
    GRANT ALL PRIVILEGES ON dog_vista TO restricted_user;
    GRANT ALL PRIVILEGES ON PROCEDURE update_cat to restricted_user;
    GRANT ALL PRIVILEGES ON PROCEDURE update_dog to restricted_user;
    EXCEPTION WHEN DUPLICATE_OBJECT THEN
    RAISE NOTICE 'restricted_user already exists - skipping...';
END
$$;
