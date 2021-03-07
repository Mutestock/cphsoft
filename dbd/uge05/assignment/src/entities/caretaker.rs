use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};


struct Caretaker {
    id: i32,
    name: String,
    phone: String,
    street: String,
}

#[async_trait]
impl CRUD<Caretaker> for Caretaker {
    async fn create(pool: &PgPool, entity: Caretaker) -> anyhow::Result<()> {
        todo!()
    }

    async fn read(&self) -> anyhow::Result<Caretaker> {
        todo!()
    }

    async fn update(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn list(&self) -> anyhow::Result<Vec<Caretaker>> {
        todo!()
    }
}