use anyhow::{anyhow, Result};
use juniper::graphql_object;
use sqlx::FromRow;

use crate::database::{paginate, Pool};

use super::root::Context;

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Compliance {
    pub id: String,
    pub name: String,
}

#[graphql_object(Context = Context)]
impl Compliance {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl Compliance {
    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Compliance>> {
        Ok(
            sqlx::query_as(&paginate("SELECT * FROM compliance", page_size, page)?)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn fetch_one(
        id: Option<String>,
        name: Option<String>,
        pool: &Pool,
    ) -> Result<Compliance> {
        let query = match (id, name) {
            (Some(id), None) => {
                sqlx::query_as("SELECT * FROM Compliance WHERE Compliance.id = $1").bind(id)
            }
            (None, Some(name)) => {
                sqlx::query_as("SELECT * FROM Compliance WHERE Compliance.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };

        Ok(query.fetch_one(pool).await?)
    }

    pub async fn create(name: &str, pool: &Pool) -> Result<Compliance> {
        let result =
            sqlx::query_as("INSERT INTO Compliance VALUES (gen_random_uuid(), $1) RETURNING *")
                .bind(name)
                .fetch_one(pool)
                .await?;
        tracing::info!(
            schema = "CVompliance",
            task = "add",
            result = "success",
            name = name,
        );
        Ok(result)
    }
}
