use crate::helpers::TestApp;


#[tokio::test]
pub async fn root_returns_verify_2fa() {
    let app = TestApp::new().await;
    let response = app.verify_2fa_root().await;

    assert_eq!(response.status().as_u16(), 200);
}
