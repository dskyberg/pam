//! Actix Web juniper example
//!
//! A simple example integrating juniper in Actix Web
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use anyhow::{anyhow, Result};
use tracing_actix_web::TracingLogger;

use database::get_db_pool;

mod database;
mod handlers;
mod o11y;
mod schema;

fn env_or_else(label: &str) -> Result<String> {
    std::env::var(label).map_err(|_| anyhow!(format!("Missing environment variable: {}", label)))
}

fn env_as_or_else<T: std::str::FromStr>(label: &str) -> Result<T> {
    std::env::var(label)
        .map_err(|_| anyhow!(format!("Missing environment variable: {}", label)))?
        .parse::<T>()
        .map_err(|_| anyhow!("Value must be {}", std::any::type_name::<T>()))
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let server_host = env_or_else("SERVER_HOST")?;
    let server_port = env_as_or_else::<u16>("SERVER_PORT")?;

    let (controller, task) = o11y::init_tracing()?;

    let pool = get_db_pool().await?;

    // Create Juniper schema
    println!("starting HTTP server on {}:{}", &server_host, server_port);
    println!(
        "graphiql playground at http://{}:{}/graph/graphiql",
        &server_host, server_port
    );
    tracing::info!(
        task = "start",
        result = "success",
        "starting HTTP server on {}:{}",
        &server_host,
        server_port
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
    .bind(((server_host).clone(), server_port))?
    .run()
    .await?;

    controller.shutdown().await;
    task.await?;

    Ok(())
}
