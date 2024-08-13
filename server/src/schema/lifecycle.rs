use anyhow::{anyhow, Result};
use juniper::graphql_object;
use sqlx::FromRow;

use crate::database::{paginate, Pool};

use super::root::Context;

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Lifecycle {
    pub id: String,
    pub name: String,
}

#[graphql_object(Context = Context)]
impl Lifecycle {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl Lifecycle {
    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Lifecycle>> {
        Ok(
            sqlx::query_as(&paginate("SELECT * FROM lifecycle", page_size, page)?)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn fetch_one(
        id: Option<String>,
        name: Option<String>,
        pool: &Pool,
    ) -> Result<Lifecycle> {
        let query = match (id, name) {
            (Some(id), None) => {
                sqlx::query_as("SELECT * FROM lifecycle WHERE ifecycle.id = $1").bind(id)
            }
            (None, Some(name)) => {
                sqlx::query_as("SELECT * FROM ifecycle WHERE lifecycle.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };

        Ok(query.fetch_one(pool).await?)
    }

    pub async fn create(name: &str, pool: &Pool) -> Result<Lifecycle> {
        Ok(
            sqlx::query_as("INSERT INTO Lifecycle VALUES (gen_random_uuid(), $1) RETURNING *")
                .bind(name)
                .fetch_one(pool)
                .await?,
        )
    }
}
