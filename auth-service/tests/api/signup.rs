use crate::helpers::{get_invalid_password, get_random_email, get_random_password, TestApp};

use auth_service::{
    domain::error::ErrorResponse,
    routes::signup::{SignupRequest, SignupResponse},
};
use auth_service_macros::api_test;

#[api_test]
pub async fn should_return_201_if_valid_input() {
    let test_case = serde_json::json!({
        "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&test_case).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".into(),
    };
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[api_test]
pub async fn should_return_400_if_invalid_input() {
    let test_cases = [
        serde_json::json!(SignupRequest {
            email: "randomemail.com".into(),
            password: get_random_password(),
            requires_2fa: false
        }),
        serde_json::json!(SignupRequest {
            email: "123@com".into(),
            password: get_invalid_password(),
            requires_2fa: false
        }),
        serde_json::json!(SignupRequest {
            email: "1.23!com".into(),
            password: "".into(),
            requires_2fa: false
        }),
        serde_json::json!(SignupRequest {
            email: get_random_email(),
            password: "".into(),
            requires_2fa: false
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;

        assert_eq!(response.status().as_u16(), 400);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials"
        );
    }
}

#[api_test]
pub async fn should_return_409_if_email_already_exists() {
    let email = get_random_email();
    let password = get_random_password();

    let credentials = serde_json::json!(SignupRequest {
        email,
        password,
        requires_2fa: false
    });

    app.post_signup(&credentials).await;

    let response = app.post_signup(&credentials).await;

    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists"
    );
}
