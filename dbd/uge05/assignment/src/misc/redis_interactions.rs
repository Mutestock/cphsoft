use crate::connection::conn::get_redis_conn;
use redis::{self, Commands, RedisError};

pub fn create_restricted_user_key_pair() -> redis::RedisResult<()> {
    let user_is_restricted: Result<bool, RedisError> = get_redis_conn()?.get("restricted");

    match user_is_restricted {
        Ok(_) => (),
        Err(_) => {
            get_redis_conn()?.set("restricted", false)?;
        }
    }
    Ok(())
}

pub fn swap_restricted_user() -> redis::RedisResult<()> {
    let restricted: bool = get_redis_conn()?.get("restricted")?;

    get_redis_conn()?.set("restricted", !restricted)?;

    Ok(())
}
