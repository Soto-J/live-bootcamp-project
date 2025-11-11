use crate::{
    app_state::app_state::AppState,
    domain::{
        data_stores::{LoginAttemptId, TwoFACode},
        error::AuthAPIError,
        Email,
    },
    utils::auth::generate_auth_cookie,
};

use ::serde::{Deserialize, Serialize};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;
use secrecy::Secret;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    pub login_attempt_id: String,
    pub two_fa_code: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Verify2FAResponse {
    pub message: String,
}

#[tracing::instrument(name = "Verify_2FA", skip_all)]
pub async fn verify_2fa_handler(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let (Ok(email), Ok(login_attempt_id_request), Ok(two_fa_code_request)) = (
        Email::parse(Secret::new(request.email)),
        LoginAttemptId::parse(request.login_attempt_id),
        TwoFACode::parse(request.two_fa_code),
    ) else {
        return (cookie_jar, Err(AuthAPIError::InvalidCredentials));
    };

    let mut two_fa_store = state.two_fa_code_store.write().await;

    let (login_attempt_id, two_fa_code) = match two_fa_store.get_code(&email).await {
        Ok(tfa_tuple) => tfa_tuple,
        _ => return (cookie_jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    if login_attempt_id_request.ne(&login_attempt_id) || two_fa_code_request.ne(&two_fa_code) {
        return (cookie_jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(e) => return (cookie_jar, Err(AuthAPIError::UnexpectedError(e))),
    };

    let updated_jar = cookie_jar.add(auth_cookie);

    if let Err(e) = two_fa_store.remove_code(&email).await {
        return (updated_jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    (updated_jar, Ok(StatusCode::OK.into_response()))
}
