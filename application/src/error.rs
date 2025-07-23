use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Fatal error: {0}; exit!")]
    FatalError(String),
}
