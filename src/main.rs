//! Actix Web juniper example
//!
//! A simple example integrating juniper in Actix Web

use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use anyhow::Result;

use self::{database::get_db_pool, handlers::register};

mod database;
mod handlers;
mod schema;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = get_db_pool()?;

    // Create Juniper schema

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(register)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
