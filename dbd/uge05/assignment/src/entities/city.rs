use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};


struct City {
    id: i32,
    zip_code: String,
    name: String,
}

#[async_trait]
impl CRUD<City> for City {
    async fn create(pool: &sqlx::PgPool, entity: City) -> anyhow::Result<()> {
        todo!()
    }

    async fn read(&self) -> anyhow::Result<City> {
        todo!()
    }

    async fn update(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn list(&self) -> anyhow::Result<Vec<City>> {
        todo!()
    }
}
