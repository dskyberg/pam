use juniper::GraphQLInputObject;
use mysql::{from_row, params, prelude::*, Error as DBError, Row};

use crate::schema::{root::Context, Category, Feature};

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

    fn features(&self, context: &Context) -> Vec<Feature> {
        let mut conn = context.db_pool.get().unwrap();

        conn.exec(
            "SELECT * FROM Feature WHERE product_id = :id",
            params! { "id" => &self.id },
        )
        .unwrap()
        .into_iter()
        .map(Feature::from_row)
        .collect()
    }

    /// walk backward from Product to Category
    fn category(&self, context: &Context) -> Option<Category> {
        let mut conn = context.db_pool.get().unwrap();
        let result: Result<Option<Row>, DBError> = conn.exec_first(
            "SELECT * FROM Category WHERE id=:id",
            params! {"id" => &self.category_id},
        );
        if let Err(_err) = result {
            None
        } else {
            let (id, name) = from_row(result.unwrap().unwrap());
            Some(Category { id, name })
        }
    }

    fn availability(&self, context: &Context) -> Vec<Availability> {
        let mut conn = context.db_pool.get().unwrap();

        conn.exec(
            "SELECT * FROM Availability WHERE Availability.item_id = :id",
            params! { "id" => &self.id },
        )
        .unwrap()
        .into_iter()
        .map(Availability::from_row)
        .collect()
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    pub name: String,
    pub category_id: String,
}
