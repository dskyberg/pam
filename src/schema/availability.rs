use chrono::{self, NaiveDateTime};
use juniper::{FieldResult, GraphQLInputObject};
use mysql::{params, prelude::*};

use super::{root::Context, Jurisdiction, LifecycleStage, StatusType};

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Availability {
    pub id: String,
    /// The Product or Feature that is being made available
    pub item_id: String,
    pub stage_id: String,
    pub status_id: String,
    pub jurisdiction_id: String,
    pub comment: Option<String>,
    pub last_updated: NaiveDateTime,
}

#[juniper::graphql_object(Context = Context)]
impl Availability {
    fn id(&self) -> &str {
        &self.id
    }
    fn item_id(&self) -> &str {
        &self.item_id
    }
    fn stage_id(&self) -> &str {
        &self.stage_id
    }
    fn status_id(&self) -> &str {
        &self.status_id
    }
    fn jurisdiction_id(&self) -> &str {
        &self.jurisdiction_id
    }
    fn last_updated(&self) -> String {
        format!("{}", self.last_updated)
    }

    fn status(&self, context: &Context) -> FieldResult<Option<StatusType>> {
        let mut conn = context.db_pool.get().unwrap();

        Ok(conn.exec_first::<StatusType, &str, _>(
            "SELECT * FROM StatusType WHERE id = :id",
            params! { "id" => &self.status_id },
        )?)
    }

    fn stage(&self, context: &Context) -> FieldResult<Option<LifecycleStage>> {
        let mut conn = context.db_pool.get().unwrap();

        Ok(conn.exec_first::<LifecycleStage, &str, _>(
            "SELECT * FROM LifecycleStage WHERE id = :id",
            params! { "id" => &self.stage_id },
        )?)
    }

    fn jurisdiction(&self, context: &Context) -> Jurisdiction {
        let mut conn = context.db_pool.get().unwrap();

        let result = conn.exec_first::<Jurisdiction, &str, _>(
            "SELECT * FROM Jurisdiction WHERE Jurisdiction.id = :id",
            params! { "id" => &self.jurisdiction_id },
        );
        result.unwrap().unwrap()
    }

    fn comment(&self) -> &Option<String> {
        &self.comment
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Cell Input")]
pub struct AvailabilityInput {
    pub item_id: String,
    pub stage_id: String,
    pub status_id: String,
    pub jurisdiction_id: String,
    pub comment: Option<String>,
}
