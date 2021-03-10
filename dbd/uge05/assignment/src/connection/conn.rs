use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;


const DATABASE_URL: &str = "DATABASE_URL";

pub async fn get_pool() -> anyhow::Result<PgPool>{
    dotenv().ok();
    let err_string = format!("Environment variable: {} not found. Pool creation failed", DATABASE_URL);
    
    let conn_str:String =  dotenv::var(DATABASE_URL)
        .expect(&err_string);

    // Use this instead if you're storing your environment variables on your server.

    // let conn_str = conn_str_from_server_environment_variables()
    //     .expect("Server environment variables not found. Pool creation failed");

    


    let pool = PgPool::connect(&conn_str)
        .await?;

    Ok(pool)
}

// Queries from file: https://docs.rs/sqlx/0.5.1/sqlx/macro.query_file.html

#[allow(dead_code)]
fn conn_str_from_server_environment_variables() -> anyhow::Result<String> {
    let conn_str = format!("postgres://{}:{}@{}:{}/{}",
        env::var("POSTGRES_USER")?,
        env::var("POSTGRES_PASSWORD")?,
        env::var("POSTGRES_CONTAINER_NAME")?,
        env::var("POSTGRES_PORT")?,
        env::var("POSTGRES_DB")?
    );
    Ok(conn_str)
}
