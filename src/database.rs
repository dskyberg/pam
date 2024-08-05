use anyhow::{anyhow, Result};
use sqlx::{postgres::PgPoolOptions, Postgres};

pub type Pool = sqlx::Pool<Postgres>;

pub async fn get_db_pool() -> Result<Pool> {
    let url = std::env::var("DATABASE_URL")?;
    tracing::info!("{}", &url);

    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await?)
}

pub fn paginate(sql: &str, page_size: Option<i32>, page: Option<i32>) -> Result<String> {
    let mut s = sql.to_owned();
    if let Some(p) = page_size {
        if p < 1 {
            return Err(anyhow!("page size must be > 0"));
        }
        s.push_str(&format!(" LIMIT {}", p));
    }
    if let Some(p) = page {
        let Some(page_size) = page_size else {
            return Err(anyhow!("page with no page size"));
        };

        if p < 1 {
            return Err(anyhow!("page must be > 0"));
        }
        if p > 1 {
            s.push_str(&format!(" OFFSET {}", page_size * (p - 1)));
        }
    }
    Ok(s)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(paginate("", None, None).is_ok());
        assert!(paginate("", Some(1), None).is_ok());
        assert!(paginate("", Some(1), Some(1)).is_ok());

        assert!(paginate("", Some(0), None).is_err());
        assert!(paginate("", Some(1), Some(0)).is_err());
        assert!(paginate("", None, Some(1)).is_err());

        let result = paginate("", Some(10), Some(1));
        assert_eq!(result.unwrap(), " LIMIT 10");

        let result = paginate("SELECT", Some(10), Some(-2));
        assert!(result.is_err());
    }
}
