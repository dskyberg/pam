use anyhow::{anyhow, Result};
use juniper::GraphQLInputObject;
use sqlx::FromRow;

use crate::{
    database::{paginate, Pool},
    schema::{root::Context, Availability, Product},
};

/// Product
#[derive(Default, Debug, FromRow)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub product_id: String,
}

#[juniper::graphql_object(Context = Context)]
impl Feature {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn product_id(&self) -> &str {
        &self.product_id
    }

    /// walk backward from Product to Category
    async fn product(&self, context: &Context) -> Result<Option<Product>> {
        let result = sqlx::query_as("SELECT * FROM Product WHERE id=$1")
            .bind(&self.product_id)
            .fetch_optional(&context.db_pool)
            .await?;

        Ok(result)
    }

    async fn availability(&self, context: &Context) -> Result<Vec<Availability>> {
        let result = sqlx::query_as("SELECT * FROM Availability WHERE Availability.item_id = $1")
            .bind(&self.id)
            .fetch_all(&context.db_pool)
            .await?;
        Ok(result)
    }
}

impl Feature {
    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as(&paginate("SELECT * FROM feature", page_size, page)?)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn fetch_one(
        id: Option<String>,
        name: Option<String>,
        pool: &Pool,
    ) -> Result<Feature> {
        let query = match (id, name) {
            (Some(id), None) => {
                sqlx::query_as("SELECT * FROM feature WHERE feature.id = $1").bind(id)
            }
            (None, Some(name)) => {
                sqlx::query_as("SELECT * FROM feature WHERE feature.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };

        Ok(query.fetch_one(pool).await?)
    }

    pub async fn create(name: &str, product_id: &str, pool: &Pool) -> Result<Feature> {
        let result =
            sqlx::query_as("INSERT INTO feature VALUES (gen_random_uuid(), $1, $2) RETURNING *")
                .bind(name)
                .bind(product_id)
                .fetch_one(pool)
                .await?;
        tracing::info!(
            schema = "Feature",
            task = "add",
            result = "success",
            name = name,
        );
        Ok(result)
    }

    pub async fn create_from_input(feature_input: &FeatureInput, pool: &Pool) -> Result<Feature> {
        let product = Product::fetch_one(None, Some(feature_input.product.clone()), pool)
            .await
            .map_err(|_| anyhow!("Product not found: {}", feature_input.product))?;
        Self::create(&feature_input.name, &product.id, pool).await
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Feature Input")]
pub struct FeatureInput {
    pub name: String,
    pub product: String,
}
