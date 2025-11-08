use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    api::helpers::get_random_password,
    domain::{data_stores::LoginAttemptId, error::ErrorResponse, Email},
    routes::{
        login::{LoginRequest, LoginResponse, TwoFactorAuthResponse},
        signup::{SignupRequest, SignupResponse},
        verify_2fa::Verify2FARequest,
    },
    utils::constants::JWT_COOKIE_NAME,
};
use auth_service_macros::api_test;

#[api_test]
async fn should_return_200_if_correct_code() {
    let email = get_random_email();
    let password = get_random_email();

    let response = app
        .post_signup(&serde_json::json!(SignupRequest {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: true,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 201);
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        SignupResponse {
            message: "User created successfully!".into(),
        }
    );

    let response = app
        .post_login(&serde_json::json!(LoginRequest {
            email: email.clone(),
            password: password.clone(),
        }))
        .await;

    let (login_attempt_id, two_fa_code) = {
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse(email.clone()).unwrap())
            .await
            .expect("Failed to get 2FA code")
    };

    assert_eq!(response.status().as_u16(), 206);
    assert_eq!(
        response
            .json::<LoginResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".into(),
            login_attempt_id: login_attempt_id.clone().into(),
        })
    );

    let response = app
        .post_verify_2fa(&serde_json::json!(Verify2FARequest {
            email: email,
            login_attempt_id: login_attempt_id.into(),
            two_fa_code: two_fa_code.into(),
        }))
        .await;

    assert!(response
        .cookies()
        .any(|cookie| cookie.name() == JWT_COOKIE_NAME));

    assert_eq!(response.status().as_u16(), 200);
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    let test_cases = [serde_json::json!(Verify2FARequest {
        email: "fakeemail".into(),
        login_attempt_id: "fakeID".into(),
        two_fa_code: "faketwofa".into(),
    })];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(response.status().as_u16(), 400);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        )
    }
}

#[api_test]
async fn should_return_401_if_incorrect_credentials() {
    let email = get_random_email();
    let password = get_random_email();

    let signup_response = app
        .post_signup(&serde_json::json!(SignupRequest {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: true,
        }))
        .await;

    assert_eq!(signup_response.status().as_u16(), 201);
    assert_eq!(
        signup_response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        SignupResponse {
            message: "User created successfully!".into(),
        }
    );

    let login_response = app
        .post_login(&serde_json::json!(LoginRequest {
            email: email.clone(),
            password: password.clone(),
        }))
        .await;

    let (login_attempt_id, _) = {
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse(email.clone()).unwrap())
            .await
            .unwrap()
    };
    assert_eq!(login_response.status().as_u16(), 206);
    assert_eq!(
        login_response
            .json::<LoginResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".into(),
            login_attempt_id: login_attempt_id.into()
        })
    );

    let verify_2fa_response = app
        .post_verify_2fa(&serde_json::json!(Verify2FARequest {
            email: email,
            login_attempt_id: LoginAttemptId::default().into(),
            two_fa_code: "1234545".into(),
        }))
        .await;

    assert_eq!(verify_2fa_response.status().as_u16(), 401);
    assert_eq!(
        verify_2fa_response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Incorrect credentials".to_owned()
    );
}

#[api_test]
async fn should_return_401_if_old_code() {
    let email = get_random_email();
    let password = get_random_password();

    let signup_response = app
        .post_signup(&serde_json::json!(SignupRequest {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: true
        }))
        .await;

    assert_eq!(signup_response.status().as_u16(), 201);
    assert_eq!(
        signup_response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        SignupResponse {
            message: "User created successfully!".into()
        }
    );

    let login_response = app
        .post_login(&serde_json::json!(LoginRequest {
            email: email.clone(),
            password: password.clone(),
        }))
        .await;

    let (login_attempt_id, _) = {
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse(email.clone()).unwrap())
            .await
            .unwrap()
    };

    assert_eq!(login_response.status().as_u16(), 206);
    assert_eq!(
        login_response
            .json::<LoginResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".into(),
            login_attempt_id: login_attempt_id.into(),
        })
    );

    let (login_attempt_id, two_fa_code) = {
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse(email.clone()).unwrap())
            .await
            .expect("Failed to get 2FA")
    };

    app.post_login(&serde_json::json!(LoginRequest {
        email: email.clone(),
        password
    }))
    .await;

    let verify_2fa_response = app
        .post_verify_2fa(&serde_json::json!(Verify2FARequest {
            email,
            login_attempt_id: login_attempt_id.into(),
            two_fa_code: two_fa_code.into(),
        }))
        .await;

    assert_eq!(verify_2fa_response.status().as_u16(), 401);
    assert_eq!(
        verify_2fa_response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Incorrect credentials".to_owned()
    );
}

#[api_test]
async fn should_return_401_if_same_code_twice() {
    let email = get_random_email();
    let password = get_random_password();

    let signup_response = app
        .post_signup(&serde_json::json!(SignupRequest {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: true
        }))
        .await;

    assert_eq!(signup_response.status().as_u16(), 201);
    assert_eq!(
        signup_response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        SignupResponse {
            message: "User created successfully!".into()
        }
    );

    let login_response = app
        .post_login(&serde_json::json!(LoginRequest {
            email: email.clone(),
            password: password.clone(),
        }))
        .await;

    let (login_attempt_id, two_fa_code) = {
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse(email.clone()).unwrap())
            .await
            .unwrap()
    };

    assert_eq!(login_response.status().as_u16(), 206);
    assert_eq!(
        login_response
            .json::<LoginResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".into(),
            login_attempt_id: login_attempt_id.clone().into(),
        })
    );

    let verify_2fa_resposne = app
        .post_verify_2fa(&serde_json::json!(Verify2FARequest {
            email: email.clone(),
            login_attempt_id: login_attempt_id.clone().into(),
            two_fa_code: two_fa_code.clone().into()
        }))
        .await;

    assert_eq!(verify_2fa_resposne.status().as_u16(), 200);

    let second_verify_2fa_resposne = app
        .post_verify_2fa(&serde_json::json!(Verify2FARequest {
            email,
            login_attempt_id: login_attempt_id.into(),
            two_fa_code: two_fa_code.into()
        }))
        .await;

    assert_eq!(second_verify_2fa_resposne.status().as_u16(), 401);
}

#[api_test]
pub async fn should_return_422_if_malformed_input() {
    let body = serde_json::json!({});

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 422);
}
