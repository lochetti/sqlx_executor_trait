use std::env;
use std::sync::Arc;

use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, PgPool};

pub async fn prepare_db_and_get_connection() -> anyhow::Result<Arc<PgPool>> {
    let url = env::var("DATABASE_URL")?;
    let parsed_url = url::Url::parse(&url)?;

    let opts = PgConnectOptions::from_url(&parsed_url)?;
    let pool = PgPool::connect_with(opts).await?;

    Ok(Arc::new(pool))
}

#[cfg(test)]
pub mod test {
    use dotenvy::dotenv;
    use std::env;

    use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool, Postgres, Transaction};

    pub async fn get_test_connection<'a>() -> anyhow::Result<Transaction<'a, Postgres>> {
        dotenv().ok();
        let url = env::var("DATABASE_URL")?;
        let parsed_url = url::Url::parse(&url)?;

        let opts = PgConnectOptions::from_url(&parsed_url)?;
        let pool = PgPool::connect_with(opts).await?;

        Ok(pool.begin().await?)
    }
}
