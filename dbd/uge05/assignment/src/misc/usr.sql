/* 

*/
DO $$
BEGIN
    CREATE ROLE restricted_user WITH PASSWORD 'restricted_user_password' LOGIN;
    GRANT ALL PRIVILEGES TO restricted_user;
    REVOKE ALL PRIVILEGES ON dogs FROM restricted_user;
    REVOKE ALL PRIVILEGES ON cats FROM restricted_user;
    EXCEPTION WHEN DUPLICATE_OBJECT THEN
    RAISE NOTICE 'restricted_user already exists - skipping...';
END
$$;
