
use async_trait::async_trait;
use crate::entities::entity_utils::CRUD;
use sqlx::postgres::PgPool;
use serde_derive::{Deserialize, Serialize};



#[derive(Deserialize,Serialize,sqlx::FromRow)]
pub struct Vet {
    id: i32,
    name: String,
    phone: String,
    street: String,
}

#[derive(Deserialize)]
pub struct NewVet {
    name: String,
    phone: String,
    street: String,
}


#[async_trait]
impl CRUD<Vet, NewVet> for Vet {
    async fn create(pool: &PgPool, entity: NewVet) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vet(name, phone, street) 
            VALUES ( $1, $2 , $3)
            "#,
        )
        .bind(entity.name)
        .bind(entity.phone)
        .bind(entity.street)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<Vet> {
        let res = sqlx::query_as::<_, Vet>(
            r#"
            SELECT * FROM vet WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(res)
    }

    async fn update(pool: &PgPool, entity: NewVet, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE vet SET (name, phone, street) = ( $1, $2, $3 )
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
            DELETE FROM vet WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn list(pool: &PgPool) -> anyhow::Result<Vec<Vet>> {
        let res = sqlx::query_as::<_, Vet>(
            r#"
            SELECT * FROM vet
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(res)
    }
}