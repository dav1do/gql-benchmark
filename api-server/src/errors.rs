use thiserror::Error as ThisError;
use tracing::error;

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Database Error: {0:?}")]
    Database(#[from] db::Error),
    #[error("Actix: {0:?}")]
    Actix(#[from] actix_web::Error),
    #[error("Serde: {0:?}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("IO Error: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Custom(String),
}

impl Error {
    pub fn custom<T: Into<String>>(v: T) -> Self {
        Self::Custom(v.into())
    }
}

impl async_graphql::ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            error => {
                tracing::error!("Error occured during graphql execution: {:?}", &error);
                e.set("code", "INTERNAL_SERVER_ERROR");
            }
        })
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::SerdeJson(_) => http::StatusCode::BAD_REQUEST,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR, //shouldn't be setting 500 on failed graphql errors
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::SerdeJson(e) => HttpResponse::build(self.status_code()).body(format!("{:?}", e)),
            _ => HttpResponse::build(self.status_code()).body(format!("{:?}", self)),
        }
    }
}
