use crate::entities::entity_utils::CRUD;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Cat {
    id: i32,
    name: String,
    age: u32,
    vet_id: i32,
    fur_color: String,
}
#[derive(Deserialize)]
pub struct NewCat {
    name: String,
    age: u32,
    vet_id: i32,
    fur_color: String,
}

#[async_trait]
impl CRUD<Cat, NewCat> for Cat {
    async fn create(pool: &PgPool, entity: NewCat) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO cat(name, age, vet_id, fur_color)
            VALUES ( $1, $2, $3, $4)
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .bind(entity.fur_color)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<Cat> {
        let res = sqlx::query_as::<_, Cat>(
            r#"
            SELECT * FROM cat WHERE ID = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(res)
    }

    async fn update(pool: &PgPool, entity: NewCat, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE cat SET (name, age, vet_id, fur_color) = ( $1, $2, $3, $4 )
            WHERE id = $5
            "#,
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .bind(entity.fur_color)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM cat WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn list(pool: &PgPool) -> anyhow::Result<Vec<Cat>> {
        let res = sqlx::query_as::<_, Cat>(
            r#"
            SELECT * FROM cat
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(res)
    }
}

impl NewCat {
    pub fn new(name: String, age: u32, vet_id: i32, fur_color: String) -> Self {
        Self {
            name: name,
            age: age,
            vet_id: vet_id,
            fur_color: fur_color,
        }
    }
}
impl Cat {
    async fn read_view(pool: &PgPool, id: i32) -> anyhow::Result<Cat> {
        let res = sqlx::query_as::<_, Cat>(
            r#"
            SELECT * FROM cat_vista WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(res)
    }

    async fn update_indirectly(pool: &PgPool, entity: NewCat, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            CALL update_cat($1, $2, $3, $4, $5)
            "#
        )
        .bind(entity.name)
        .bind(entity.age)
        .bind(entity.vet_id)
        .bind(entity.fur_color)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }


    async fn list_view(pool: &PgPool) -> anyhow::Result<Vec<Cat>>{
        let res = sqlx::query_as::<_, Cat>(
            r#"
            SELECT * FROM cat_vista
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
}
