use crate::Error;
use once_cell::sync::Lazy;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use std::sync::Arc;
use std::time::Duration;
use util::ResultLogger;

pub type DatabaseConnection = sqlx::pool::PoolConnection<Database>;
pub type Database = sqlx::Postgres;

#[derive(Clone, Debug)]
pub struct DatabasePool {
    inner_sqlx: Arc<sqlx::PgPool>,
    inner_read_only: Option<Arc<sqlx::PgPool>>,
}

pub struct DatabasePoolOptions {
    pub max_size: u32,
    pub min_size: u32,
    pub connect_timeout: Duration,
}

impl DatabasePool {
    pub async fn new_from_config(url: &str, ro_url: &Option<String>) -> Result<Self, Error> {
        let db_options = crate::DatabasePoolOptions {
            max_size: *crate::MAX_DATABASE_POOL_SIZE,
            min_size: *crate::MIN_DATABASE_POOL_SIZE,
            connect_timeout: *crate::DATABASE_CONNECTION_TIMEOUT,
        };

        DatabasePool::new(&db_options, &url, ro_url).await
    }

    pub async fn new(
        options: &DatabasePoolOptions,
        url: &str,
        ro_url: &Option<String>,
    ) -> Result<Self, Error> {
        let enable_logging = enable_sqlx_logging();

        let connection_options: PgConnectOptions = url.parse().log_warn()?;

        let connection_options = if !enable_logging {
            connection_options.disable_statement_logging()
        } else {
            connection_options
        };

        let pool = PgPoolOptions::new()
            .acquire_timeout(options.connect_timeout)
            .max_connections(options.max_size)
            .min_connections(options.min_size)
            .connect_with(connection_options)
            .await
            .log_warn()?;

        let ro_pool = if let Some(url) = ro_url {
            let connection_options: PgConnectOptions = url.parse().log_warn()?;

            let connection_options = if !enable_logging {
                connection_options.disable_statement_logging()
            } else {
                connection_options
            };

            Some(
                PgPoolOptions::new()
                    .acquire_timeout(options.connect_timeout)
                    .max_connections(options.max_size)
                    .min_connections(options.min_size)
                    .max_lifetime(*crate::MAX_CONNECTION_LIFETIME)
                    .connect_with(connection_options)
                    .await?,
            )
        } else {
            None
        };

        Ok(Self {
            inner_sqlx: Arc::new(pool),
            inner_read_only: ro_pool.map(|p| Arc::new(p)),
        })
    }

    pub async fn acquire(&self) -> Result<DatabaseConnection, Error> {
        Ok(self.inner_sqlx.acquire().await?)
    }

    ///returns a read only connection if available. will use the primary writer otherwise
    pub async fn acquire_read_only(&self) -> Result<DatabaseConnection, Error> {
        if let Some(ro) = &self.inner_read_only {
            Ok(ro.acquire().await?)
        } else {
            self.acquire().await
        }
    }
}

pub fn enable_sqlx_logging() -> bool {
    if let Ok(value) = std::env::var("SQLX_LOG") {
        value == "1"
    } else {
        false
    }
}

pub static DATABASE_CONNECTION_TIMEOUT: Lazy<std::time::Duration> = Lazy::new(|| {
    let seconds = std::env::var("DATABASE_CONNECTION_TIMEOUT")
        .unwrap_or("5".into())
        .parse::<u64>()
        .unwrap_or(5);
    std::time::Duration::from_secs(seconds)
});

pub static MAX_CONNECTION_LIFETIME: Lazy<std::time::Duration> = Lazy::new(|| {
    let hours = std::env::var("MAX_CONNECTION_LIFETIME_HOURS")
        .unwrap_or("24".into())
        .parse::<u64>()
        .unwrap_or(24);
    std::time::Duration::from_secs(hours * 3600)
});

pub static MAX_DATABASE_POOL_SIZE: Lazy<u32> = Lazy::new(|| {
    std::env::var("MAX_DATABASE_POOL_SIZE")
        .unwrap_or("16".into())
        .parse::<u32>()
        .unwrap_or(16)
});

//always keep this many connections available in the pool
pub static MIN_DATABASE_POOL_SIZE: Lazy<u32> = Lazy::new(|| {
    std::env::var("MIN_DATABASE_POOL_SIZE")
        .unwrap_or("1".into())
        .parse::<u32>()
        .unwrap_or(1)
});
