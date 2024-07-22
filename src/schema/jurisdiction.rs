use juniper::{graphql_object, GraphQLInputObject};
use mysql::{params, prelude::*};

use super::{root::Context, Cell};

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Jurisdiction {
    pub id: String,
    pub name: String,
    pub title: String,
}

#[graphql_object(Context = Context)]
impl Jurisdiction {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn title(&self) -> &str {
        &self.title
    }

    fn cells(&self, context: &Context) -> Vec<Cell> {
        let mut conn = context.db_pool.get().unwrap();

        conn.exec(
            "SELECT * FROM Cell WHERE jurisdiction_id = :id",
            params! { "id" => &self.id },
        )
        .unwrap()
        .into_iter()
        .map(Cell::from_row)
        .collect()
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Jurisdiction Input")]
pub struct JurisdictionInput {
    pub name: String,
}
