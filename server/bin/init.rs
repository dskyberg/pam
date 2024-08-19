use anyhow::Result;

use database::get_db_pool;

use initialize::initialize_db;
mod initialize;
lazy_static::lazy_static! {
    static ref SERVER_HOST: String = std::env::var("SERVER_HOST").expect("missing environment variable SERVER_HOST");
    static ref SERVER_PORT: u16 = std::env::var("SERVER_PORT").expect("missing environment variable SERVER_PORT").parse::<u16>().expect("SERVER_PORT must be u16");
}

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let pool = get_db_pool().await?;

    initialize_db(&pool).await?;

    Ok(())
}
