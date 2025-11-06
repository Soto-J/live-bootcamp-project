use crate::helpers::{get_random_email, get_random_password, TestApp};

use auth_service::{
    domain::{email::Email, error::ErrorResponse},
    routes::{
        login::{LoginRequest, TwoFactorAuthResponse},
        signup::SignupRequest,
    },
    utils::constants::JWT_COOKIE_NAME,
};

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let mut app = TestApp::new().await;

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

    app.clean_up().await
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app = TestApp::new().await;

    let email = get_random_email();
    let password = get_random_password();

    let signup_credentials = serde_json::json!(SignupRequest {
        email: email.clone(),
        password: password.clone(),
        requires_2fa: true
    });

    app.post_signup(&signup_credentials).await;

    let login_credentials = serde_json::json!(LoginRequest {
        email: email.clone(),
        password,
    });

    let response = app.post_login(&login_credentials).await;

    assert_eq!(response.status().as_u16(), 206);
    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let two_fa_code_store = app.two_fa_code_store.clone();

    let response = two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(email).unwrap())
        .await
        .unwrap();

    assert_eq!(json_body.login_attempt_id, response.0);

    app.clean_up().await
}

#[tokio::test]
pub async fn should_return_400_if_invalid_credentials() {
    let mut app = TestApp::new().await;

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
    );

    app.clean_up().await
}

#[tokio::test]
pub async fn should_return_401_if_incorrect_credentials() {
    let mut app = TestApp::new().await;

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
        "Incorrect credentials"
    );

    app.clean_up().await
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let test_cases = [
        serde_json::json!({
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

    app.clean_up().await
}
