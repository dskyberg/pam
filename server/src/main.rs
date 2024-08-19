//! Actix Web juniper example
//!
//! A simple example integrating juniper in Actix Web
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;
use std::process;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

use database::get_db_pool;
mod database;
mod handlers;
mod schema;

lazy_static::lazy_static! {
    static ref SERVER_HOST: String = std::env::var("SERVER_HOST").expect("missing environment variable SERVER_HOST");
    static ref SERVER_PORT: u16 = std::env::var("SERVER_PORT").expect("missing environment variable SERVER_PORT").parse::<u16>().expect("SERVER_PORT must be u16");
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let (layer, task) = tracing_loki::builder()
        .label("host", "mine")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(layer)
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        .init();
    tokio::spawn(task);

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
