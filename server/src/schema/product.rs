use anyhow::{anyhow, Result};
use juniper::GraphQLInputObject;
use sqlx::FromRow;

use crate::{
    database::{paginate, Pool},
    schema::{root::Context, Category, Feature},
};

use super::Availability;

/// Product
#[derive(Default, Debug, FromRow)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category_id: String,
}

#[juniper::graphql_object(Context = Context)]
impl Product {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn category_id(&self) -> &str {
        &self.category_id
    }

    async fn features(&self, context: &Context) -> Result<Vec<Feature>> {
        let features = sqlx::query_as("SELECT * FROM feature WHERE product_id = $1")
            .bind(&self.id)
            .fetch_all(&context.db_pool)
            .await?;

        Ok(features)
    }

    /// walk backward from Product to Category
    async fn category(&self, context: &Context) -> Result<Option<Category>> {
        let result = sqlx::query_as("SELECT * FROM Category WHERE id=$1")
            .bind(&self.category_id)
            .fetch_optional(&context.db_pool)
            .await?;
        Ok(result)
    }

    async fn availability(&self, context: &Context) -> Result<Vec<Availability>> {
        let availabilities =
            sqlx::query_as("SELECT * FROM Availability WHERE Availability.item_id = $1")
                .bind(&self.id)
                .fetch_all(&context.db_pool)
                .await?;

        Ok(availabilities)
    }
}

impl Product {
    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as(&paginate("SELECT * FROM product", page_size, page)?)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn fetch_one(
        id: Option<String>,
        name: Option<String>,
        pool: &Pool,
    ) -> Result<Product> {
        let query = match (id, name) {
            (Some(id), None) => {
                sqlx::query_as("SELECT * FROM product WHERE product.id = $1").bind(id)
            }
            (None, Some(name)) => {
                sqlx::query_as("SELECT * FROM product WHERE product.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };

        Ok(query.fetch_one(pool).await?)
    }

    pub async fn create(name: &str, category_id: &str, pool: &Pool) -> Result<Product> {
        let result = sqlx::query_as(
            "INSERT INTO Product(id, name, category_id) VALUES(gen_random_uuid(),$1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(category_id)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn create_from_input(input: &ProductInput, pool: &Pool) -> Result<Product> {
        let category = Category::fetch_one(None, Some(input.category.clone()), pool)
            .await
            .map_err(|_| anyhow!("Category not found: {}", input.category))?;
        Self::create(&input.name, &category.id, pool).await
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    pub name: String,
    pub category: String,
}
