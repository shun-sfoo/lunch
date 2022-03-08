use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("missing {0} param in environment")]
    MissingEnvParam(String),
    #[error("path {0} is unattach")]
    Path(String),
}
