use axum::{extract::FromRequestParts, http::request};
use serde::{Deserialize, Serialize};

use crate::common::error::AppError;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Userinfo {}

impl<S> FromRequestParts<S> for Userinfo
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "microservice")]
        {
            let auth = parts
                .headers
                .get("Authorization")
                .ok_or(AppError::AuthenticationError)?
                .to_str()
                .map_err(|_| AppError::AuthenticationError)?;
            if let Some(token) = auth.strip_prefix("Bearer ") {
                use bincode::decode_from_slice;

                let bytes = base64::decode(token).map_err(|_| AppError::AuthenticationError)?;
                let (userinfo, _): (Userinfo, usize) =
                    decode_from_slice(&bytes, config::standard())
                        .map_err(|_| AppError::AuthenticationError)?;

                Ok(user_info)
            } else {
                AppError::AuthenticationError
            }
        }

        #[cfg(feature = "monolith")]
        {
            parts
                .extensions
                .get::<Userinfo>()
                .cloned()
                .ok_or(AppError::AuthenticationError)
        }
    }
}
