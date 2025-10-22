use crate::helpers::{get_random_email, get_random_password, TestApp};

use auth_service::{
    routes::{LoginRequest, SignupRequest},
    utils::JWT_COOKIE_NAME,
    ErrorResponse,
};

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
pub async fn should_return_400_if_invalid_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = get_random_password();

    let signup_credentials = serde_json::json!(SignupRequest {
        email: email.clone(),
        password,
        requires_2fa: false
    });

    app.post_signup(&signup_credentials).await;

    let login_credentials = serde_json::json!(LoginRequest {
        email: "hello world".into(),
        password: "".into()
    });

    let response = app.post_login(&login_credentials).await;

    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid credentials"
    )
}

#[tokio::test]
pub async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = get_random_password();

    app.post_signup(&serde_json::json!(SignupRequest {
        email,
        password,
        requires_2fa: false
    }))
    .await;

    let email = get_random_email();
    let password = get_random_password();

    let credentials = serde_json::json!(LoginRequest { email, password });

    let response = app.post_login(&credentials).await;

    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Unathorized"
    )
}

#[tokio::test]
pub async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = get_random_password();

    let credentials = serde_json::json!(SignupRequest {
        email,
        password,
        requires_2fa: true
    });

    app.post_signup(&credentials).await;

    let malformed_credentials = serde_json::json!({});

    let response = app.post_login(&malformed_credentials).await;

    assert_eq!(response.status().as_u16(), 422)
}
