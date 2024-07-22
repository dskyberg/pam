use juniper::{
    graphql_object, graphql_value, EmptySubscription, FieldError, FieldResult, RootNode,
};
use mysql::{params, prelude::*, Error as DBError, Row};

use super::{Category, CategoryInput, Cell, Jurisdiction, Product, ProductInput};
use crate::database::Pool;

pub struct Context {
    pub db_pool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all categories")]
    fn categories(context: &Context) -> FieldResult<Vec<Category>> {
        let mut conn = context.db_pool.get()?;
        Ok(conn.exec("SELECT * FROM Category", ())?)
    }

    #[graphql(description = "Get a single category by name")]
    fn category(context: &Context, name: String) -> FieldResult<Option<Category>> {
        let mut conn = context.db_pool.get()?;

        conn.exec_first::<Category, &str, _>(
            "SELECT * FROM Category WHERE name=:name",
            params! {"name" => name},
        )
        .map_err(|_e| {
            FieldError::new(
                "Category Not Found",
                graphql_value!({ "not_found": "category not found" }),
            )
        })
    }

    #[graphql(description = "List of all jurisdictions")]
    fn jurisdictions(context: &Context) -> FieldResult<Vec<Jurisdiction>> {
        let mut conn = context.db_pool.get()?;

        Ok(conn.exec("SELECT * FROM Jurisdiction", ())?)
    }

    #[graphql(description = "Get a single jurisdiction by name")]
    fn jurisdiction(context: &Context, name: String) -> FieldResult<Option<Jurisdiction>> {
        let mut conn = context.db_pool.get()?;

        Ok(conn.exec_first::<Jurisdiction, &str, _>(
            "SELECT * FROM Jurisdiction WHERE name=:name",
            params! {"name" => name},
        )?)
    }

    #[graphql(description = "List of all products")]
    fn products(context: &Context) -> FieldResult<Vec<Product>> {
        let mut conn = context.db_pool.get()?;
        Ok(conn.exec("SELECT * FROM Product", ())?)
    }

    #[graphql(description = "Get a single product by name")]
    fn product(context: &Context, name: String) -> FieldResult<Option<Product>> {
        let mut conn = context.db_pool.get()?;

        Ok(conn.exec_first::<Product, &str, _>(
            "SELECT * FROM Product WHERE name=:name",
            params! {"name" => name},
        )?)
    }

    #[graphql(description = "List of all cells")]
    fn cells(context: &Context) -> FieldResult<Vec<Cell>> {
        let mut conn = context.db_pool.get()?;
        Ok(conn.exec("SELECT * FROM Cell", ())?)
    }

    #[graphql(description = "Get a single cell by name")]
    fn cell(context: &Context, name: String) -> FieldResult<Option<Cell>> {
        let mut conn = context.db_pool.get()?;

        Ok(conn.exec_first::<Cell, &str, _>(
            "SELECT * FROM Cell WHERE name=:name",
            params! {"name" => name},
        )?)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    fn create_category(
        context: &Context,
        category: CategoryInput,
    ) -> FieldResult<Option<Category>> {
        let mut conn = context.db_pool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_string();
        conn.exec::<Category, _, _>(
            "INSERT INTO Category VALUES(:id, :name)",
            params! {
                "id" => &new_id,
                "name" => &category.name,
            },
        )?;
        log::trace!("Inserted id: {}, name: {}", &new_id, &category.name);

        let result = conn
            .exec_first::<Category, &str, _>(
                "SELECT * FROM Category WHERE name=:name",
                params! {"name" => category.name},
            )
            .map_err(|_e| {
                log::error!("Failed to fetch freshly inserted Category");
                FieldError::new(
                    "Category Not Found",
                    graphql_value!({ "not_found": "category not found" }),
                )
            })?;
        log::trace!("Fetched {:?}", &result);
        Ok(result)
    }

    fn create_product(context: &Context, product: ProductInput) -> FieldResult<Product> {
        let mut conn = context.db_pool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.exec_first(
            "INSERT INTO Product(id, name, category_id) VALUES(:id, :name, :category_id)",
            params! {
                "id" => &new_id,
                "name" => &product.name,
                "category_id" => &product.category_id
            },
        );

        match insert {
            Ok(_opt_row) => Ok(Product {
                id: new_id,
                name: product.name,
                category_id: product.category_id,
            }),
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned(),
                };
                Err(FieldError::new(
                    "Failed to create new product",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
