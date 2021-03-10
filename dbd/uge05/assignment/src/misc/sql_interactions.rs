use crate::connection::conn::{get_pool};
use crate::misc::redis_interactions::create_restricted_user_key_pair;

// This script swaps the user to a user with restricted privileges
pub async fn execute_restricted_user_creation() -> anyhow::Result<()> {
    let pool = get_pool().await?;
    create_restricted_user_key_pair()
        .expect("Could not create restricted user key value pair");
    sqlx::query_file!("src/misc/usr.sql").execute(&pool).await?;

    Ok(())
}

// This function utilizes an sql script to populate the database
pub async fn execute_database_population_script() -> anyhow::Result<()> {
    let pool = get_pool().await?;
    sqlx::query_file!("src/misc/pop.sql").execute(&pool).await?;

    Ok(())
}
