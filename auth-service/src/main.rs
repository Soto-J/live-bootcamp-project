use auth_service::{
    app_state::AppState,
    services::{HashmapUserStore, HashsetBannedTokenStore},
    utils::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));

    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app.");

    app.run().await.expect("Failed to run app.")
}

// Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
// This is needed for Docker to work, which we will add later on.
// See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
