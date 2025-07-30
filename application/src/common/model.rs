use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ApiResult<T>
where
    T: Serialize,
{
    pub result: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResult<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            result: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn sucess_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            result: 0,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn failure(result: u16, message: impl Into<String>) -> Self {
        Self {
            result,
            message: message.into(),
            data: None,
        }
    }
}

impl<T: Serialize + Clone> IntoResponse for ApiResult<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
