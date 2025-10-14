use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User, services::UserStoreError};

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup_handler(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let mut user_store = state.user_store.write().await;

    if request.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(SignupResponse {
                message: "Password is too short.".to_string(),
            }),
        );
    }

    if request.email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(SignupResponse {
                message: "Email cannot be empty.".to_string(),
            }),
        );
    }

    if !request.email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(SignupResponse {
                message: "Invalid email format.".to_string(),
            }),
        );
    }

    let user = User::new(request.email, request.password, request.requires_2fa);
    user_store.add_user(user).unwrap();

    (
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "User created successfully!".to_string(),
        }),
    )
}

pub async fn signup_malformed_request_422() -> impl IntoResponse {
    StatusCode::UNPROCESSABLE_ENTITY.into_response()
}
