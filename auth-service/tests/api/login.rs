use auth_service::routes::{LoginRequest, SignupRequest};

use crate::helpers::{get_random_email, get_random_password, TestApp};

#[tokio::test]
pub async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = get_random_password();

    let credentials = serde_json::json!(LoginRequest { email, password });

    app.post_signup(&credentials).await;

    let invalid_credentials = serde_json::json!(SignupRequest {
        email: get_random_email(),
        password: get_random_password()
    });

    let response = app.login_root(&invalid_credentials).await;

    assert_eq!(response.status().as_u16(), 422)
}

#[tokio::test]
pub async fn should_return_400_if_invalid_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = get_random_password();

    let signup_credentials = serde_json::json!(LoginRequest {
        email: email.clone(),
        password
    });
    app.post_signup(&signup_credentials).await;

    let login_credentials = serde_json::json!(LoginRequest {
        email,
        password: get_random_password()
    });
    let response = app.login_root(&login_credentials).await;

    assert_eq!(response.status().as_u16(), 400)
}

#[tokio::test]
pub async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = get_random_password();

    let credentials = serde_json::json!(LoginRequest { email, password });
    app.post_signup(&credentials).await;
}
