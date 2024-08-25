use anyhow::Result;
use chrono::{self, NaiveDateTime};
use juniper::graphql_object;
use juniper::GraphQLInputObject;
use sqlx::FromRow;

use crate::database::Pool;

use super::root::Context;

#[derive(Default, Debug, PartialEq, Eq, FromRow)]
pub struct Comment {
    pub id: String,
    pub item_id: String,
    pub text: String,
    pub created: NaiveDateTime,
    pub created_by: Option<String>,
}

#[graphql_object(Context = Context)]
impl Comment {
    fn id(&self) -> &str {
        &self.id
    }

    #[graphql(description = "The id of the associated item being commented on")]
    fn item_id(&self) -> &str {
        &self.item_id
    }

    #[graphql(description = "The comment text")]
    fn text(&self) -> &str {
        &self.text
    }

    #[graphql(description = "The date the comment was created")]
    fn created(&self) -> String {
        format!("{}", self.created)
    }

    #[graphql(description = "The person that created it")]
    fn created_by(&self) -> &Option<String> {
        &self.created_by
    }
}

impl Comment {
    pub async fn fetch_by_item_id(item_id: &str, pool: &Pool) -> Result<Vec<Comment>> {
        Ok(sqlx::query_as(
            "SELECT * FROM comment WHERE comment.item_id=$1 ORDER BY comment.created DESC",
        )
        .bind(item_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn create(
        item_id: &str,
        text: &str,
        created_by: Option<&str>,
        pool: &Pool,
    ) -> Result<Comment> {
        let result =  sqlx::query_as("INSERT INTO comment (id, item_id, text, created_by, created) VALUES (gen_random_uuid(), $1, $2, $3, CURRENT_TIMESTAMP) RETURNING *")
                .bind(item_id)
                .bind(text)
                .bind(created_by)
                .fetch_one(pool)
                .await?;
        tracing::info!(
            schema = "Comment",
            task = "add",
            result = "success",
            text = text,
        );

        Ok(result)
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Comment Input")]
pub struct CommentInput {
    pub item_id: String,
    pub text: String,
    pub created_by: Option<String>,
}
