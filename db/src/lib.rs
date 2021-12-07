pub mod errors;
pub mod models;
mod rds;

pub use errors::Error;

pub use rds::{
    Database, DatabaseConnection, DatabasePool, DatabasePoolOptions, DATABASE_CONNECTION_TIMEOUT,
    MAX_CONNECTION_LIFETIME, MAX_DATABASE_POOL_SIZE, MIN_DATABASE_POOL_SIZE,
};

pub type Id = uuid::Uuid;
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Debug)]
pub struct Context {
    pub pool: DatabasePool,
}

impl Context {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }
}
