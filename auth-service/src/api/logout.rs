use axum::{http::StatusCode, response::IntoResponse};

async fn logout_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
