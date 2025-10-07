use crate::helpers::TestApp;


#[tokio::test]
pub async fn root_returns_verify_token() {
    let app = TestApp::new().await;
    let response = app.verify_token_root().await;

    assert_eq!(response.status().as_u16(), 200);
}
