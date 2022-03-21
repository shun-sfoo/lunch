use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("missing param {0} in environment")]
    MissingEnvParam(String),
    #[error("axum make into service error")]
    Serve,
    #[error("database connect error")]
    DbConnect,
    #[error("create table {0} error")]
    CreatTable(String),
    #[error("select {0} by {1} error")]
    Find(String, String),
    #[error("create {0} by {1} error")]
    Create(String, String),
}

#[allow(dead_code)]
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
