use juniper::{graphql_object, GraphQLInputObject};
use mysql::prelude::*;

use super::root::Context;

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct LifecycleStage {
    pub id: String,
    pub name: String,
}

#[graphql_object(Context = Context)]
impl LifecycleStage {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Lifecycle Stage Input")]
pub struct LifecycleStageInput {
    pub name: String,
}
