use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    password: String
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    username: String,
    password: String
}




#[async_trait]
impl CRUD<User, NewUser> for User {
    async fn create(pool: &PgPool, user: NewUser) -> anyhow::Result<()>{
        sqlx::query!(
            r#"
            INSERT INTO users(username, password) 
            VALUES ( $1, $2 )
            "#,
            user.username, user.password
        )
        .execute(pool)
        .await?;

        Ok(())
    }
    async fn read(&self) -> anyhow::Result<User>{
        unimplemented!()
    }
    async fn update(&self) -> anyhow::Result<()>{
        unimplemented!()
    }
    async fn delete(&self) -> anyhow::Result<()>{
        unimplemented!()
    }
    async fn list(&self) -> anyhow::Result<Vec<User>>{
        unimplemented!()
    }
}
