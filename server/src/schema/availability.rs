use anyhow::{anyhow, Result};
use chrono::{self, NaiveDateTime};
use juniper::GraphQLInputObject;
use sqlx::FromRow;

use crate::database::{paginate, Pool};

use super::{root::Context, Comment, Compliance, Jurisdiction, Lifecycle};

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Availability {
    pub id: String,
    /// The Product or Feature that is being made available
    pub item_id: String,
    pub jurisdiction_id: String,
    pub lifecycle_id: String,
    pub compliance_id: String,
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
        description = "The id of the associated jurisdiction. Use Availability.jurisdiction for the actual."
    )]

    fn jurisdiction_id(&self) -> &str {
        &self.jurisdiction_id
    }

    #[graphql(
        description = "The id of the associated lifecycle stage.  Use Availability.lifecycle for the actual."
    )]
    fn lifecycle_id(&self) -> &str {
        &self.lifecycle_id
    }

    #[graphql(
        description = "The id of the associated compliance. Use Availability.compliance for the actual."
    )]
    fn compliance_id(&self) -> &str {
        &self.compliance_id
    }

    #[graphql(description = "The data last updated")]
    fn last_updated(&self) -> String {
        format!("{}", self.last_updated)
    }

    #[graphql(description = "The linked Jurisdiction")]
    async fn jurisdiction(&self, context: &Context) -> Result<Jurisdiction> {
        let result = sqlx::query_as("SELECT * FROM Jurisdiction WHERE Jurisdiction.id = $1")
            .bind(&self.jurisdiction_id)
            .fetch_one(&context.db_pool)
            .await?;
        Ok(result)
    }

    #[graphql(description = "The linked Lifecycle")]
    async fn lifecycle(&self, context: &Context) -> Result<Option<Lifecycle>> {
        let result = sqlx::query_as("SELECT * FROM Lifecycle WHERE id = $1")
            .bind(&self.lifecycle_id)
            .fetch_optional(&context.db_pool)
            .await?;
        Ok(result)
    }

    #[graphql(description = "The linked status")]
    async fn compliance(&self, context: &Context) -> Result<Option<Compliance>> {
        let result = sqlx::query_as("SELECT * FROM Compliance WHERE id = $1")
            .bind(&self.compliance_id)
            .fetch_optional(&context.db_pool)
            .await?;
        Ok(result)
    }

    #[graphql(description = "The linked status")]
    async fn comments(&self, context: &Context) -> Result<Vec<Comment>> {
        Comment::fetch_by_item_id(&self.id, &context.db_pool).await
    }

    #[graphql(description = "Summary of availability attributes as a single string")]
    async fn summary(&self, context: &Context) -> Result<String> {
        let result: Option<AvailabilitySummary> = sqlx::query_as(
            r#"SELECT
                Jurisdiction.name as jurisdiction,
                Lifecycle.name as lifecycle,
                Compliance.name as compliance,
                Availability.last_updated
            FROM Availability
            LEFT JOIN Jurisdiction ON Jurisdiction.id = Availability.jurisdiction_id
            LEFT JOIN Lifecycle on Lifecycle.id = Availability.lifecycle_id
            LEFT JOIN Compliance on Compliance.id = Availability.compliance_id
            WHERE Availability.id = $1"#,
        )
        .bind(&self.id)
        .fetch_optional(&context.db_pool)
        .await?;

        match result {
            Some(summary) => Ok(format!(
                "{}, {}, {}, {}",
                summary.jurisdiction, summary.lifecycle, summary.compliance, summary.last_updated
            )),
            None => Ok("Not Found".to_owned()),
        }
    }
}

