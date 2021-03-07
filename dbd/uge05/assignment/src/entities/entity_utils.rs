use async_trait::async_trait;
use sqlx::postgres::PgPool;

#[async_trait]
pub trait CRUD<T>{
    async fn create(pool: &PgPool, entity: T) -> anyhow::Result<()>;
    async fn read(&self) -> anyhow::Result<T>;
    async fn update(&self) -> anyhow::Result<()>;
    async fn delete(&self) -> anyhow::Result<()>;
    async fn list(&self) -> anyhow::Result<Vec<T>>;
}