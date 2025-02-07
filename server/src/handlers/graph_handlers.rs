use actix_web::{get, route, web, Error, HttpResponse, Responder};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::{
    database::Pool,
    schema::root::{create_schema, Context, Schema},
};

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let json = serde_json::to_string_pretty(&data).expect("Failed to serialize data");
    tracing::info!("{}", &json);
    let ctx = Context {
        db_pool: pool.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

/// GraphiQL UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(graphiql_source("/graph/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config.app_data(web::Data::new(create_schema())).service(
        web::scope("graph")
            .service(graphql)
            .service(graphql_playground),
    );
}