impl Availability {
    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Availability>> {
        Ok(
            sqlx::query_as(&paginate("SELECT * FROM availability", page_size, page)?)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn fetch_by_id(id: &str, pool: &Pool) -> Result<Availability> {
        Ok(
            sqlx::query_as("SELECT * FROM availability WHERE availability.id = $1")
                .bind(id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn fetch_by_item_id_jurisdiction(
        id: &str,
        jurisdiction_name: &str,
        pool: &Pool,
    ) -> Result<Option<Jurisdiction>> {
        let jurisdiction = Jurisdiction::fetch_one(None, Some(jurisdiction_name.to_owned()), pool)
            .await
            .map_err(|_| anyhow::anyhow!("Jurisdiction {} does not exist", jurisdiction_name))?;

        Ok(sqlx::query_as(
            r#"SELECT * from Availability
            LEFT JOIN Jurisdiction ON Jurisdiction.id = Availability.jurisdiction_id
            WHERE Availability.item_id = $1 and Jurisdiction.name = $2;"#,
        )
        .bind(id)
        .bind(&jurisdiction.id)
        .fetch_optional(pool)
        .await?)
    }

    pub async fn create(
        item_id: &str,
        jurisdiction_id: &str,
        lifecycle_id: &str,
        compliance_id: &str,
        pool: &Pool,
    ) -> Result<Self> {
        let result = sqlx::query_as(
            r#"INSERT INTO Availability
            VALUES ( gen_random_uuid(), $1, $2, $3, $4, CURRENT_TIMESTAMP)
            RETURNING *"#,
        )
        .bind(item_id)
        .bind(jurisdiction_id)
        .bind(lifecycle_id)
        .bind(compliance_id)
        .fetch_one(pool)
        .await?;

        tracing::info!(
            schema = "Availability",
            task = "add",
            result = "success",
            item_id = item_id
        );
        Ok(result)
    }

    pub async fn create_from_input(input: &AvailabilityInput, pool: &Pool) -> Result<Self> {
        let jurisdiction = Jurisdiction::fetch_one(None, Some(input.jurisdiction.clone()), pool)
            .await
            .map_err(|_| anyhow!("Jurisdiction not found: {}", &input.jurisdiction))?;

        let lifecycle = Lifecycle::fetch_one(None, Some(input.lifecycle.clone()), pool)
            .await
            .map_err(|_| anyhow!("Lifecycle not found: {}", &input.lifecycle))?;

        let compliance = Compliance::fetch_one(None, Some(input.compliance.clone()), pool)
            .await
            .map_err(|_| anyhow!("Compliance not found: {}", &input.compliance))?;

        Self::create(
            &input.item_id,
            &jurisdiction.id,
            &lifecycle.id,
            &compliance.id,
            pool,
        )
        .await
    }

    pub async fn update(
        id: String,
        lifecycle: Option<String>,
        compliance: Option<String>,
        pool: &Pool,
    ) -> Result<Availability> {
        let mut separator = "";
        let mut sql = String::from("UPDATE availability SET ");

        if let Some(lifecycle_name) = lifecycle {
            let lifecycle = Lifecycle::fetch_one(None, Some(lifecycle_name), pool).await?;
            sql = format!("{}{}lifecycle_id = {}", sql, &separator, &lifecycle.id);
            separator = ", ";
        }

        if let Some(compliance_name) = compliance {
            let compliance = Compliance::fetch_one(None, Some(compliance_name), pool).await?;
            sql = format!("{}{}compliance_id = {}", sql, &separator, compliance.id);
            separator = ", ";
        }

        sql = format!(
            "{}{}last_updated = CURRENT_TIMESTAMP WHERE availability.id = {} RETURNING *",
            sql, &separator, id
        );

        let result = sqlx::query_as(&sql).fetch_one(pool).await?;

        tracing::info!(
            schema = "Availability",
            task = "update",
            result = "success",
            "{}",
            &sql
        );
        Ok(result)
    }

    pub async fn update_from_input(
        input: AvailabilityUpdateInput,
        pool: &Pool,
    ) -> Result<Availability> {
        Self::update(input.id, input.lifecycle, input.compliance, pool).await
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Availability Input")]
pub struct AvailabilityInput {
    pub item_id: String,
    pub jurisdiction: String,
    pub lifecycle: String,
    pub compliance: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Availability Update Input")]
pub struct AvailabilityUpdateInput {
    pub id: String,
    pub lifecycle: Option<String>,
    pub compliance: Option<String>,
}

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct AvailabilitySummary {
    pub jurisdiction: String,
    pub lifecycle: String,
    pub compliance: String,
    pub last_updated: NaiveDateTime,
}
