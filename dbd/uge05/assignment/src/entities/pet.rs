
use crate::entities::entity_utils::CRUD;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Pet{
    id: i32,
    name: String,
    age: i32,
}


#[derive(Deserialize)]
pub struct NewPet {
    name: String,
    age: i32,
}



#[async_trait]
impl CRUD<Pet, NewPet> for Pet {
    async fn create(pool: &PgPool, entity: NewPet) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO pet(name, age) 
            VALUES ( $1, $2 )
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
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
            UPDATE pet SET (name, age) = ( $1, $2 )
            WHERE id = $3
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
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