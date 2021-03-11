use crate::entities::entity_utils::CRUD;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Dog {
    id: i32,
    name: String,
    age: u32,
    vet_id: i32,
    bark_pitch: f32,
}

#[derive(Deserialize)]
pub struct NewDog {
    name: String,
    age: u32,
    vet_id: i32,
    bark_pitch: f32,
}

#[async_trait]
impl CRUD<Dog, NewDog> for Dog {
    async fn create(pool: &sqlx::PgPool, entity: NewDog) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO dog(name, age, vet_id, bark_pitch)
            VALUES ( $1, $2, $3, $4)
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .bind(entity.bark_pitch)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<Dog> {
        let res = sqlx::query_as::<_, Dog>(
            r#"
            SELECT * FROM dog WHERE ID = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(res)
    }

    async fn update(pool: &PgPool, entity: NewDog, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE dog SET (name, age, vet_id, bark_pitch) = ( $1, $2, $3, $4 )
            WHERE id = $5
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .bind(entity.bark_pitch)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM dog WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn list(pool: &PgPool) -> anyhow::Result<Vec<Dog>> {
        let res = sqlx::query_as::<_, Dog>(
            r#"
            SELECT * FROM dog
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(res)
    }
}

impl NewDog {
    pub fn new(name: String, age: u32, vet_id: i32, bark_pitch: f32) -> Self {
        Self {
            name: name,
            age: age,
            vet_id: vet_id,
            bark_pitch: bark_pitch,
        }
    }
}

impl Dog {
    pub async fn read_view(pool: &PgPool, id: i32) -> anyhow::Result<Dog> {
        let res = sqlx::query_as::<_, Dog>(
            r#"
            SELECT * FROM dog_vista WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(res)
    }

    pub async fn update_procedure(pool: &PgPool, entity: NewDog, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            CALL update_dog($1, $2, $3, $4, $5)
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .bind(entity.bark_pitch)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn list_view(pool: &PgPool) -> anyhow::Result<Vec<Dog>> {
        let res = sqlx::query_as::<_, Dog>(
            r#"
            SELECT * FROM dog_vista
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
}
