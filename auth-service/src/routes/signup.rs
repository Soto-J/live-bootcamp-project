use crate::{
    app_state::app_state::AppState,
    domain::{
        data_stores::UserStore, email::Email, error::AuthAPIError, password::Password, user::User,
    },
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
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

// naming the span “Signup”, skipping all arguments from being recorded in the trace, 
// and capturing errors with Debug formatting if they occur. 
#[tracing::instrument(name = "Signup", skip_all, err(Debug))]
pub async fn signup_handler(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(user.email()).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    };

    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "User created successfully!".into(),
        }),
    ))
}
