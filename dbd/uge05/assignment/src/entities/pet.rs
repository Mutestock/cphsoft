
use crate::entities::entity_utils::CRUD;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};
struct Pet{
    id: i32,
    name: String,
    age: u8,
}

#[async_trait]
impl CRUD<Pet> for Pet {
    async fn create(pool: &sqlx::PgPool, entity: Pet) -> anyhow::Result<()> {
        todo!()
    }

    async fn read(&self) -> anyhow::Result<Pet> {
        todo!()
    }

    async fn update(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn list(&self) -> anyhow::Result<Vec<Pet>> {
        todo!()
    }
}