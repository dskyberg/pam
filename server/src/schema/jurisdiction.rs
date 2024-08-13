use anyhow::{anyhow, Result};
use juniper::{graphql_object, GraphQLInputObject};
use sqlx::FromRow;

use crate::database::{paginate, Pool};

use super::{root::Context, Cell};

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Jurisdiction {
    pub id: String,
    pub name: String,
    pub title: String,
}

#[graphql_object(Context = Context)]
impl Jurisdiction {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn title(&self) -> &str {
        &self.title
    }

    async fn cells(&self, context: &Context) -> Result<Vec<Cell>> {
        let result = sqlx::query_as("SELECT * FROM Cell WHERE jurisdiction_id = $1")
            .bind(&self.id)
            .fetch_all(&context.db_pool)
            .await?;
        Ok(result)
    }
}

impl Jurisdiction {
    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Jurisdiction>> {
        Ok(
            sqlx::query_as(&paginate("SELECT * FROM jurisdiction", page_size, page)?)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn fetch_one(
        id: Option<String>,
        name: Option<String>,
        pool: &Pool,
    ) -> Result<Jurisdiction> {
        let query = match (id, name) {
            (Some(id), None) => {
                sqlx::query_as("SELECT * FROM jurisdiction WHERE jurisdiction.id = $1").bind(id)
            }
            (None, Some(name)) => {
                sqlx::query_as("SELECT * FROM jurisdiction WHERE jurisdiction.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };

        Ok(query.fetch_one(pool).await?)
    }

    pub async fn create_from_input(input: &JurisdictionInput, pool: &Pool) -> Result<Jurisdiction> {
        Ok(
            sqlx::query_as("INSERT INTO feature VALUES (gen_random_uuid(), $1, $2) RETURNING *")
                .bind(&input.name)
                .bind(&input.title)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn delete(
        id: Option<String>,
        name: Option<String>,
        pool: &Pool,
    ) -> Result<Jurisdiction> {
        let query = match (id, name) {
            (Some(id), None) => {
                sqlx::query_as("DELETE FROM jurisdiction WHERE jurisdiction.id = $1").bind(id)
            }
            (None, Some(name)) => {
                sqlx::query_as("DELETE FROM jurisdiction WHERE jurisdiction.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };
        Ok(query.fetch_one(pool).await?)
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Jurisdiction Input")]
pub struct JurisdictionInput {
    pub name: String,
    pub title: String,
}
