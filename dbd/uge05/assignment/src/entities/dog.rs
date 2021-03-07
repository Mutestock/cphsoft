
use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};

struct Dog {
    id: i32,
    name: String,
    age: u8,
    bark_pitch: f32,
}

#[async_trait]
impl CRUD<Dog> for Dog {
    async fn create(pool: &sqlx::PgPool, entity: Dog) -> anyhow::Result<()> {
        todo!()
    }

    async fn read(&self) -> anyhow::Result<Dog> {
        todo!()
    }

    async fn update(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn list(&self) -> anyhow::Result<Vec<Dog>> {
        todo!()
    }
}
