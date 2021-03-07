use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};


pub struct Cat {
    id: i32,
    name: String,
    age: u8,
    fur_color: String,
}


#[async_trait]
impl CRUD<Cat> for Cat {
    async fn create(pool: &PgPool, entity: Cat) -> anyhow::Result<()> {
        todo!()
    }

    async fn read(&self) -> anyhow::Result<Cat> {
        todo!()
    }

    async fn update(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn list(&self) -> anyhow::Result<Vec<Cat>> {
        todo!()
    }
}
