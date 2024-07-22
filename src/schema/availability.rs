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
    #[graphql(description = "The id of the associated product or feature")]
    fn item_id(&self) -> &str {
        &self.item_id
    }
    #[graphql(
        description = "The id of the associated lifecycle stage.  Use Availability.stage for the actual."
    )]
    fn stage_id(&self) -> &str {
        &self.stage_id
    }
    #[graphql(
        description = "The id of the associated status. Use Availability.status for the actual."
    )]
    fn status_id(&self) -> &str {
        &self.status_id
    }
    #[graphql(
        description = "The id of the associated jurisdiction. Use Availability.jurisdiction for the actual."
    )]
    fn jurisdiction_id(&self) -> &str {
        &self.jurisdiction_id
    }

    #[graphql(description = "The data last updated")]
    fn last_updated(&self) -> String {
        format!("{}", self.last_updated)
    }

    #[graphql(description = "The linked status")]
    fn status(&self, context: &Context) -> FieldResult<Option<StatusType>> {
        let mut conn = context.db_pool.get().unwrap();

        Ok(conn.exec_first::<StatusType, &str, _>(
            "SELECT * FROM StatusType WHERE id = :id",
            params! { "id" => &self.status_id },
        )?)
    }

    #[graphql(description = "The linked Lifecycle")]
    fn stage(&self, context: &Context) -> FieldResult<Option<LifecycleStage>> {
        let mut conn = context.db_pool.get().unwrap();

        Ok(conn.exec_first::<LifecycleStage, &str, _>(
            "SELECT * FROM LifecycleStage WHERE id = :id",
            params! { "id" => &self.stage_id },
        )?)
    }

    #[graphql(description = "The linked Jurisdiction")]
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

    #[graphql(description = "Summary of availability attributes as a single string")]
    fn summary(&self, context: &Context) -> FieldResult<String> {
        let mut conn = context.db_pool.get().unwrap();

        let result = conn.exec_first::<AvailabilitySummary, &str, _>(
            r#"SELECT
                Jurisdiction.name as jurisdiction,
                StatusType.name as status,
                LifecycleStage.name as stage,
                Availability.comment,
                Availability.last_updated
            FROM Availability
            LEFT JOIN Jurisdiction ON Jurisdiction.id = Availability.jurisdiction_id
            LEFT JOIN StatusType on StatusType.id = Availability.status_id
            LEFT JOIN LifecycleStage on LifecycleStage.id = Availability.stage_id
            WHERE Availability.id = :id"#,
            params! { "id" => &self.id },
        )?;
        match result {
            Some(summary) => Ok(format!(
                "{}, {}, {}, {}, {}",
                summary.jurisdiction,
                summary.stage,
                summary.status,
                summary.comment.unwrap_or("".to_string()),
                summary.last_updated
            )),
            None => Ok("Not Found".to_owned()),
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Availability Input")]
pub struct AvailabilityInput {
    pub item_name: String,
    pub jurisdiction: String,
    pub stage: String,
    pub status: String,
    pub comment: Option<String>,
}

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct AvailabilitySummary {
    pub jurisdiction: String,
    pub status: String,
    pub stage: String,
    pub comment: Option<String>,
    pub last_updated: NaiveDateTime,
}
