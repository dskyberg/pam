use juniper::{graphql_object, GraphQLInputObject};
use mysql::prelude::*;

use super::root::Context;

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct StatusType {
    pub id: String,
    pub name: String,
}

#[graphql_object(Context = Context)]
impl StatusType {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "StatusType Input")]
pub struct StatusTypeInput {
    pub name: String,
}
