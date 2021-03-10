use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Cat {
    id: i32,
    name: String,
    age: u8,
    fur_color: String,
}
#[derive(Deserialize)]
pub struct NewCat{
    name: String,
    age: u8,
    fur_color: String,
}

#[async_trait]
impl CRUD<Cat, NewCat> for Cat {
    async fn create(pool: &PgPool, entity: NewCat) -> anyhow::Result<()> {
        todo!()
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<Cat> {
        todo!()
    }

    async fn update(pool: &PgPool, entity: U, id: i32) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        todo!()
    }

    async fn list(pool: &PgPool) -> anyhow::Result<Vec<Cat>> {
        todo!()
    }
}
