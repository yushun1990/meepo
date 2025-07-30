use application::AppState;
use axum::{Router, routing::get};

use super::handlers::hello;

pub fn hello_routes() -> Router<AppState> {
    Router::new().route("/hello", get(hello))
}
