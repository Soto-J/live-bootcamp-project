use auth_service::{
    app_state::app_state::AppState,
    configure_mysql, configure_redis,
    services::{
        data_stores::{
            HashsetBannedTokenStore, MySqlUserStore, RedisBannedTokenStore, RedisTwoFACodeStore,
        },
        MockEmailClient,
    },
    utils::constants::prod,
    Application,
};

use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let mysql_pool = configure_mysql().await;
    let redis_connection = Arc::new(RwLock::new(configure_redis()));

    let user_store = Arc::new(RwLock::new(MySqlUserStore::new(mysql_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone(),
    )));
    let two_fa_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    let email_client = Arc::new(RwLock::new(MockEmailClient));

    let app_state = AppState::new(user_store, banned_token_store, two_fa_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app.");

    app.run().await.expect("Failed to run app.")
}
