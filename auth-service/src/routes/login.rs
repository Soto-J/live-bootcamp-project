use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{self, Deserialize, Serialize};
use validator::Validate;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
};

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    Email::parse(request.email.clone())?;
    Password::parse(request.password.clone())?;

    let user_store = state.user_store.read().await;

    let response = user_store
        .login_user(&request.email, &request.password)
        .await;

    match response {
        Ok(_) => Ok((
            StatusCode::ACCEPTED,
            Json(LoginResponse {
                message: "User logged in successfully".into(),
            }),
        )),
        Err(_) => Err(AuthAPIError::InvalidCredentials),
    }
}
