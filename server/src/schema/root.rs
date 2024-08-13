use anyhow::Result;
use juniper::{graphql_object, EmptySubscription, RootNode};

use super::{
    Availability, AvailabilityInput, Category, Cell, Compliance, Feature, FeatureInput,
    Jurisdiction, JurisdictionInput, Lifecycle, Matrix, Product, ProductInput,
};
use crate::database::Pool;

pub struct Context {
    pub db_pool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all categories")]
    async fn categories(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Category>> {
        Category::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get the matrix")]
    async fn matrix(
        _page_size: Option<i32>,
        _page: Option<i32>,
        _context: &Context,
    ) -> Result<Matrix> {
        Ok(Matrix {})
    }

    #[graphql(description = "Get a single category by id or name")]
    async fn category(
        id: Option<String>,
        name: Option<String>,
        context: &Context,
    ) -> Result<Category> {
        Category::fetch_one(id, name, &context.db_pool).await
    }

    #[graphql(description = "List all products")]
    async fn products(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Product>> {
        Product::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get a single product by id or name")]
    async fn product(
        name: Option<String>,
        id: Option<String>,
        context: &Context,
    ) -> Result<Product> {
        Product::fetch_one(id, name, &context.db_pool).await
    }

    #[graphql(description = "Listall features")]
    async fn features(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Feature>> {
        Feature::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get a single feature by id or name")]
    async fn feature(
        id: Option<String>,
        name: Option<String>,
        context: &Context,
    ) -> Result<Feature> {
        Feature::fetch_one(id, name, &context.db_pool).await
    }

    #[graphql(description = "List all jurisdictions")]
    async fn jurisdictions(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Jurisdiction>> {
        Jurisdiction::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get a single jurisdiction by id or name")]
    async fn jurisdiction(
        id: Option<String>,
        name: Option<String>,
        context: &Context,
    ) -> Result<Jurisdiction> {
        Jurisdiction::fetch_one(id, name, &context.db_pool).await
    }

    #[graphql(description = "List all compliances")]
    async fn compliances(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Compliance>> {
        Compliance::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get a single compliance by id or name")]
    async fn compliance(
        id: Option<String>,
        name: Option<String>,
        context: &Context,
    ) -> Result<Compliance> {
        Compliance::fetch_one(id, name, &context.db_pool).await
    }

    #[graphql(description = "List all lifecycles")]
    async fn lifecycles(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Lifecycle>> {
        Lifecycle::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get a single lifecycle by id or name")]
    async fn lifecycle(
        id: Option<String>,
        name: Option<String>,
        context: &Context,
    ) -> Result<Lifecycle> {
        Lifecycle::fetch_one(id, name, &context.db_pool).await
    }

    #[graphql(description = "List all cells")]
    async fn cells(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Cell>> {
        Cell::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get a single cell by id or name")]
    async fn cell(id: Option<String>, name: Option<String>, context: &Context) -> Result<Cell> {
        Cell::fetch_one(id, name, &context.db_pool).await
    }

    #[graphql(description = "List all availabilities")]
    async fn availabilities(
        page_size: Option<i32>,
        page: Option<i32>,
        context: &Context,
    ) -> Result<Vec<Availability>> {
        Availability::fetch_all(page_size, page, &context.db_pool).await
    }

    #[graphql(description = "Get a single availability by id")]
    async fn availability(id: String, context: &Context) -> Result<Availability> {
        Availability::fetch_by_id(&id, &context.db_pool).await
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_category(context: &Context, category_name: String) -> Result<Category> {
        let result = Category::create(&category_name, &context.db_pool).await?;
        tracing::trace!("Inserted {:?}", &result);
        Ok(result)
    }

    async fn create_product(context: &Context, product_input: ProductInput) -> Result<Product> {
        let result = Product::create_from_input(&product_input, &context.db_pool).await?;
        tracing::trace!("Inserted {:?}", &result);
        Ok(result)
    }

    async fn create_feature(context: &Context, feature_input: FeatureInput) -> Result<Feature> {
        let result = Feature::create_from_input(&feature_input, &context.db_pool).await?;
        tracing::trace!("Inserted {:?}", &result);
        Ok(result)
    }

    async fn create_lifecycle(context: &Context, name: String) -> Result<Lifecycle> {
        let result = Lifecycle::create(&name, &context.db_pool).await?;
        tracing::trace!("Inserted {:?}", &result);
        Ok(result)
    }

    async fn create_compliance(context: &Context, name: String) -> Result<Compliance> {
        let result = Compliance::create(&name, &context.db_pool).await?;
        tracing::trace!("Inserted {:?}", &result);
        Ok(result)
    }

    async fn create_jurisdiction(
        context: &Context,
        jurisdiction: JurisdictionInput,
    ) -> Result<Jurisdiction> {
        let result = Jurisdiction::create_from_input(&jurisdiction, &context.db_pool).await?;
        tracing::trace!("Inserted {:?}", &result);
        Ok(result)
    }

    async fn create_availability(
        context: &Context,
        availability_input: AvailabilityInput,
    ) -> Result<Availability> {
        let result = Availability::create_from_input(&availability_input, &context.db_pool).await?;
        tracing::info!("Added Availability: {:?}", &result);
        Ok(result)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
