use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Sqlx: {0:?}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Serde: {0:?}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    Custom(String),
}

impl Error {
    pub fn custom<T: Into<String>>(v: T) -> Self {
        Self::Custom(v.into())
    }
}
