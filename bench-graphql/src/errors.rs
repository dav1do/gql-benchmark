use async_graphql::ErrorExtensions;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("BadInput (field: {field:?}, reason: {message:?})")]
    BadInput { field: String, message: String },
    #[error("{0}")]
    Custom(String),
    #[error("Database Error: {0:?}")]
    DatabaseError(#[from] db::Error),
    #[error("DataLoaderError: {0:?}")]
    DataLoaderError(#[from] DataLoaderError),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            Error::BadInput { field, message } => {
                tracing::debug!("Bad input: {}: {}", &field, &message);
                e.set("code", "BAD_USER_INPUT");
                e.set("field", field.as_str());
            }
            error => {
                tracing::error!("Error occured during graphql execution: {:?}", &error);
                e.set("code", "INTERNAL_SERVER_ERROR");
            }
        })
    }
}

impl Error {
    pub fn custom<T: Into<String>>(v: T) -> Self {
        Self::Custom(v.into())
    }
}

#[derive(Debug, Clone, ThisError)]
pub enum DataLoaderError {
    #[error("{0}")]
    Custom(String),
    #[error("SQL error: {0:?}")]
    UnexpectedSqlError(String),
    #[error("Unexpected database error: {0:?}")]
    UnhandledDatabaseError(String),
}

impl From<db::Error> for DataLoaderError {
    fn from(e: db::Error) -> Self {
        match e {
            db::Error::Sqlx(e) => Self::UnexpectedSqlError(e.to_string()),
            db::Error::Custom(msg) => Self::Custom(msg),
            e => Self::UnhandledDatabaseError(e.to_string()),
        }
    }
}
