//! Actix Web juniper example
//!
//! A simple example integrating juniper in Actix Web
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;
use tracing_actix_web::TracingLogger;

use database::get_db_pool;

mod database;
mod handlers;
mod o11y;
mod schema;

lazy_static::lazy_static! {
    static ref SERVER_HOST: String = std::env::var("SERVER_HOST").expect("missing environment variable SERVER_HOST");
    static ref SERVER_PORT: u16 = std::env::var("SERVER_PORT").expect("missing environment variable SERVER_PORT").parse::<u16>().expect("SERVER_PORT must be u16");
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tokio::spawn(o11y::build()?);

    let pool = get_db_pool().await?;

    // Create Juniper schema
    println!("starting HTTP server on {}:{}", *SERVER_HOST, *SERVER_PORT);
    println!(
        "graphiql playground at http://{}:{}/graph/graphiql",
        *SERVER_HOST, *SERVER_PORT
    );
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // the graphiql UI requires CORS to be enabled
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(pool.clone()))
            .configure(handlers::graph_handlers::register)
            .configure(handlers::api_handlers::register)
            .configure(handlers::static_handlers::register)
    })
    .workers(2)
    .bind(((*SERVER_HOST).clone(), *SERVER_PORT))?
    .run()
    .await?;

    Ok(())
}
