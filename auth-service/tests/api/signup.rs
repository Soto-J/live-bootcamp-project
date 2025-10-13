use crate::helpers::{get_random_email, get_random_password, TestApp};
use auth_service::routes::SignupResponse;

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
        })).await;

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
