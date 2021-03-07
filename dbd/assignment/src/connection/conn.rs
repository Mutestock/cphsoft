use sqlx::postgres::PgPool;
use std::env;


pub async fn get_pool() -> anyhow::Result<PgPool>{
    
    let conn_str = format!("postgres://{}:{}@{}:{}/{}",
        env::var("POSTGRES_USER")?,
        env::var("POSTGRES_PASSWORD")?,
        env::var("POSTGRES_CONTAINER_NAME")?,
        env::var("POSTGRES_PORT")?,
        env::var("POSTGRES_DB")?
    );

    let pool = PgPool::connect(&conn_str)
        .await?;

    Ok(pool)
}

// Queries from file: https://docs.rs/sqlx/0.5.1/sqlx/macro.query_file.html
