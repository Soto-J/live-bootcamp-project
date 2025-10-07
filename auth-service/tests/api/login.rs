use crate::helpers::TestApp;



#[tokio::test]
pub async fn root_returns_login() {
    let app = TestApp::new().await;
    let response = app.login_root().await;

    assert_eq!(response.status().as_u16(), 200);
}
