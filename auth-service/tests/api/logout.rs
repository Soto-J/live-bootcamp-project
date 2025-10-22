use crate::helpers::TestApp;

use auth_service::{
    api::get_random_email,
    domain::{BannedTokenStore, Email},
    utils::{generate_auth_cookie, JWT_COOKIE_NAME},
};
use reqwest::Url;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let email = Email::parse(get_random_email()).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            cookie.value()
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let banned_token_store = app.banned_token_store.read().await;
    assert!(banned_token_store.has_token(cookie.value()))
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let email = Email::parse(get_random_email()).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            cookie.value()
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    app.post_logout().await;
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400)
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401)
}

// #[tokio::test]
// async fn should_return_422_if_malformed_credentials() {
//     let app = TestApp::new().await;

//     let email = Email::parse(get_random_email()).unwrap();
//     let cookie = generate_auth_cookie(&email).unwrap();

//     let response = app.post_logout().await;

//     assert_eq!(response.status().as_u16(), 422)
// }
