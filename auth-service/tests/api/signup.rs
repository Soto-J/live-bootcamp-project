use {
    crate::helpers::{get_random_email, get_random_password, TestApp},
    auth_service::{
        app_state::AppState,
        routes::{signup, SignupRequest, SignupResponse},
        services::{HashmapUserStore, UserStoreError},
        ErrorResponse,
    },
    axum::{
        extract::State,
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    std::sync::Arc,
    tokio::sync::RwLock,
};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
           "password": "password123",
           "requires2FA": true
        }),
        serde_json::json!({
           "email": get_random_email(),
           "requires2FA": false
        }),
        serde_json::json!({
           "email": get_random_email(),
           "password": "password123123",
        }),
        serde_json::json!({
           "email": get_random_email(),
        }),
        serde_json::json!({
           "password": "password123123",
        }),
        serde_json::json!({
           "password": "password123123",
        }),
        serde_json::json!({
         "requires2FA": false
        }),
        serde_json::json!({
         "requires2FA": true
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
pub async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": get_random_email(),
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": "password123123",
            "requires2FA": false
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            201,
            "Failed for input: {:?}",
            test_case
        )
    }

    let response = app
        .post_signup(&serde_json::json!({
            "email": get_random_email(),
            "password": get_random_password(),
            "requires2FA": true
        }))
        .await;

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
pub async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!(SignupRequest {
            email: "randomemail.com".to_string(),
            password: get_random_password(),
            requires_2fa: true,
        }),
        serde_json::json!(SignupRequest {
            email: get_random_email(),
            password: "1234567".to_string(),
            requires_2fa: true,
        }),
        serde_json::json!(SignupRequest {
            email: get_random_email(),
            password: "".to_string(),
            requires_2fa: true,
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
pub async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let user = serde_json::json!(SignupRequest {
        email: get_random_email(),
        password: get_random_password(),
        requires_2fa: true,
    });

    app.post_signup(&user).await;

    let response = app.post_signup(&user).await;

    assert_eq!(
        response.status().as_u16(),
        409,
        "Expects to return 409 - CONFLICT."
    )
}
