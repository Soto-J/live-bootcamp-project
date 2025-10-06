use crate::tests::api::helpers::TestApp;

#[tokio::test]
pub async fn root_returns_signup() {
    let app = TestApp::new().await;
    let response = app.signup_root().await;

    assert_eq!(response.status().as_u16(), 200);
}
