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
