use juniper::GraphQLInputObject;
use mysql::{from_row, params, prelude::*, Error as DBError, Row};

use crate::schema::{root::Context, Product};

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
    fn product(&self, context: &Context) -> Option<Product> {
        let mut conn = context.db_pool.get().unwrap();
        let result: Result<Option<Row>, DBError> = conn.exec_first(
            "SELECT * FROM Product WHERE id=:id",
            params! {"id" => &self.product_id},
        );
        if let Err(_err) = result {
            None
        } else {
            let (id, name, category_id) = from_row(result.unwrap().unwrap());
            Some(Product {
                id,
                name,
                category_id,
            })
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Feature Input")]
pub struct FeatureInput {
    pub name: String,
    pub product_id: String,
}
