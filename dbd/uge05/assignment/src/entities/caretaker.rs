use crate::entities::entity_utils::CRUD;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Caretaker {
    id: i32,
    name: String,
    phone: String,
    street: String,
}

#[derive(Deserialize)]
pub struct NewCaretaker {
    name: String,
    phone: String,
    street: String,
}

#[async_trait]
impl CRUD<Caretaker, NewCaretaker> for Caretaker {
    async fn create(pool: &PgPool, entity: NewCaretaker) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO caretaker(name, phone, street) 
            VALUES ( $1, $2, $3)
            "#,
        )
        .bind(entity.name)
        .bind(entity.phone)
        .bind(entity.street)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<Caretaker> {
        let res = sqlx::query_as::<_, Caretaker>(
            r#"
            SELECT * FROM caretaker WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(res)
    }

    async fn update(pool: &PgPool, entity: NewCaretaker, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE caretaker SET (name, phone, street) = ( $1, $2, $3)
            WHERE id = $4
            "#,
        )
        .bind(entity.name)
        .bind(entity.phone)
        .bind(entity.street)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM caretaker WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn list(pool: &PgPool) -> anyhow::Result<Vec<Caretaker>> {
        let res = sqlx::query_as::<_, Caretaker>(
            r#"
            SELECT * FROM caretaker
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(res)
    }
}
