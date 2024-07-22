use anyhow::Result;
use r2d2_mysql::{
    mysql::{Opts, OptsBuilder},
    MySqlConnectionManager,
};

pub type Pool = r2d2::Pool<MySqlConnectionManager>;

pub fn get_db_pool() -> Result<Pool> {
    let password = std::env::var("MYSQL_ROOT_PASSWORD")?;
    let schema = std::env::var("MYSQL_SCHEMA")?;
    let db_url = format!("mysql://root:{password}@127.0.0.1/{schema}");
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    Ok(r2d2::Pool::new(manager)?)
}
