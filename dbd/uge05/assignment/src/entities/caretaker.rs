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
    city_id: i32,
}

#[derive(Deserialize)]
pub struct NewCaretaker {
    name: String,
    phone: String,
    street: String,
    city_id: i32,
}

impl NewCaretaker {
    pub fn new(name: String, phone: String, street: String, city_id: i32) -> Self {
        Self {
            name:name,
            phone:phone,
            street:street,
            city_id:city_id,
        }
    }
}


#[async_trait]
impl CRUD<Caretaker, NewCaretaker> for Caretaker {
    async fn create(pool: &PgPool, entity: NewCaretaker) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO caretaker(name, phone, street, city_id) 
            VALUES ( $1, $2, $3, $4 ) ON CONFLICT DO NOTHING
            "#,
        )
        .bind(entity.name)
        .bind(entity.phone)
        .bind(entity.street)
        .bind(entity.city_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<Caretaker> {
        let res = sqlx::query_as::<_, Caretaker>(
            r#"
            SELECT * FROM caretaker WHERE ID = $1
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
            UPDATE caretaker SET (name, phone, street, city_id) = ( $1, $2, $3, $4 )
            WHERE id = $5
            "#,
        )
        .bind(entity.name)
        .bind(entity.phone)
        .bind(entity.street)
        .bind(entity.city_id)
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
