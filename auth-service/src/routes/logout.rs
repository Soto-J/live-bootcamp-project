use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{validate_token, JWT_COOKIE_NAME},
};

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

pub async fn logout_handler(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match cookie_jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (cookie_jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    let _ = match validate_token(&token, state.banned_token_store.clone()).await {
        Ok(claims) => claims,
        Err(_) => return (cookie_jar, Err(AuthAPIError::InvalidToken)),
    };

    if state
        .banned_token_store
        .write()
        .await
        .add_token(token)
        .await
        .is_err()
    {
        return (cookie_jar, Err(AuthAPIError::UnexpectedError));
    }

    let cookie_jar = cookie_jar.remove(JWT_COOKIE_NAME);

    (cookie_jar, Ok(StatusCode::OK))
}
