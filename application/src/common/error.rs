use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use super::model::ApiResult;

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

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::AuthenticationError => StatusCode::UNAUTHORIZED,
        };

        let body = axum::Json(ApiResult::<()> {
            result: status.as_u16(),
            message: self.to_string(),
            data: None,
        });

        (status, body).into_response()
    }
}
