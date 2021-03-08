use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};


pub struct City {
    id: i32,
    zip_code: String,
    name: String,
}

pub struct NewCity {
    zip_code: String,
    name: String
}



#[async_trait]
impl CRUD<City, NewCity> for City {
    async fn create(pool: &sqlx::PgPool, entity: NewCity) -> anyhow::Result<()> {
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
