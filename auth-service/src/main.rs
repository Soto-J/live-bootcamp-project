use auth_service::{
    app_state::app_state::AppState,
    configure_mysql, get_redis_client,
    services::{
        data_stores::{HashmapTwoFACodeStore, HashsetBannedTokenStore, MySqlUserStore},
        MockEmailClient,
    },
    utils::constants::{prod, REDIS_HOST_NAME},
    Application,
};

use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let mysql_pool = configure_mysql().await;

    let user_store = Arc::new(RwLock::new(MySqlUserStore::new(mysql_pool)));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient));

    let app_state = AppState::new(user_store, banned_token_store, two_fa_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app.");

    app.run().await.expect("Failed to run app.")
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Rediss client")
        .get_connection()
        .expect("Failed to get Redis connection")
}
