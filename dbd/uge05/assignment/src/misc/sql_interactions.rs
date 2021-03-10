use crate::connection::conn::get_pool;
use sqlx::postgres::PgPool;

// This script swaps the user to a user with restricted privileges
pub async fn user_swap(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query_file!("src/misc/usr.sql").execute(pool).await?;

    Ok(())
}

// This function utilizes an sql script to populate the database
pub async fn execute_database_population_script() -> anyhow::Result<()> {
    let pool = get_pool().await?;
    sqlx::query_file!("src/misc/pop.sql").execute(&pool).await?;

    Ok(())
}
