use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Forbidden request")]
    Forbidden,

    #[error("Authentication error")]
    AuthenticationError,
}
