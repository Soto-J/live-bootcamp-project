use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    Email::parse(request.email.clone())?;
    Password::parse(request.password.clone())?;

    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if let Err(_) = user_store.add_user(user).await {
        return Err(AuthAPIError::UserAlreadyExists);
    };

    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "User created successfully!".to_string(),
        }),
    ))
}

pub async fn signup_malformed_request_422() -> impl IntoResponse {
    StatusCode::UNPROCESSABLE_ENTITY.into_response()
}
