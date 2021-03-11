use crate::entities::entity_utils::CRUD;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Pet {
    id: i32,
    name: String,
    age: i32,
    vet_id: i32,
}

#[derive(Deserialize)]
pub struct NewPet {
    name: String,
    age: i32,
    vet_id: i32,
}

impl NewPet {
    pub fn new(name: String, age: i32, vet_id: i32) -> Self {
        Self {
            name:name,
            age:age,
            vet_id:vet_id,
        }
    }
}


#[async_trait]
impl CRUD<Pet, NewPet> for Pet {
    async fn create(pool: &PgPool, entity: NewPet) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO pet(name, age, vet_id) 
            VALUES ( $1, $2, $3 ) ON CONFLICT DO NOTHING
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<Pet> {
        let res = sqlx::query_as::<_, Pet>(
            r#"
            SELECT * FROM pet WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(res)
    }

    async fn update(pool: &PgPool, entity: NewPet, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE pet SET (name, age, vet_id) = ( $1, $2, $3 )
            WHERE id = $4
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM pet WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn list(pool: &PgPool) -> anyhow::Result<Vec<Pet>> {
        let res = sqlx::query_as::<_, Pet>(
            r#"
            SELECT * FROM pet
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(res)
    }
}
