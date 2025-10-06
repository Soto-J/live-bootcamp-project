use crate::tests::api::helpers::TestApp;

#[tokio::test]
pub async fn root_returns_logout() {
    let app = TestApp::new().await;
    let response = app.logout_root().await;

    assert_eq!(response.status().as_u16(), 200);
}
