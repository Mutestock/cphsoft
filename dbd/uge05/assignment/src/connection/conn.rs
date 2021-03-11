use dotenv::dotenv;
use redis::{self, Commands, RedisError};
use sqlx::postgres::PgPool;
use std::env;

lazy_static! {
    static ref PRODUCTION_MODE: bool = {
        let res = match env::var_os("PRODUCTION") {
            Some(_) => true,
            None => false,
        };
        res
    };
}

const DATABASE_URL: &str = "DATABASE_URL";
const REDIS_DATABASE_URL: &str = "REDIS_DATABASE_URL";
const RESTRICTED_DATABASE_URL: &str = "RESTRICTED_DATABASE_URL";

pub async fn get_pool() -> anyhow::Result<PgPool> {
    if *PRODUCTION_MODE {
        dotenv::from_filename(".env.prod").ok();
    } else {
        dotenv().ok();
    }

    let err_str = format!(
        "Environment variable: {} not found. Pool creation failed",
        DATABASE_URL
    );

    let mut conn_str: String = dotenv::var(DATABASE_URL).expect(&err_str);

    let restricted_user: bool = get_redis_conn()?.get("restricted")?;

    if restricted_user {
        conn_str = dotenv::var(RESTRICTED_DATABASE_URL).expect(&err_str);
    }

    // Use this instead if you're storing your environment variables on your server.
    // let conn_str = conn_str_from_server_environment_variables()
    //     .expect(&err_str);

    let pool = PgPool::connect(&conn_str).await?;

    Ok(pool)
}

// Queries from file: https://docs.rs/sqlx/0.5.1/sqlx/macro.query_file.html

#[allow(dead_code)]
fn conn_str_from_server_environment_variables() -> anyhow::Result<String> {
    let conn_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("POSTGRES_USER")?,
        env::var("POSTGRES_PASSWORD")?,
        env::var("POSTGRES_CONTAINER_NAME")?,
        env::var("POSTGRES_PORT")?,
        env::var("POSTGRES_DB")?
    );
    Ok(conn_str)
}

pub fn get_redis_conn() -> redis::RedisResult<redis::Connection> {
    dotenv().ok();
    let err_str = format!(
        "Environment variable: {} not found. Redis connection failed",
        REDIS_DATABASE_URL
    );
    let conn_str: String = dotenv::var(REDIS_DATABASE_URL).expect(&err_str);
    let client = redis::Client::open(conn_str)?;
    let conn = client.get_connection()?;

    Ok(conn)
}
