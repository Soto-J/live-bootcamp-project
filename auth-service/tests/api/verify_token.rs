use auth_service::{domain::Email, utils::generate_auth_cookie};

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

    assert_eq!(response.status().as_u16(), 400)
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let token = serde_json::json!({});

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 422)
}
