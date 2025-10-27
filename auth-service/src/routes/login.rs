use crate::{
    app_state::app_state::AppState,
    domain::{
        data_stores::{LoginAttemptId, TwoFACode},
        email::Email,
        error::AuthAPIError,
        password::Password,
    },
    utils::auth::generate_auth_cookie,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{self, Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
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

    let user_store = &state.user_store.read().await;

    if user_store
        .validate_user(&valid_email, &valid_password)
        .await
        .is_err()
    {
        return (cookie_jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&valid_email).await {
        Ok(user) => user,
        _ => return (cookie_jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&valid_email) {
        Ok(cookie) => cookie,
        _ => return (cookie_jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = cookie_jar.add(auth_cookie);

    if !user.has_2fa() {
        return handle_no_2fa(&user.email(), updated_jar.clone()).await;
    }

    handle_2fa(&valid_email, &state, updated_jar.clone()).await
}

fn parse_credentials(email: String, password: String) -> Result<(Email, Password), AuthAPIError> {
    let email = Email::parse(email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    Ok((email, password))
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    let add_login_attempt_id = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await;

    if add_login_attempt_id.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let send_to_email_client: Result<(), String> = state
        .email_client
        .write()
        .await
        .send_email(&email, "2FA Code", two_fa_code.as_ref())
        .await;

    if send_to_email_client.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        login_attempt_id: login_attempt_id.into(),
        message: "2FA required".into(),
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let response = Json(LoginResponse::RegularAuth);

    (jar, Ok((StatusCode::OK, response)))
}
