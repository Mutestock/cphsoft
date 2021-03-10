use async_trait::async_trait;
use sqlx::postgres::PgPool;

#[async_trait]
pub trait CRUD<T, U>{
    async fn create(pool: &PgPool, entity: U) -> anyhow::Result<()>;
    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<T>;
    async fn update(pool: &PgPool, entity: U, id: i32) -> anyhow::Result<()>;
    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()>;
    async fn list(pool: &PgPool) -> anyhow::Result<Vec<T>>;
}