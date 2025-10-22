use auth_service::{
    domain::{BannedTokenStore, Email},
    utils::generate_auth_cookie,
};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let email = Email::parse(get_random_email()).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();

    let token = serde_json::json!({
        "token": cookie.value()
    });

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 200)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let token = serde_json::json!({
        "token": "invalidToken"
    });

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 401)
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;
    let mut banned_token_store = app.banned_token_store.write().await;

    let fake_token = "fake token";
    banned_token_store.store_token(&fake_token).unwrap();

    let token = serde_json::json!({
        "token": fake_token
    });

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 401)
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let token = serde_json::json!({});

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 422)
}
