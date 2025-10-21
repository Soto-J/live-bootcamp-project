use crate::{
    domain::AuthAPIError,
    utils::{validate_token, JWT_COOKIE_NAME},
};

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

pub async fn logout_handler(
    cookie_jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match cookie_jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (cookie_jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    match validate_token(&token).await {
        Ok(_) => {}
        Err(_) => return (cookie_jar, Err(AuthAPIError::InvalidToken)),
    };

    let cookie_jar = cookie_jar.remove(JWT_COOKIE_NAME);

    (cookie_jar, Ok(StatusCode::OK))
}
