use application::{AppState, common::error::AppError};
use axum::{extract::State, response::IntoResponse};

use crate::domains::hello::Hello;

pub async fn hello(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let h = Hello::new(state.db()).hello()?;
    Ok(h)
}
