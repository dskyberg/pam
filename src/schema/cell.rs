use juniper::{FieldResult, GraphQLInputObject};
use mysql::{params, prelude::*};

use super::{root::Context, Jurisdiction};

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Cell {
    pub id: String,
    pub name: String,
    pub csp: String,
    pub country: String,
    pub region: String,
    pub csp_region: String,
    pub jurisdiction_id: String,
}

#[juniper::graphql_object(Context = Context)]
impl Cell {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn csp(&self) -> &str {
        &self.csp
    }
    fn country(&self) -> &str {
        &self.country
    }
    fn region(&self) -> &str {
        &self.region
    }
    fn csp_region(&self) -> &str {
        &self.csp_region
    }

    fn jurisdiction_id(&self) -> &str {
        &self.jurisdiction_id
    }

    fn jurisdiction(&self, context: &Context) -> FieldResult<Option<Jurisdiction>> {
        let mut conn = context.db_pool.get().unwrap();

        Ok(conn.exec_first::<Jurisdiction, &str, _>(
            "SELECT * FROM Jurisdiction WHERE id = :id",
            params! { "id" => &self.jurisdiction_id },
        )?)
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Cell Input")]
pub struct CellInput {
    pub name: String,
    pub csp: String,
    pub country: String,
    pub region: String,
    pub csp_region: String,
    pub jurisdiction_id: String,
}
