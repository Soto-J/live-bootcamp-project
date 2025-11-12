use auth_service::{
    app_state::app_state::AppState,
    configure_mysql, configure_redis,
    domain::Email,
    services::{
        data_stores::{MySqlUserStore, RedisBannedTokenStore, RedisTwoFACodeStore},
        MockEmailClient, PostmarkEmailClient,
    },
    utils::{
        constants::{prod, POSTMARK_AUTH_TOKEN},
        tracing::init_tracing,
    },
    Application,
};
use reqwest::Client;

use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Failed to initialize tracing");

    let mysql_pool = configure_mysql().await;
    let redis_connection = Arc::new(RwLock::new(configure_redis()));

    let user_store = Arc::new(RwLock::new(MySqlUserStore::new(mysql_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone(),
    )));
    let two_fa_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    let email_client = Arc::new(RwLock::new(configure_postmark_email_client()));

    let app_state = AppState::new(user_store, banned_token_store, two_fa_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app.");

    app.run().await.expect("Failed to run app.")
}

fn configure_postmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(prod::email_client::SENDER.to_owned().into()).unwrap(),
        POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}
