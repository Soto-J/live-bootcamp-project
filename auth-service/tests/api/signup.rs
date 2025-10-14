use crate::helpers::{get_random_email, get_random_password, TestApp};
use auth_service::{
    app_state::AppState,
    routes::{signup, SignupRequest, SignupResponse},
    services::HashmapUserStore,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tokio::sync::RwLock;

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
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // Create an array of invalid inputs. Then, iterate through the array and
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.

    let test_cases = [
        axum::Json(SignupRequest {
            email: "randomemail.com".to_string(),
            password: get_random_password(),
            requires_2fa: true,
        }),
        axum::Json(SignupRequest {
            email: get_random_email(),
            password: "12345".to_string(),
            requires_2fa: true,
        }),
        axum::Json(SignupRequest {
            email: get_random_email(),
            password: "".to_string(),
            requires_2fa: true,
        }),
    ];

    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));

    let state = State(AppState::new(user_store));

    for test_case in test_cases {
        let response = signup(state.clone(), test_case).await.into_response();

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "Expect to return 400 - BAD_REQUEST."
        )
    }
}
