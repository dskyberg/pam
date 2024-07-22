use juniper::{graphql_object, GraphQLInputObject};
use mysql::{params, prelude::*};

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

    fn products(&self, context: &Context) -> Vec<Product> {
        let mut conn = context.db_pool.get().unwrap();

        conn.exec(
            "SELECT * FROM Product WHERE category_id = :id",
            params! { "id" => &self.id },
        )
        .unwrap()
        .into_iter()
        .map(Product::from_row)
        .collect()
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Category Input")]
pub struct CategoryInput {
    pub name: String,
}
