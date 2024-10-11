use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal server error")]
    Anyhow(#[from] anyhow::Error),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Resource not found")]
    NotFound,
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Anyhow(_) | Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Anyhow(ref e) => tracing::error!("Generic error: {}", e),
            Self::Database(ref e) => tracing::error!("Database error: {}", e),
            _ => (),
        };

        let message = match self {
            Error::NotFound => self.to_string(),
            _ => "An internal error occurred".to_string(),
        };

        (self.status_code(), message).into_response()
    }
}