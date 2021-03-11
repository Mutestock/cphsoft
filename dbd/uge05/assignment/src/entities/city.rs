use crate::entities::entity_utils::CRUD;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct City {
    id: i32,
    zip_code: String,
    city_name: String,
}
#[derive(Deserialize)]
pub struct NewCity {
    zip_code: String,
    city_name: String,
}

impl NewCity {
    pub fn new(zip_code: String, city_name: String) -> Self {
        Self {
            zip_code: zip_code,
            city_name: city_name,
        }
    }
}

#[async_trait]
impl CRUD<City, NewCity> for City {
    async fn create(pool: &PgPool, entity: NewCity) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO city(zip_code, city_name) 
            VALUES ( $1, $2 ) ON CONFLICT DO NOTHING
            "#,
        )
        .bind(entity.zip_code)
        .bind(entity.city_name)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn read(pool: &PgPool, id: i32) -> anyhow::Result<City> {
        let res = sqlx::query_as::<_, City>(
            r#"
            SELECT * FROM city WHERE ID = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(res)
    }

    async fn update(pool: &PgPool, entity: NewCity, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE city SET (zip_code, city_name) = ( $1, $2 )
            WHERE id = $3
            "#,
        )
        .bind(entity.zip_code)
        .bind(entity.city_name)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM city WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn list(pool: &PgPool) -> anyhow::Result<Vec<City>> {
        let res = sqlx::query_as::<_, City>(
            r#"
            SELECT * FROM city
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(res)
    }
}
