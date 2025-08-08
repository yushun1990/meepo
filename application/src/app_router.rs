use std::time::Duration;

use axum::{
    BoxError, Router,
    error_handling::HandleErrorLayer,
    http::{
        Method, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    response::IntoResponse,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    AppState,
    common::{error::AppError, model::ApiResult},
};

pub fn create_router(
    state: AppState,
    sub_routes: Router<AppState>,
    timeout_secs: u64,
    enable_swagger: bool,
) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    // create common middleware stack for error handling, timeout, and CORS
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(timeout_secs))
        .layer(cors);

    if enable_swagger {
        tracing::info!("Swagger enabled, close it when deploy to prod.");
    }

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .merge(sub_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    tracing::info_span!("request", method=%req.method(), uri=%req.uri())
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                    latency: std::time::Duration,
                    _span: &tracing::Span| {
                    tracing::info!(
                        "request completed status={status}, latency={latency:?}",
                        status=response.status(), latency=latency
                    );
                })
        )
        .fallback(fallback)
        .layer(middleware_stack)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "Ok\n"
}

async fn fallback() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NOT_FOUND, "Not Found"))
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    let status = if error.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let message = error.to_string();
    tracing::error!(?status, %message, "Request failed");

    let body = ApiResult::<()>::failure(status.as_u16(), message);

    (status, body)
}
