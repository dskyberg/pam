use anyhow::{anyhow, Result};
use juniper::GraphQLInputObject;
use sqlx::FromRow;

use crate::database::{paginate, Pool};

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

    async fn jurisdiction(&self, context: &Context) -> Result<Option<Jurisdiction>> {
        Ok(sqlx::query_as("SELECT * FROM Jurisdiction WHERE id = $1")
            .bind(&self.jurisdiction_id)
            .fetch_optional(&context.db_pool)
            .await?)
    }
}

impl Cell {
    pub async fn fetch_all(
        page_size: Option<i32>,
        page: Option<i32>,
        pool: &Pool,
    ) -> Result<Vec<Self>> {
        let sql = paginate("SELECT * FROM cell", page_size, page)?;
        tracing::info!(sql);
        Ok(sqlx::query_as(&sql).fetch_all(pool).await?)
    }
    pub async fn fetch_one(id: Option<String>, name: Option<String>, pool: &Pool) -> Result<Cell> {
        let query = match (id, name) {
            (Some(id), None) => sqlx::query_as("SELECT * FROM cell WHERE cell.id = $1").bind(id),
            (None, Some(name)) => {
                sqlx::query_as("SELECT * FROM cell WHERE cell.name = $1").bind(name)
            }
            _ => return Err(anyhow!("Either id or name must be provided")),
        };

        Ok(query.fetch_one(pool).await?)
    }

    pub async fn create_from_input(input: &CellInput, pool: &Pool) -> Result<Cell> {
        let jurisdiction = Jurisdiction::fetch_one(None, Some(input.jurisdiction.clone()), pool)
            .await
            .map_err(|_| anyhow!("Jurisdiction not found: {}", input.jurisdiction))?;

        let result = sqlx::query_as(
                "INSERT INTO Cell(id, name, csp,country,region,csp_region,jurisdiction) VALUES(gen_random_uuid(),$1, $2, $3,$4,$5,$6) RETURNING *",
            )
            .bind(&input.name)
            .bind(&input.csp)
            .bind(&input.country)
            .bind(&input.region)
            .bind(&input.csp_region)
            .bind(&jurisdiction.id)
            .fetch_one(pool)
            .await?;
        Ok(result)
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
    pub jurisdiction: String,
}
