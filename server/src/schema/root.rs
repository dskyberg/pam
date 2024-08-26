use anyhow::Result;
use juniper::{graphql_object, EmptySubscription, RootNode};

use super::{
    Availability, AvailabilityInput, AvailabilityUpdateInput, Category, Cell, CellInput, Comment,
    CommentInput, Compliance, Feature, FeatureInput, Jurisdiction, JurisdictionInput, Lifecycle,
    Matrix, Product, ProductInput,
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

    #[graphql(description = "Get a single availability by item and jurisdiction")]
    async fn availability_for(
        item_id: String,
        jurisdiction_id: String,
        context: &Context,
    ) -> Result<Option<Availability>> {
        Availability::fetch_by_item_jurisdiction(&item_id, &jurisdiction_id, &context.db_pool).await
    }
    #[graphql(description = "Get comments for and availability")]
    async fn comments(item_id: String, context: &Context) -> Result<Vec<Comment>> {
        Comment::fetch_by_item_id(&item_id, &context.db_pool).await
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Add a new Category")]
    async fn create_category(context: &Context, name: String) -> Result<Category> {
        Category::create(&name, &context.db_pool).await
    }

    #[graphql(description = "Add a new Product from ProductInput")]
    async fn create_product(context: &Context, input: ProductInput) -> Result<Product> {
        Product::create_from_input(&input, &context.db_pool).await
    }

    #[graphql(description = "Add a new FGeature from FeatureInput")]
    async fn create_feature(context: &Context, input: FeatureInput) -> Result<Feature> {
        Feature::create_from_input(&input, &context.db_pool).await
    }

    #[graphql(description = "Add a new Lifecycle")]
    async fn create_lifecycle(
        context: &Context,
        name: String,
        description: String,
    ) -> Result<Lifecycle> {
        Lifecycle::create(&name, &description, &context.db_pool).await
    }

    #[graphql(description = "Add a new Compliance")]
    async fn create_compliance(context: &Context, name: String) -> Result<Compliance> {
        Compliance::create(&name, &context.db_pool).await
    }

    #[graphql(description = "Add a new Justification")]
    async fn create_jurisdiction(
        context: &Context,
        input: JurisdictionInput,
    ) -> Result<Jurisdiction> {
        Jurisdiction::create_from_input(&input, &context.db_pool).await
    }

    #[graphql(description = "Add a new Cell")]
    async fn create_cell(context: &Context, input: CellInput) -> Result<Cell> {
        Cell::create_from_input(&input, &context.db_pool).await
    }

    #[graphql(description = "Add an Availability from AvailibilityInput")]
    async fn create_availability(
        context: &Context,
        input: AvailabilityInput,
    ) -> Result<Availability> {
        Availability::create_from_input(&input, &context.db_pool).await
    }

    #[graphql(description = "Update an Availability from AvailabilityUpdateInput")]
    async fn update_availability(
        context: &Context,
        input: AvailabilityUpdateInput,
    ) -> Result<Availability> {
        Availability::update_from_input(input, &context.db_pool).await
    }

    #[graphql(description = "Add a Comment from CommentInput")]
    async fn create_comment(context: &Context, input: CommentInput) -> Result<Comment> {
        Comment::create(
            &input.item_id,
            &input.text,
            input.created_by.as_deref(),
            &context.db_pool,
        )
        .await
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
