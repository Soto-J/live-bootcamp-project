use auth_service::{
    routes::{LoginRequest, SignupRequest},
    ErrorResponse,
};

use crate::helpers::{get_random_email, get_random_password, TestApp};

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
        email: "hello world".into(),
        password: "".into()
    });

    let response = app.login_root(&login_credentials).await;

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

    app.post_signup(&serde_json::json!(LoginRequest { email, password }))
        .await;

    let email = get_random_email();
    let password = get_random_password();

    let credentials = serde_json::json!(LoginRequest { email, password });

    let response = app.login_root(&credentials).await;

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

// #[tokio::test]
// pub async fn should_return_422_if_malformed_credentials() {
//     let app = TestApp::new().await;

//     let email = get_random_email();
//     let password = get_random_password();

//     let credentials = serde_json::json!(SignupRequest {
//         email,
//         password,
//         requires_2fa: true
//     });

//     app.post_signup(&credentials).await;

//     let invalid_credentials = serde_json::json!(LoginRequest {
//         email: "hello world".into(),
//         password: "".into(),
//     });

//     let response = app.login_root(&invalid_credentials).await;

//     assert_eq!(response.status().as_u16(), 422)
// }
