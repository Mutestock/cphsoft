
use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};

pub struct Vet {
    id: i32,
    name: String,
    phone: String,
    street: String,
}

pub struct NewVet {
    name: String,
    phone: String,
    street: String,
}


#[async_trait]
impl CRUD<Vet, NewVet> for Vet {
    async fn create(pool: &sqlx::PgPool, entity: NewVet) -> anyhow::Result<()> {
        todo!()
    }

    async fn read(&self) -> anyhow::Result<Vet> {
        todo!()
    }

    async fn update(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn list(&self) -> anyhow::Result<Vec<Vet>> {
        todo!()
    }
}