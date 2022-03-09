use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("missing param {0} in environment")]
    MissingEnvParam(String),
    #[error("database connect error")]
    DbConnect,
    #[error("create table {0} error")]
    CreatTable(String),
    #[error("axum make into service error")]
    Serve,
}

#[derive(Debug)]
pub enum HttpError {
    Auth,     // 401
    Internal, // 500
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server, Error"),
        };
        (code, msg).into_response()
    }
}
