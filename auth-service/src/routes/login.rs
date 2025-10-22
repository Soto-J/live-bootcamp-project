use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStore},
    utils::generate_auth_cookie,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{self, Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let (valid_email, valid_password) = match parse_credentials(request.email, request.password) {
        Ok(valid_credentials) => valid_credentials,
        _ => return (cookie_jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = state.user_store.read().await;

    let _ = match get_user(&*user_store, &valid_email, &valid_password).await {
        Ok(user) => user,
        Err(_) => return (cookie_jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&valid_email) {
        Ok(cookie) => cookie,
        Err(_) => return (cookie_jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = cookie_jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}

fn parse_credentials(email: String, password: String) -> Result<(Email, Password), AuthAPIError> {
    let email = Email::parse(email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    Ok((email, password))
}

async fn get_user(
    user_store: &dyn UserStore,
    email: &Email,
    password: &Password,
) -> Result<User, AuthAPIError> {
    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    match user_store.get_user(&email).await {
        Ok(user) => Ok(user),
        Err(_) => Err(AuthAPIError::IncorrectCredentials),
    }
}
