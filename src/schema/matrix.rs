use anyhow::Result;
use juniper::graphql_object;
use sqlx::FromRow;

use super::{root::Context, Category, Compliance, Jurisdiction, Lifecycle};

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Matrix {}

#[graphql_object(Context = Context)]
impl Matrix {
    #[graphql(description = "The Matrix with Categories, Products and Features")]
    pub async fn categories(
        &self,
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Category>> {
        Category::fetch_all(page_size, page, &context.db_pool).await
    }
    #[graphql(description = "List of Matrix Compliance")]
    pub async fn compliances(
        &self,
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Compliance>> {
        Compliance::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "List of Matrix Lifecycles")]
    pub async fn lifecycles(
        &self,
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Lifecycle>> {
        Lifecycle::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "List of Matrix Jurisdictions")]
    pub async fn jurisdictions(
        &self,
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Jurisdiction>> {
        Jurisdiction::fetch_all(page_size, page, &context.db_pool).await
    }
}
