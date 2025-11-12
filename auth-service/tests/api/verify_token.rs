use crate::helpers::{get_random_email, TestApp};

use auth_service::{domain::error::ErrorResponse, utils::constants::JWT_COOKIE_NAME};
use auth_service_macros::api_test;
use secrecy::ExposeSecret;

#[api_test]
async fn should_return_200_valid_token() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email.expose_secret(),
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email.expose_secret(),
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();

    let verify_token_body = serde_json::json!({
        "token": &token,
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[api_test]
async fn should_return_401_if_invalid_token() {
    let token = serde_json::json!({
        "token": "invalidToken"
    });

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[api_test]
async fn should_return_401_if_banned_token() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email.expose_secret(),
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email.expose_secret(),
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    // ---------------------------------------------------------

    let verify_token_body = serde_json::json!({
        "token": token,
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid auth token".to_owned()
    );
}

#[api_test]
async fn should_return_422_if_malformed_input() {
    let token = serde_json::json!({});

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 422);
}
