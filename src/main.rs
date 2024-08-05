//! Actix Web juniper example
//!
//! A simple example integrating juniper in Actix Web
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;
use tracing_actix_web::TracingLogger;

use self::database::get_db_pool;

mod database;
mod handlers;
mod schema;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = get_db_pool().await?;

    // Create Juniper schema

    tracing::info!("starting HTTP server on port 8080");
    tracing::info!("GraphiQL playground: http://localhost:8080/api/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // the graphiql UI requires CORS to be enabled
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(pool.clone()))
            .configure(handlers::api_handlers::register)
            .configure(handlers::static_handlers::register)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
