use anyhow::{anyhow, Result};
use juniper::{graphql_object, GraphQLInputObject};
use sqlx::FromRow;

use crate::database::{paginate, Pool};

use super::{root::Context, Product};

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Category Input")]
pub struct UserInput {
    pub name: String,
}

#[graphql_object(Context = Context)]
impl Category {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }

    async fn products(&self, context: &Context) -> Result<Vec<Product>> {
        let row = sqlx::query_as("SELECT * FROM Product WHERE category_id = $1")
            .bind(&self.id)
            .fetch_all(&context.db_pool)
            .await?;
        Ok(row)
    }
}

#[derive(Debug, Clone, FromRow)]
struct Count {
    pub count: i64,
}

impl Category {
    pub async fn count(pool: &Pool) -> Result<i64> {
        let count: Count = sqlx::query_as("SELECT COUNT (id) FROM category")
            .fetch_one(pool)
            .await?;

        Ok(count.count)
    }

    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Category>> {
        Ok(
            sqlx::query_as(&paginate("SELECT * FROM category", page_size, page)?)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn fetch_one(
        id: Option<String>,
        name: Option<String>,
        pool: &Pool,
    ) -> Result<Category> {
        let query = match (id, name) {
            (Some(id), None) => {
                sqlx::query_as("SELECT * FROM category WHERE category.id = $1").bind(id)
            }
            (None, Some(name)) => {
                sqlx::query_as("SELECT * FROM category WHERE category.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };

        Ok(query.fetch_one(pool).await?)
    }

    pub async fn create(name: &str, pool: &Pool) -> Result<Category> {
        let result =
            sqlx::query_as("INSERT INTO category VALUES (gen_random_uuid(), $1) RETURNING *")
                .bind(name)
                .fetch_one(pool)
                .await?;
        tracing::info!(
            schema = "Category",
            task = "add",
            result = "success",
            name = name
        );

        Ok(result)
    }
}
